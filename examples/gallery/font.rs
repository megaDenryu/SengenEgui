//! 日本語フォントの導入。egui の既定フォントは日本語の字形を含まないため、OS のフォントを
//! 読み込んで既定の族の後ろへ足す。見つからなければ既定のまま続ける（起動は止めない）。

use std::sync::Arc;

use eframe::egui;

const 候補一覧: &[&str] = &[
    "C:/Windows/Fonts/meiryo.ttc",
    "C:/Windows/Fonts/YuGothM.ttc",
    "C:/Windows/Fonts/msgothic.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
];

pub fn 日本語フォントを設定する(文脈: &egui::Context) {
    let Some(バイト列) = 候補一覧.iter().find_map(|パス| std::fs::read(パス).ok())
    else {
        eprintln!("日本語フォントが見つからない。表示が崩れる場合は OS へ日本語フォントを導入する");
        return;
    };
    let mut 定義 = egui::FontDefinitions::default();
    定義.font_data.insert(
        "日本語".to_owned(),
        Arc::new(egui::FontData::from_owned(バイト列)),
    );
    for 族 in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(一覧) = 定義.families.get_mut(&族) {
            一覧.push("日本語".to_owned());
        }
    }
    文脈.set_fonts(定義);
}
