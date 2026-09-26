//! 仮想縦スクロールが、組んだ行の範囲を描画のたびに1回だけ応答として発し、スクロールすると範囲が変わり、
//! `写す` の後も発することを確かめる。

mod common;

use sengen_egui::{
    ノード, 仮想縦スクロール, 文字表示, 画素, 見えている行の範囲
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    見えた(見えている行の範囲),
}

fn 高さを限った一覧(行数: usize) -> ノード<応答> {
    仮想縦スクロール("仮想", 画素(16.0), 行数, |番号| {
        文字表示(番号.to_string()).into()
    })
    .最大高さ(画素(100.0))
    .見えている行を発する(応答::見えた)
    .into()
}

fn 見えた範囲の一覧(集まり: Vec<応答>) -> Vec<見えている行の範囲> {
    集まり.into_iter().map(|応答::見えた(範囲)| 範囲).collect()
}

#[test]
fn 描画のたびに先頭から見えている行の範囲を1回だけ発する() {
    let 文脈 = egui::Context::default();
    let 木を組む = || 高さを限った一覧(100);
    for _ in 0..2 {
        let 範囲の一覧 = 見えた範囲の一覧(common::描画する(&文脈, vec![], &木を組む));
        assert_eq!(範囲の一覧.len(), 1, "1フレームに1回だけ発する");
        let 範囲 = 範囲の一覧[0];
        assert_eq!(範囲.最初(), 0);
        assert!(範囲.最後() < 99, "高さの上限で全部の行は見えない: {範囲:?}");
        assert!(範囲.含むか(範囲.最後()) && !範囲.含むか(範囲.最後() + 1));
    }
}

#[test]
fn スクロールすると発する範囲が先へ進む() {
    let 文脈 = egui::Context::default();
    let 木を組む = || 高さを限った一覧(100);
    let 最初の範囲 = 見えた範囲の一覧(common::描画する(&文脈, vec![], &木を組む))[0];
    let 転がす = vec![
        egui::Event::PointerMoved(egui::pos2(40.0, 40.0)),
        egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -400.0),
            modifiers: egui::Modifiers::default(),
        },
    ];
    let mut 範囲 = 見えた範囲の一覧(common::描画する(&文脈, 転がす, &木を組む))[0];
    for _ in 0..60 {
        範囲 = 見えた範囲の一覧(common::描画する(&文脈, vec![], &木を組む))[0];
    }
    assert!(
        範囲.最初() > 最初の範囲.最初(),
        "転がした後の範囲 {範囲:?} は最初の範囲 {最初の範囲:?} より先にある"
    );
}

#[test]
fn 写した後も見えている行の範囲を親の応答として発する() {
    let 文脈 = egui::Context::default();
    let 木を組む = || 高さを限った一覧(100).写す(|応答::見えた(範囲)| 範囲.最初());
    assert_eq!(common::描画する(&文脈, vec![], &木を組む), vec![0]);
}

#[test]
fn 行が1つも無ければ発しない() {
    let 文脈 = egui::Context::default();
    let 木を組む = || 高さを限った一覧(0);
    assert!(common::描画する(&文脈, vec![], &木を組む).is_empty());
}
