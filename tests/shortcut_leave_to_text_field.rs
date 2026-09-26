//! 入力欄にフォーカスがある間は発しないキー操作を確かめる。修飾キーを含む組(Ctrl+Z)でも、入力欄の編集の間は
//! 入力欄に任せて発行せず、フォーカスが無ければその回に発行する。

mod common;

use sengen_egui::{
    キーの組, キー操作, ノード, 一行テキスト入力, 子, 縦積み
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    文字を変えた(String),
    取り消した,
}

const 入力欄の位置: egui::Pos2 = egui::pos2(30.0, 18.0);

fn 入力欄と取り消しの組() -> ノード<応答> {
    縦積み(子![
        一行テキスト入力("値", 応答::文字を変えた).確定時のみ発行("確定"),
        キー操作(
            キーの組::生成する(egui::Modifiers::COMMAND, egui::Key::Z),
            応答::取り消した
        )
        .入力欄にフォーカスがある間は発しない(),
    ])
    .into()
}

#[test]
fn 入力欄にフォーカスがある間は修飾キーの組でも発行せずフォーカスも奪わない() {
    let eguiの本体 = egui::Context::default();
    assert!(common::左クリックする(&eguiの本体, 入力欄の位置, &入力欄と取り消しの組).is_empty());
    let 押下 = common::描画する(
        &eguiの本体,
        vec![common::キー押下(egui::Key::Z, egui::Modifiers::COMMAND)],
        &入力欄と取り消しの組,
    );
    let 次の回 = common::描画する(&eguiの本体, vec![], &入力欄と取り消しの組);
    assert!(押下.is_empty() && 次の回.is_empty(), "{押下:?} {次の回:?}");
    assert!(
        eguiの本体.memory(|記憶| 記憶.focused()).is_some(),
        "入力欄のフォーカスは残る"
    );
}

#[test]
fn フォーカスが無ければ修飾キーの組はその回に発行される() {
    let eguiの本体 = egui::Context::default();
    assert!(common::描画する(&eguiの本体, vec![], &入力欄と取り消しの組).is_empty());
    let 押下 = common::描画する(
        &eguiの本体,
        vec![common::キー押下(egui::Key::Z, egui::Modifiers::COMMAND)],
        &入力欄と取り消しの組,
    );
    assert_eq!(押下, vec![応答::取り消した]);
}
