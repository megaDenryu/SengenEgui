//! 付け足し（説明・右クリックメニュー・ドラッグ元・落とし先）と見えない部品（キー操作）を確かめる。
//! 右クリックメニューを付けたボタンを左クリックしたとき、ボタンの応答が横取りされないことが要点である。

mod common;

use sengen_egui::{
    キーの組, キー操作, ドラッグ元, ノード, ボタン, 付け足せる, 子, 文字表示, 縦積み, 落とし先,
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    押した,
    消した,
    受け取った(u32),
    保存した,
}

fn 右クリックメニュー付きのボタン() -> ノード<応答> {
    縦積み(子![ボタン("押す", 応答::押した).右クリックメニュー(
        子![ボタン("消す", 応答::消した)]
    )])
    .into()
}

#[test]
fn 右クリックメニューを付けたボタンを左クリックするとボタンの応答が出る() {
    let 文脈 = egui::Context::default();
    let 集まり = common::左クリックする(
        &文脈,
        egui::pos2(30.0, 18.0),
        &右クリックメニュー付きのボタン,
    );
    assert_eq!(集まり, vec![応答::押した]);
}

#[test]
fn 右クリックで開いたメニューの項目を押すと項目の応答が出る() {
    let 文脈 = egui::Context::default();
    let 位置 = egui::pos2(30.0, 18.0);
    let 開いた = common::クリックする(
        &文脈,
        位置,
        egui::PointerButton::Secondary,
        &右クリックメニュー付きのボタン,
    );
    assert!(開いた.is_empty(), "{開いた:?}");
    let 選んだ = common::左クリックする(
        &文脈,
        egui::pos2(位置.x + 12.0, 位置.y + 14.0),
        &右クリックメニュー付きのボタン,
    );
    assert_eq!(選んだ, vec![応答::消した]);
}

#[test]
fn 説明とドラッグ元と落とし先は描画できて操作が無ければ応答は空になる() {
    let 文脈 = egui::Context::default();
    let 木を組む = || {
        縦積み(子![
            文字表示("見出し").説明("説明の文"),
            ドラッグ元("元", 7u32, 文字表示("運ぶ")),
            落とし先(文字表示("ここへ"), 応答::受け取った),
        ])
        .into()
    };
    assert!(common::描画する(&文脈, vec![], &木を組む).is_empty());
    let 木を組む = || 木を組む().写す(|応答| format!("{応答:?}"));
    assert!(common::描画する(&文脈, vec![], &木を組む).is_empty());
}

#[test]
fn キーの組が押されたフレームだけキー操作の応答が出る() {
    let 文脈 = egui::Context::default();
    let 木を組む = || {
        縦積み(子![キー操作(
            キーの組::生成する(egui::Modifiers::COMMAND, egui::Key::S),
            応答::保存した
        )])
        .into()
    };
    assert!(common::描画する(&文脈, vec![], &木を組む).is_empty());
    let 押下 = egui::Event::Key {
        key: egui::Key::S,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    };
    assert_eq!(
        common::描画する(&文脈, vec![押下], &木を組む),
        vec![応答::保存した]
    );
}
