use cursive::Cursive;
use cursive::traits::*;
use cursive::views::*;

use crate::yt_dlp::Settings;

const QUALITYS: [&str; 3] = ["1080p", "720p", "480p"];

pub fn settings_dialog() -> Dialog {
    let (quality, directory) = current_values();

    Dialog::around(
        LinearLayout::vertical()
            .child(
                Button::new(format!("Quality: {quality}"), cycle_quality).with_name("quality_btn"),
            )
            .child(DummyView)
            .child(Button::new(format!("Dir: {directory}"), edit_directory).with_name("dir_btn")),
    )
    .title("Settings")
    .button("Close", |s| {
        s.pop_layer();
    })
}

fn current_values() -> (String, String) {
    ("1080p".to_string(), "./downloads".to_string())
}

fn cycle_quality(s: &mut Cursive) {
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

fn edit_directory(s: &mut Cursive) {
    let current = s
        .user_data::<Settings>()
        .map(|st| st.directory.clone())
        .unwrap_or_else(|| "./downloads".to_string());

    s.add_layer(
        Dialog::around(
            EditView::new()
                .content(current)
                .on_submit(save_directory)
                .with_name("dir_input")
                .fixed_width(50),
        )
        .title("Download directory")
        .button("Save", |s| {
            let path = s
                .call_on_name("dir_input", |v: &mut EditView| v.get_content())
                .unwrap();
            save_directory(s, &path);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn save_directory(s: &mut Cursive, path: &str) {
    let path = path.trim().to_string();
    if path.is_empty() {
        return;
    }
    s.with_user_data(|settings: &mut Settings| settings.directory = path.clone());
    s.call_on_name("dir_btn", |b: &mut Button| {
        b.set_label(format!("Dir: {path}"))
    });
    s.pop_layer();
}
