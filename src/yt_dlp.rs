use cursive::traits::*;
use cursive::views::*;
use cursive::{CbSink, Cursive};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::Settings;
use crate::ui::{bar, main_dialog};

// start downloading
pub fn start(s: &mut Cursive, url: &str) {
    let url = url.trim().to_string();
    if url.is_empty() {
        return s.add_layer(Dialog::info("Link cannot be empty"));
    }
    let settings = s.user_data::<Settings>().cloned().unwrap_or(Settings {
        format: "Video".into(),
        quality: "1080p".into(),
    });
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(bar(0.0)).with_name("bar"))
                .child(TextView::new("Starting...").with_name("status"))
                .fixed_width(60),
        )
        .title("Downloading")
        .with_name("popup"),
    );
    let sink = s.cb_sink().clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(run(url, settings, sink))
    });
}

async fn run(url: String, settings: Settings, sink: CbSink) {
    let set = |pct: f32, msg: String, done: bool| {
        let _ = sink.send(Box::new(move |s: &mut Cursive| {
            s.call_on_name("bar", |t: &mut TextView| t.set_content(bar(pct)));
            s.call_on_name("status", |t: &mut TextView| t.set_content(msg));
            if done {
                s.call_on_name("popup", |d: &mut Dialog| {
                    d.add_button("Close", |s| {
                        s.pop_layer();
                        s.add_layer(main_dialog());
                    });
                    d.add_button("Quit", |s| s.quit());
                });
            }
        }));
    };

    let output_dir = PathBuf::from("./downloads");
    if let Err(e) = tokio::fs::create_dir_all(&output_dir).await {
        return set(0.0, format!("Error: {e}"), true);
    }

    // build the yt-dlp args ourselves, since we spawn the process directly
    // (this mirrors what ytd_rs::YtDlp::extract_audio_only()/arg() used to build)
    let mut args: Vec<String> = vec!["--newline".into()];
    if settings.format == "Audio" {
        args.push("--format".into());
        args.push("bestaudio/best".into());
        args.push("--extract-audio".into());
        args.push("--audio-format".into());
        args.push("mp3".into());
    } else if settings.quality != "" {
        let height = settings.quality.trim_end_matches('p');
        args.push("-f".into());
        args.push(format!(
            "bestvideo[height<={height}]+bestaudio/best[height<={height}]"
        ));
        args.push("-t".into());
        args.push(format!("mp4"));
    }
    args.push(url);

    let mut child = match Command::new("yt-dlp")
        .current_dir(&output_dir)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return set(0.0, format!("Error: {e}"), true),
    };

    let stdout = child.stdout.take().expect("stdout was piped");
    let stderr = child.stderr.take().expect("stderr was piped");

    // collect stderr cause ytdlp was being stupid
    let stderr_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        let mut collected = String::new();
        while let Ok(Some(line)) = lines.next_line().await {
            collected.push_str(&line);
            collected.push('\n');
        }
        collected
    });

    let mut stdout_lines = BufReader::new(stdout).lines();
    while let Ok(Some(line)) = stdout_lines.next_line().await {
        if let Some(pct) = line
            .split_whitespace()
            .find_map(|w| w.strip_suffix('%'))
            .and_then(|p| p.parse::<f32>().ok())
        {
            set(pct, line.clone(), false);
        }
    }

    let status = child.wait().await;
    let stderr_output = stderr_task.await.unwrap_or_default();

    match status {
        Ok(s) if s.success() => set(100.0, "Download complete.".into(), true),
        Ok(s) => {
            let msg = if stderr_output.trim().is_empty() {
                format!("yt-dlp exited with status {:?}", s.code())
            } else {
                stderr_output.trim().to_string()
            };
            set(0.0, format!("Error: {msg}"), true)
        }
        Err(e) => set(0.0, format!("Error: {e}"), true),
    }
}
