use cursive::Cursive;
use cursive::style::{BorderStyle, Palette};
use cursive::traits::*;
use cursive::views::*;

use crate::settings::settings_dialog;
use crate::yt_dlp::{Settings, run};

pub fn theme() -> cursive::theme::Theme {
    cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::style::BaseColor::*;
            {
                use cursive::style::Color::TerminalDefault;
                use cursive::style::PaletteColor::*;
                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }
            {
                use cursive::style::Effect::*;
                use cursive::style::PaletteStyle::*;
                use cursive::style::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
                palette[EditableTextCursor] = Style::secondary().combine(Reverse).combine(Underline)
            }
        }),
    }
}

fn format_selector(fmt: &str) -> String {
    let videoselect = if fmt == "Video" { "·" } else { " " };
    let audioselect = if fmt == "Audio" { "·" } else { " " };
    format!("| {} video | {} audio |", videoselect, audioselect)
}

pub fn main_dialog() -> Dialog {
    let buttons = LinearLayout::horizontal()
        .child(Button::new("Video", |s| set_format(s, "Video")))
        .child(DummyView.fixed_width(2))
        .child(Button::new("Audio", |s| set_format(s, "Audio")))
        .child(DummyView.fixed_width(2))
        .child(Button::new("Settings", |s| {
            s.add_layer(settings_dialog());
        }))
        .child(DummyView.full_width())
        .child(Button::new("Download", |s| {
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
    .title("tubert")
}

fn set_format(s: &mut Cursive, fmt: &str) {
    s.with_user_data(|settings: &mut Settings| settings.format = fmt.to_string());
    s.call_on_name("format", |t: &mut TextView| {
        t.set_content(format_selector(fmt))
    });
}

pub fn bar(pct: f32) -> String {
    let n = (pct.min(100.0) / 100.0 * 30.0) as usize;
    format!("[{}{}] {:>5.1}%", "#".repeat(n), "_".repeat(30 - n), pct)
}

fn start(s: &mut Cursive, url: &str) {
    let url = url.trim().to_string();
    if url.is_empty() {
        return s.add_layer(Dialog::info("Link cannot be empty"));
    }
    let settings = s.user_data::<Settings>().cloned().unwrap_or(Settings {
        format: "Video".into(),
        quality: "1080p".into(),
        directory: "./downloads".into(),
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
