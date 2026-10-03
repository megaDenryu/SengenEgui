//! 時間の行の並びの選んでいる行の描き分けを確かめる。`選んでいる行にする` を付けた行だけ、背景を行の背景色でなく
//! テーマの選択の色で描き、ほかの行は行の背景色のままである。

mod common;

use common::time_rows_point::時間の行の並びの上の点;

use common::ratio_place::指定の色で塗った矩形を集める;
use common::time_rows::{塊, 空の部品, 試験の塊, 試験の応答};
use sengen_egui::{スタイル, ノード, 時間の行, 色};

const 行の背景色: 色 = 色::from_rgb(10, 200, 30);

/// 3つの行を積み、まん中の空の行を選んでいる行にした木。
fn まん中の行を選んだ木() -> ノード<試験の応答> {
    空の部品()
        .行を置く(時間の行().塊を置く(試験の塊::甲, 塊(10.0, 20.0)))
        .行を置く(時間の行().選んでいる行にする())
        .行を置く(時間の行())
        .装飾(スタイル {
            背景色: Some(行の背景色),
            ..スタイル::無指定
        })
        .into()
}

/// 指定の縦の位置を中央に持つ、塗った矩形の色の並び。
fn 縦の中央で塗った色(出力: &egui::FullOutput, 縦: f32) -> Vec<egui::Color32> {
    出力
        .shapes
        .iter()
        .filter_map(|切り抜いた図形| match &切り抜いた図形.shape {
            egui::Shape::Rect(矩形) if (矩形.rect.center().y - 縦).abs() < 0.5 => {
                Some(矩形.fill)
            }
            _ => None,
        })
        .collect()
}

#[test]
fn 選んでいる行だけ背景を選択の色で描き_ほかの行は行の背景色のまま() {
    let eguiの本体 = egui::Context::default();
    let 点 = 時間の行の並びの上の点::読み取る(&eguiの本体, 3.0);
    let _ = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &まん中の行を選んだ木,
    );
    let (_, 出力) = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &まん中の行を選んだ木,
    );
    let 行の背景の中央の縦: Vec<f32> =
        指定の色で塗った矩形を集める(&出力, 行の背景色)
            .iter()
            .map(|矩形| 矩形.center().y)
            .collect();
    let 期待: Vec<f32> = [0.0, 2.0].iter().map(|行| 点.行の中(*行, 0.0).y).collect();
    assert_eq!(行の背景の中央の縦, 期待);
    let 選択の色 = eguiの本体
        .style()
        .visuals
        .selection
        .bg_fill
        .gamma_multiply(0.45);
    assert!(
        縦の中央で塗った色(&出力, 点.行の中(1.0, 0.0).y).contains(&選択の色),
        "選んでいる行の背景を選択の色で描く"
    );
}
