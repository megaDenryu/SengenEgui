//! ドラッグ元から落とし先へ値が運ばれて応答が出ることと、容器を包んだ右クリックメニューを確かめる。

mod common;

use sengen_egui::{
    ドラッグ元, ノード, ボタン, 付け足せる, 子, 文字表示, 縦積み, 落とし先
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    押した,
    消した,
    受け取った(u32),
}

fn 運ぶ木() -> ノード<応答> {
    縦積み(子![
        ドラッグ元("元", 7u32, ボタン("運ぶ", 応答::押した)),
        落とし先(文字表示("ここへ落とす"), 応答::受け取った),
    ])
    .into()
}

#[test]
fn ドラッグ元から落とし先へ運んで放すと値の応答が出る() {
    let 文脈 = egui::Context::default();
    let 元 = egui::pos2(30.0, 18.0);
    let 先 = egui::pos2(30.0, 48.0);
    let mut 集まり = common::描画する(&文脈, vec![], &運ぶ木);
    集まり.extend(common::描画する(
        &文脈,
        vec![
            egui::Event::PointerMoved(元),
            common::ボタンの出来事(元, egui::PointerButton::Primary, true),
        ],
        &運ぶ木,
    ));
    for 位置 in [egui::pos2(30.0, 30.0), 先, 先] {
        集まり.extend(common::描画する(
            &文脈,
            vec![egui::Event::PointerMoved(位置)],
            &運ぶ木,
        ));
    }
    集まり.extend(common::描画する(
        &文脈,
        vec![common::ボタンの出来事(
            先,
            egui::PointerButton::Primary,
            false,
        )],
        &運ぶ木,
    ));
    assert_eq!(集まり, vec![応答::受け取った(7)]);
}

fn 右クリックメニュー付きの容器() -> ノード<応答> {
    縦積み(子![ボタン("押す", 応答::押した), 文字表示("文字")])
        .右クリックメニュー(子![ボタン("消す", 応答::消した)])
}

#[test]
fn 容器を包んだ右クリックメニューでも中のボタンの左クリックが通る() {
    let 文脈 = egui::Context::default();
    let 集まり =
        common::左クリックする(&文脈, egui::pos2(30.0, 18.0), &右クリックメニュー付きの容器);
    assert_eq!(集まり, vec![応答::押した]);
}

#[test]
fn 容器の文字の上で右クリックするとメニューが出て項目を押せる() {
    let 文脈 = egui::Context::default();
    let 位置 = egui::pos2(20.0, 40.0);
    let 開いた = common::クリックする(
        &文脈,
        位置,
        egui::PointerButton::Secondary,
        &右クリックメニュー付きの容器,
    );
    assert!(開いた.is_empty(), "{開いた:?}");
    let 選んだ = common::左クリックする(
        &文脈,
        egui::pos2(位置.x + 12.0, 位置.y + 14.0),
        &右クリックメニュー付きの容器,
    );
    assert_eq!(選んだ, vec![応答::消した]);
}
