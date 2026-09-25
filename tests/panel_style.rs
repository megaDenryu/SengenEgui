//! パネルの装飾が、縁のパネルと中央のパネルのどちらでも背景色で地を塗り、内余白で中身をずらし、
//! 装飾の無いパネルはテーマの地の色と既定の余白のまま描くことを確かめる。

mod common;

use common::output::入力を指定して描画する;
use sengen_egui::{
    スタイル, ノード, パネル, パネルの位置, 余白画素, 子, 文字表示, 縦積み
};

const 赤い地: スタイル = スタイル {
    背景色: Some(egui::Color32::RED),
    内余白: Some(余白画素(24)),
    ..スタイル::無指定
};

fn 色で塗った矩形があるか(図形: &egui::Shape, 色: egui::Color32) -> bool {
    match 図形 {
        egui::Shape::Rect(矩形) => 矩形.fill == 色,
        egui::Shape::Vec(図形一覧) => {
            図形一覧.iter().any(|中| 色で塗った矩形があるか(中, 色))
        }
        _ => false,
    }
}

fn 文字の左端(図形: &egui::Shape, 文字列: &str) -> Option<f32> {
    match 図形 {
        egui::Shape::Text(文字) if 文字.galley.text() == 文字列 => Some(文字.pos.x),
        egui::Shape::Vec(図形一覧) => {
            図形一覧.iter().find_map(|中| 文字の左端(中, 文字列))
        }
        _ => None,
    }
}

/// 装飾を付けるかを選んだパネルを描き、赤く塗ったかと、中の文字の左端を返す。
fn 描いて調べる(位置: パネルの位置, 装飾する: bool) -> (bool, Option<f32>) {
    let eguiの本体 = egui::Context::default();
    let 木を組む = || -> ノード<()> {
        let 調べる = パネル("調べる", 位置, 子![文字表示("中身")]);
        let 調べる = if 装飾する {
            調べる.装飾(赤い地)
        } else {
            調べる
        };
        縦積み(子![調べる]).into()
    };
    let (_, 出力) =
        入力を指定して描画する(&eguiの本体, egui::RawInput::default(), &木を組む);
    let 図形一覧 = 出力
        .shapes
        .iter()
        .map(|切り抜いた図形| &切り抜いた図形.shape);
    let 赤い = 図形一覧
        .clone()
        .any(|図形| 色で塗った矩形があるか(図形, egui::Color32::RED));
    (
        赤い,
        図形一覧
            .into_iter()
            .find_map(|図形| 文字の左端(図形, "中身")),
    )
}

fn 装飾で地を塗り内余白で中身をずらすことを確かめる(
    位置: パネルの位置,
    既定の枠: egui::Frame,
) {
    let (装飾ありの赤, 装飾ありの左端) = 描いて調べる(位置, true);
    let (装飾なしの赤, 装飾なしの左端) = 描いて調べる(位置, false);
    assert!(装飾ありの赤 && !装飾なしの赤);
    let ずれ = 24.0 - 既定の枠.inner_margin.leftf();
    let 実際のずれ = 装飾ありの左端
        .zip(装飾なしの左端)
        .map(|(あり, なし)| あり - なし);
    assert_eq!(実際のずれ, Some(ずれ));
}

#[test]
fn 上のパネルの装飾は地を塗り内余白で中身をずらす() {
    let 既定の枠 = egui::Frame::side_top_panel(&egui::Style::default());
    装飾で地を塗り内余白で中身をずらすことを確かめる(
        パネルの位置::上,
        既定の枠,
    );
}

#[test]
fn 中央のパネルの装飾は地を塗り内余白で中身をずらす() {
    let 既定の枠 = egui::Frame::central_panel(&egui::Style::default());
    装飾で地を塗り内余白で中身をずらすことを確かめる(
        パネルの位置::中央,
        既定の枠,
    );
}
