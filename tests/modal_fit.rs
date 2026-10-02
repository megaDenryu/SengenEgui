//! モーダルの `画面に収める` と格子の `列の最大幅` を確かめる。行の多い格子と長い文を持つモーダルを小さな画面に描き、
//! 収める指定があればモーダルの枠が画面の中に収まり、指定が無ければはみ出すことと、格子の升目の文字が列の最大幅で折り返すことを確かめる。

mod common;

use common::output::入力を指定して描画する;
use sengen_egui::{ノード, モーダル, モーダル型, 子, 文字表示, 格子, 画素};

/// 確かめる画面の大きさ(幅 400・高さ 300 の論理画素)。
const 画面の大きさ: egui::Vec2 = egui::vec2(400.0, 300.0);

/// 格子の列の最大幅。
const 列の最大幅: f32 = 120.0;

const 長い文: &str = "この文は格子の升目に入れるには長すぎるため、列の最大幅を指定したときは折り返して何行にも分かれる";

fn 画面の入力() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, 画面の大きさ)),
        ..Default::default()
    }
}

/// 40行の格子と長い文を持つ、開いたモーダル。
fn 行の多いモーダル() -> モーダル型<()> {
    let 升目 = (0..40)
        .flat_map(|番号| {
            [
                文字表示(format!("{番号}行目")).into(),
                文字表示(長い文).into(),
            ]
        })
        .collect();
    モーダル(
        "行の多いモーダル",
        true,
        子![
            文字表示(長い文.repeat(3)),
            格子("行の多い格子", 2, 升目).列の最大幅(画素(列の最大幅)),
        ],
    )
}

/// 木を数回描き(最初の回は egui が大きさだけを測る)、モーダルの枠の範囲と最後の回の出力を返す。
fn 描いた枠の範囲(
    木を組む: &dyn Fn() -> ノード<()>
) -> (egui::Rect, egui::FullOutput) {
    let eguiの本体 = egui::Context::default();
    let mut 最後の出力 = None;
    for _ in 0..4 {
        最後の出力 = Some(入力を指定して描画する(&eguiの本体, 画面の入力(), 木を組む).1);
    }
    let 範囲 = eguiの本体
        .memory(|記憶| 記憶.area_rect(egui::Id::new("行の多いモーダル")))
        .unwrap_or_else(|| panic!("モーダルの枠の範囲を記憶していない"));
    let 出力 = 最後の出力.unwrap_or_else(|| panic!("一度も描いていない"));
    (範囲, 出力)
}

#[test]
fn 画面に収める指定があればモーダルの枠は画面の中に収まる() {
    let (範囲, _) = 描いた枠の範囲(&|| 行の多いモーダル().画面に収める(画素(16.0)).into());
    let 画面 = egui::Rect::from_min_size(egui::Pos2::ZERO, 画面の大きさ);
    assert!(
        画面.contains_rect(範囲),
        "枠 {範囲:?} が画面 {画面:?} からはみ出す"
    );
}

#[test]
fn 画面に収める指定が無ければ行の多いモーダルは画面の高さを超える() {
    let (範囲, _) = 描いた枠の範囲(&|| 行の多いモーダル().into());
    assert!(範囲.height() > 画面の大きさ.y, "{範囲:?}");
}

#[test]
fn 格子の升目の文字は列の最大幅で折り返す() {
    let (_, 出力) = 描いた枠の範囲(&|| 行の多いモーダル().画面に収める(画素(16.0)).into());
    let 升目の文字の幅: Vec<f32> = 出力
        .shapes
        .iter()
        .filter_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Text(文字の形) if 文字の形.galley.text() == 長い文 => {
                Some(文字の形.galley.rect.width())
            }
            _ => None,
        })
        .collect();
    assert!(!升目の文字の幅.is_empty(), "升目の長い文を描いていない");
    assert!(
        升目の文字の幅.iter().all(|幅| *幅 <= 列の最大幅 + 0.5),
        "{升目の文字の幅:?}"
    );
}
