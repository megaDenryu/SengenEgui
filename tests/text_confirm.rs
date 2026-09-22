//! 確定時のみ発行の入力欄を、合成した入力で確かめる。フォーカスが外れたら発行し、Escape は取り消し、
//! 木から消えて再表示されたら古い下書きを捨てる。

mod common;

use sengen_egui::{ノード, 一行テキスト入力, 子, 条件付き表示, 縦積み};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    文字を変えた(String),
}

const 入力欄の位置: egui::Pos2 = egui::pos2(30.0, 18.0);
const 何も無い位置: egui::Pos2 = egui::pos2(400.0, 400.0);

fn 入力欄のある木(出す: bool) -> ノード<応答> {
    縦積み(子![条件付き表示(出す, || {
        一行テキスト入力("値", 応答::文字を変えた).確定時のみ発行("確定")
    })])
    .into()
}

fn 出す木() -> ノード<応答> {
    入力欄のある木(true)
}

#[test]
fn フォーカスが外れたときだけ新しい値の応答が出る() {
    let 文脈 = egui::Context::default();
    assert!(common::左クリックする(&文脈, 入力欄の位置, &出す木).is_empty());
    let 入力中 = common::描画する(&文脈, vec![common::文字入力("a")], &出す木);
    assert!(入力中.is_empty(), "{入力中:?}");
    let 確定 = common::左クリックする(&文脈, 何も無い位置, &出す木);
    assert_eq!(確定, vec![応答::文字を変えた("値a".to_string())]);
}

#[test]
fn escapeでフォーカスが外れたら下書きを捨てて応答を出さない() {
    let 文脈 = egui::Context::default();
    assert!(common::左クリックする(&文脈, 入力欄の位置, &出す木).is_empty());
    assert!(common::描画する(&文脈, vec![common::文字入力("a")], &出す木).is_empty());
    let 取り消し = common::描画する(
        &文脈,
        vec![common::キー押下(
            egui::Key::Escape,
            egui::Modifiers::NONE,
        )],
        &出す木,
    );
    assert!(取り消し.is_empty(), "{取り消し:?}");
    assert!(common::描画する(&文脈, vec![], &出す木).is_empty());
    assert!(common::左クリックする(&文脈, 入力欄の位置, &出す木).is_empty());
    assert!(common::左クリックする(&文脈, 何も無い位置, &出す木).is_empty());
}

#[test]
fn 木から消えて再表示された入力欄は古い下書きを捨てる() {
    let 文脈 = egui::Context::default();
    assert!(common::左クリックする(&文脈, 入力欄の位置, &出す木).is_empty());
    assert!(common::描画する(&文脈, vec![common::文字入力("a")], &出す木).is_empty());
    let 消す木 = || 入力欄のある木(false);
    assert!(common::描画する(&文脈, vec![], &消す木).is_empty());
    assert!(common::描画する(&文脈, vec![], &消す木).is_empty());
    assert!(common::左クリックする(&文脈, 入力欄の位置, &出す木).is_empty());
    let 確定 = common::左クリックする(&文脈, 何も無い位置, &出す木);
    assert!(確定.is_empty(), "{確定:?}");
}
