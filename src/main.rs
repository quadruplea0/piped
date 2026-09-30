mod settings;
mod ui;
mod yt_dlp;

use yt_dlp::Settings;

fn main() {
    let mut siv = cursive::default();
    siv.set_theme(ui::theme());
    siv.set_user_data(Settings {
        format: "Video".into(),
        quality: "1080p".into(),
        directory: "./downloads".into(),
    });
    siv.add_layer(ui::main_dialog());
    siv.run();
}
