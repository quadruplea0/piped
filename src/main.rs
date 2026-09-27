use cursive::With;
use cursive::style::{BorderStyle, Palette};

mod ui;
mod yt_dlp;

use ui::main_dialog;

// possible qualities to download the video in
pub const QUALITYS: [&str; 3] = ["1080p", "720p", "480p"];

#[derive(Clone)]
pub struct Settings {
    pub format: String,
    pub quality: String,
}

fn main() {
    let mut siv = cursive::default();
    // theme that actually looks good, taken from https://github.com/gyscos/cursive/blob/main/cursive/examples/theme_manual.rs
    siv.set_theme(cursive::theme::Theme {
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
    });

    siv.set_user_data(Settings {
        format: "Video".into(),
        quality: "1080p".into(),
    });
    siv.add_layer(main_dialog());
    siv.run();
}
