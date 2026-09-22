//! キー操作を、入力欄にフォーカスがある状態と修飾キーの厳密な一致について確かめる。

mod common;

use sengen_egui::{
    キーの組, キー操作, ノード, 一行テキスト入力, 子, 縦積み
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    文字を変えた(String),
    押した,
    保存した,
}

const 入力欄の位置: egui::Pos2 = egui::pos2(30.0, 18.0);

fn 入力欄と単独キー() -> ノード<応答> {
    縦積み(子![
        一行テキスト入力("値", 応答::文字を変えた).確定時のみ発行("確定"),
        キー操作(キーの組::単独(egui::Key::A), 応答::押した),
    ])
    .into()
}

fn 入力欄と保存の組() -> ノード<応答> {
    縦積み(子![
        一行テキスト入力("値", 応答::文字を変えた).確定時のみ発行("確定"),
        キー操作(
            キーの組::生成する(egui::Modifiers::CTRL, egui::Key::S),
            応答::保存した
        ),
    ])
    .into()
}

#[test]
fn 入力欄にフォーカスがある間は単独のキーの組を発行せず文字が入力欄へ届く() {
    let 文脈 = egui::Context::default();
    assert!(common::左クリックする(&文脈, 入力欄の位置, &入力欄と単独キー).is_empty());
    let 入力中 = common::描画する(
        &文脈,
        vec![
            common::キー押下(egui::Key::A, egui::Modifiers::NONE),
            common::文字入力("a"),
        ],
        &入力欄と単独キー,
    );
    assert!(入力中.is_empty(), "{入力中:?}");
    let 確定 = common::左クリックする(&文脈, egui::pos2(400.0, 400.0), &入力欄と単独キー);
    assert_eq!(確定, vec![応答::文字を変えた("値a".to_string())]);
}

#[test]
fn フォーカスが無ければ単独のキーの組はその回に発行される() {
    let 文脈 = egui::Context::default();
    assert!(common::描画する(&文脈, vec![], &入力欄と単独キー).is_empty());
    let 押下 = common::描画する(
        &文脈,
        vec![common::キー押下(egui::Key::A, egui::Modifiers::NONE)],
        &入力欄と単独キー,
    );
    assert_eq!(押下, vec![応答::押した]);
}

#[test]
fn 修飾キーの組は入力欄の確定の後に発行される() {
    let 文脈 = egui::Context::default();
    assert!(common::左クリックする(&文脈, 入力欄の位置, &入力欄と保存の組).is_empty());
    assert!(common::描画する(&文脈, vec![common::文字入力("a")], &入力欄と保存の組).is_empty());
    let mut 集まり = common::描画する(
        &文脈,
        vec![common::キー押下(egui::Key::S, egui::Modifiers::CTRL)],
        &入力欄と保存の組,
    );
    集まり.extend(common::描画する(&文脈, vec![], &入力欄と保存の組));
    assert_eq!(
        集まり,
        vec![応答::文字を変えた("値a".to_string()), 応答::保存した]
    );
}

#[test]
fn 修飾キーが余分に付いた押下では発行されない() {
    let 文脈 = egui::Context::default();
    assert!(common::描画する(&文脈, vec![], &入力欄と保存の組).is_empty());
    let 押下 = common::描画する(
        &文脈,
        vec![common::キー押下(
            egui::Key::S,
            egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
        )],
        &入力欄と保存の組,
    );
    assert!(押下.is_empty(), "{押下:?}");
    let 正しい押下 = common::描画する(
        &文脈,
        vec![common::キー押下(egui::Key::S, egui::Modifiers::CTRL)],
        &入力欄と保存の組,
    );
    assert_eq!(正しい押下, vec![応答::保存した]);
}
