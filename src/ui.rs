use cursive::Cursive;
use cursive::traits::*;
use cursive::views::*;

use crate::yt_dlp::start;
use crate::{QUALITYS, Settings};

// download formats
pub fn format_selector(fmt: &str) -> String {
    let videoselect = if fmt == "Video" { "·" } else { " " };
    let audioselect = if fmt == "Audio" { "·" } else { " " };
    format!("| {} video | {} audio |", videoselect, audioselect)
}

// main download dialog
pub fn main_dialog() -> Dialog {
    let buttons = LinearLayout::horizontal()
        // pretty sure the cursive::views isnt needed but just in case
        .child(cursive::views::Button::new("Video", |s| {
            set_format(s, "Video")
        }))
        .child(DummyView.fixed_width(2))
        .child(cursive::views::Button::new("Audio", |s| {
            set_format(s, "Audio")
        }))
        .child(DummyView.fixed_width(2))
        .child(
            cursive::views::Button::new("Quality: 1080p", cycle_quality).with_name("quality_btn"),
        )
        .child(DummyView.full_width())
        .child(cursive::views::Button::new("Download", |s| {
            let url = s
                .call_on_name("url", |v: &mut EditView| v.get_content())
                .unwrap();
            start(s, &url);
        }))
        .child(DummyView.fixed_width(2))
        .child(Button::new("Quit", |s| s.quit()))
        .fixed_width(60);

    Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Paste the youtube link:"))
            .child(
                EditView::new()
                    .on_submit(start)
                    .with_name("url")
                    .fixed_width(60),
            )
            .child(TextView::new(format_selector("Video")).with_name("format"))
            .child(DummyView)
            .child(buttons),
    )
    .title("piped")
}

pub fn set_format(s: &mut Cursive, fmt: &str) {
    s.with_user_data(|settings: &mut Settings| settings.format = fmt.to_string());
    s.call_on_name("format", |t: &mut TextView| {
        t.set_content(format_selector(fmt))
    });
}

pub fn cycle_quality(s: &mut Cursive) {
    let next = s
        .with_user_data(|settings: &mut Settings| {
            let idx = QUALITYS
                .iter()
                .position(|q| *q == settings.quality)
                .unwrap_or(0);
            let next = QUALITYS[(idx + 1) % QUALITYS.len()];
            settings.quality = next.to_string();
            next.to_string()
        })
        .unwrap();
    s.call_on_name("quality_btn", |b: &mut Button| {
        b.set_label(format!("Quality: {next}"))
    });
}

pub fn bar(pct: f32) -> String {
    let n = (pct.min(100.0) / 100.0 * 30.0) as usize;
    format!("[{}{}] {:>5.1}%", "#".repeat(n), "_".repeat(30 - n), pct)
}
