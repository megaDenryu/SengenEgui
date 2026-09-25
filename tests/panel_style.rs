//! パネルの装飾が、背景色でパネルの地を塗り、装飾の無いパネルはテーマの地の色のまま描くことを確かめる。

mod common;

use common::output::入力を指定して描画する;
use sengen_egui::{
    スタイル, ノード, パネル, パネルの位置, 余白画素, 子, 文字表示, 縦積み
};

const 赤い地: スタイル = スタイル {
    背景色: Some(egui::Color32::RED),
    内余白: Some(余白画素(12)),
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

fn 赤く塗ったか(装飾する: bool) -> bool {
    let eguiの本体 = egui::Context::default();
    let 木を組む = || -> ノード<()> {
        let 上 = パネル("上", パネルの位置::上, 子![文字表示("上")]);
        let 上 = if 装飾する {
            上.装飾(赤い地)
        } else {
            上
        };
        縦積み(子![
            上,
            パネル("中央", パネルの位置::中央, 子![文字表示("中央")])
        ])
        .into()
    };
    let (_, 出力) =
        入力を指定して描画する(&eguiの本体, egui::RawInput::default(), &木を組む);
    出力.shapes.iter().any(|切り抜いた図形| {
        色で塗った矩形があるか(&切り抜いた図形.shape, egui::Color32::RED)
    })
}

#[test]
fn 背景色を指定したパネルはその色で地を塗る() {
    assert!(赤く塗ったか(true));
}

#[test]
fn 装飾の無いパネルは指定しない色で塗らない() {
    assert!(!赤く塗ったか(false));
}
