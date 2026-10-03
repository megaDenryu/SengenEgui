//! 時間の行の並びの描画を確かめる。塊の無い行も含めて行ごとに背景を描き、行は置いた順に下から上へ積む。
//! 塊に付けた等しい間隔の区切りには縦の線を描く。

mod common;

use common::time_rows_point::時間の行の並びの上の点;

use common::ratio_place::指定の色で塗った矩形を集める;
use common::time_rows::{塊, 空の部品};
use sengen_egui::{スタイル, ノード, 時間の行, 色};

const 行の背景色: 色 = 色::from_rgb(10, 200, 30);

/// 下の行に塊を1つ置き、上の2つの行を空にした木。
fn 空の行を含む木() -> ノード<common::time_rows::試験の応答> {
    空の部品()
        .行を置く(時間の行().塊を置く(common::time_rows::試験の塊::甲, 塊(10.0, 20.0)))
        .行を置く(時間の行())
        .行を置く(時間の行())
        .装飾(スタイル {
            背景色: Some(行の背景色),
            ..スタイル::無指定
        })
        .into()
}

#[test]
fn 塊の無い行も含めて行ごとに背景を下から上へ積んで描く() {
    let eguiの本体 = egui::Context::default();
    let 点 = 時間の行の並びの上の点::読み取る(&eguiの本体, 3.0);
    let _ = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &空の行を含む木,
    );
    let (_, 出力) = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &空の行を含む木,
    );
    let 背景の中央の縦: Vec<f32> = 指定の色で塗った矩形を集める(&出力, 行の背景色)
        .iter()
        .map(|矩形| 矩形.center().y)
        .collect();
    let 期待: Vec<f32> = [0.0, 1.0, 2.0]
        .iter()
        .map(|行| 点.行の中(*行, 0.0).y)
        .collect();
    assert_eq!(背景の中央の縦, 期待);
}

fn 縦の線の横の位置を集める(図形: &egui::Shape, 集まり: &mut Vec<f32>) {
    match 図形 {
        egui::Shape::LineSegment { points, .. } if points[0].x == points[1].x => {
            集まり.push(points[0].x)
        }
        egui::Shape::Vec(図形一覧) => {
            for 中 in 図形一覧 {
                縦の線の横の位置を集める(中, 集まり);
            }
        }
        _ => {}
    }
}

#[test]
fn 等しい間隔の区切りを付けた塊は区切りの値に縦の線を描く() {
    let eguiの本体 = egui::Context::default();
    let 点 = 時間の行の並びの上の点::読み取る(&eguiの本体, 1.0);
    let 四つに区切った木 = || -> ノード<common::time_rows::試験の応答> {
        let Some(四) = std::num::NonZeroU32::new(4) else {
            panic!("4 は0でない");
        };
        let 区切った塊 = 塊(10.0, 20.0).等しい間隔の区切りを付ける(四);
        空の部品()
            .行を置く(時間の行().塊を置く(common::time_rows::試験の塊::甲, 区切った塊))
            .into()
    };
    let _ = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &四つに区切った木,
    );
    let (_, 出力) = common::output::入力を指定して描画する(
        &eguiの本体,
        egui::RawInput::default(),
        &四つに区切った木,
    );
    let mut 横の位置一覧 = Vec::new();
    for 切り抜いた図形 in &出力.shapes {
        縦の線の横の位置を集める(&切り抜いた図形.shape, &mut 横の位置一覧);
    }
    let 期待: Vec<f32> = [15.0, 20.0, 25.0]
        .iter()
        .map(|値| 点.行の中(0.0, *値).x)
        .collect();
    assert_eq!(横の位置一覧, 期待);
}
