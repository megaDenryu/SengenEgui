//! キー操作が文字を打つ部品を見分ける規則を確かめる。描かれなくなった入力欄と識別子が重なるボタンを入力欄とみなさないことと、
//! 入力欄の編集を Escape でやめた回の Escape をキー操作が消費しないこと。

mod common;

use sengen_egui::{
    キーの組, キー操作, ノード, ボタン, 一行テキスト入力, 子, 縦積み
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    文字を変えた(String),
    空白を押した,
    ボタンを押した,
    エスケープを押した,
}

const 入力欄の位置: egui::Pos2 = egui::pos2(30.0, 18.0);
const 無し: egui::Modifiers = egui::Modifiers::NONE;

fn 空白キーと入力欄() -> ノード<応答> {
    縦積み(子![
        キー操作(キーの組::単独(egui::Key::Space), 応答::空白を押した),
        一行テキスト入力("値", 応答::文字を変えた),
    ])
    .into()
}

fn 空白キーとボタン() -> ノード<応答> {
    縦積み(子![
        キー操作(キーの組::単独(egui::Key::Space), 応答::空白を押した),
        ボタン("押す", 応答::ボタンを押した),
    ])
    .into()
}

#[test]
fn 描かれなくなった入力欄と識別子が重なるボタンを入力欄とみなさない() {
    let eguiの本体 = egui::Context::default();
    for _ in 0..3 {
        let _ = common::描画する(&eguiの本体, vec![], &空白キーと入力欄);
    }
    for _ in 0..3 {
        let _ = common::描画する(&eguiの本体, vec![], &空白キーとボタン);
    }
    let タブ = vec![common::キー押下(egui::Key::Tab, 無し)];
    let _ = common::描画する(&eguiの本体, タブ, &空白キーとボタン);
    let フォーカス = eguiの本体.memory(|記憶| 記憶.focused());
    assert!(フォーカス.is_some(), "Tab でボタンにフォーカスが移る");
    assert!(
        フォーカス.is_some_and(
            |識別子| egui::text_edit::TextEditState::load(&eguiの本体, 識別子).is_some()
        ),
        "この試験は、ボタンの識別子に前の入力欄の状態が残っている場合を確かめる"
    );
    let 空白 = vec![common::キー押下(egui::Key::Space, 無し)];
    let mut 集めた = common::描画する(&eguiの本体, 空白, &空白キーとボタン);
    集めた.extend(common::描画する(&eguiの本体, vec![], &空白キーとボタン));
    assert_eq!(集めた, vec![応答::空白を押した]);
}

fn エスケープと入力欄() -> ノード<応答> {
    縦積み(子![
        キー操作(キーの組::単独(egui::Key::Escape), 応答::エスケープを押した),
        一行テキスト入力("値", 応答::文字を変えた).確定時のみ発行("確定"),
    ])
    .into()
}

#[test]
fn 入力欄の編集をエスケープでやめた回はキー操作が消費せず_次の回からは発行する() {
    let eguiの本体 = egui::Context::default();
    assert!(common::左クリックする(&eguiの本体, 入力欄の位置, &エスケープと入力欄).is_empty());
    assert!(
        eguiの本体.memory(|記憶| 記憶.focused()).is_some(),
        "入力欄にフォーカスがある"
    );
    let エスケープ = || vec![common::キー押下(egui::Key::Escape, 無し)];
    let やめた回 = common::描画する(&eguiの本体, エスケープ(), &エスケープと入力欄);
    assert!(やめた回.is_empty(), "{やめた回:?}");
    assert!(
        eguiの本体.memory(|記憶| 記憶.focused()).is_none(),
        "入力欄のフォーカスが外れる"
    );
    let _ = common::描画する(&eguiの本体, vec![], &エスケープと入力欄);
    let 次の回 = common::描画する(&eguiの本体, エスケープ(), &エスケープと入力欄);
    assert_eq!(次の回, vec![応答::エスケープを押した]);
}
