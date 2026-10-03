//! 複数の範囲枠が、隅のつまみのために確保した余白の内側の基準の矩形へ、下地を描くことを確かめる。

mod common;

use common::ratio_place::{
    塗りつぶす子を作る, 指定の色で塗った矩形を集める
};
use sengen_egui::{
    ノード, 割合で表した矩形, 画素の組, 色, 複数の範囲枠, 複数の範囲枠の操作
};

const 下地の色: 色 = 色::from_rgb(10, 200, 30);

fn 下地を塗った木() -> ノード<複数の範囲枠の操作> {
    複数の範囲枠(画素の組(200.0, 100.0), 塗りつぶす子を作る(下地の色))
        .枠を置く(割合で表した矩形::全体, |操作| 操作)
        .into()
}

#[test]
fn 下地は余白の内側の基準の矩形いっぱいに描かれる() {
    let eguiの本体 = egui::Context::default();
    let _ = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &下地を塗った木,
    );
    let (_, 出力) = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &下地を塗った木,
    );
    let 塗った矩形一覧 = 指定の色で塗った矩形を集める(&出力, 下地の色);
    let 中央パネルの内余白と部品の余白 = egui::pos2(16.0, 16.0);
    let 期待 = egui::Rect::from_min_size(中央パネルの内余白と部品の余白, egui::vec2(200.0, 100.0));
    assert_eq!(塗った矩形一覧, vec![期待]);
}
