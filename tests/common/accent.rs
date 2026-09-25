//! 強調色まわりの色の試験の共通部。琥珀色の強調色を持つ濃色のテーマと、描いた文字の書式の色の読み取り。

use egui::Color32;
use sengen_egui::{テーマ, ノード, 明暗};

use super::glyph::文字の図形一覧;

pub const 琥珀: Color32 = Color32::from_rgb(217, 119, 6);
pub const 地の色: Color32 = Color32::from_rgb(15, 23, 42);
pub const 文字色: Color32 = Color32::from_rgb(226, 232, 240);
pub const 指定の文字色: Color32 = Color32::from_rgb(2, 6, 23);

pub fn 琥珀のテーマ(強調色の上の文字色: Option<Color32>) -> テーマ {
    テーマ {
        基調: 明暗::濃色,
        強調色: Some(琥珀),
        地の色: Some(地の色),
        文字色: Some(文字色),
        部品の面の色: None,
        強調色の上の文字色,
        表示倍率: None,
        部品の間隔: None,
        ボタンの内余白: None,
        部品の角丸: None,
    }
}

/// 2フレーム描き、指定の文字列を描いた文字の図形の、文字の書式の色を描いた順に返す。色を付けていない文字は PLACEHOLDER になる。
pub fn 文字の色一覧(
    eguiの本体: &egui::Context,
    文字列: &str,
    木を組む: &dyn Fn() -> ノード<()>,
) -> Vec<Color32> {
    let _ = super::output::入力を指定して描画する(
        eguiの本体,
        egui::RawInput::default(),
        木を組む,
    );
    let (_, 出力) = super::output::入力を指定して描画する(
        eguiの本体,
        egui::RawInput::default(),
        木を組む,
    );
    文字の図形一覧(&出力)
        .into_iter()
        .filter(|図形| 図形.galley.text() == 文字列)
        .filter_map(|図形| {
            図形
                .galley
                .job
                .sections
                .first()
                .map(|区切り| 区切り.format.color)
        })
        .collect()
}

pub fn 文字の色(
    eguiの本体: &egui::Context,
    文字列: &str,
    木を組む: &dyn Fn() -> ノード<()>,
) -> Option<Color32> {
    文字の色一覧(eguiの本体, 文字列, 木を組む).first().copied()
}
