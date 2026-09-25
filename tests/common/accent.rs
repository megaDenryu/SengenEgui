//! 選ばれた項目と範囲選択の色の試験の共通部。琥珀色の強調色を持つ濃色のテーマと、描いた文字の書式の色の読み取り。

use egui::Color32;
use sengen_egui::{テーマ, ノード, 明暗};

use super::glyph::文字の図形一覧;

pub const 琥珀: Color32 = Color32::from_rgb(217, 119, 6);
pub const 地の色: Color32 = Color32::from_rgb(15, 23, 42);
pub const 文字色: Color32 = Color32::from_rgb(226, 232, 240);
pub const 指定の文字色: Color32 = Color32::from_rgb(2, 6, 23);

pub fn 琥珀のテーマ(強調色の上の文字色: Option<Color32>) -> テーマ {
    テーマ {
        強調色: Some(琥珀),
        地の色: Some(地の色),
        文字色: Some(文字色),
        強調色の上の文字色,
        ..テーマ::基調から作る(明暗::濃色)
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

/// 描いた文字の図形のうち、指定の文字列の行に塗られた、PLACEHOLDER でも文字色でもない頂点の色（範囲選択の地の色）。
pub fn 範囲選択の地の色(出力: &egui::FullOutput, 文字列: &str) -> Option<Color32> {
    文字の図形一覧(出力)
        .into_iter()
        .filter(|図形| 図形.galley.text() == 文字列)
        .flat_map(|図形| {
            let 行一覧 = 図形.galley.rows.clone();
            行一覧
                .into_iter()
                .flat_map(|行| 行.row.visuals.mesh.vertices.clone())
        })
        .map(|頂点| 頂点.color)
        .find(|色| *色 != Color32::PLACEHOLDER && *色 != 文字色)
}

/// WCAG 2 のコントラスト比。試験の側で独立に計算し、部品の計算と突き合わせる。
pub fn コントラスト比(甲: Color32, 乙: Color32) -> f32 {
    let 相対輝度 = |色: Color32| {
        let 線形 = |成分: u8| {
            let 値 = f32::from(成分) / 255.0;
            if 値 <= 0.04045 {
                値 / 12.92
            } else {
                ((値 + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * 線形(色.r()) + 0.7152 * 線形(色.g()) + 0.0722 * 線形(色.b())
    };
    let (明るい方, 暗い方) = (
        相対輝度(甲).max(相対輝度(乙)),
        相対輝度(甲).min(相対輝度(乙)),
    );
    (明るい方 + 0.05) / (暗い方 + 0.05)
}
