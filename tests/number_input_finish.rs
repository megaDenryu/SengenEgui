//! 数値入力の操作を終えたら発する指定を確かめる。ドラッグの間は値が変わった応答が出て、放すと終えた応答が最後に出る。
//! 直接入力は打鍵のたびでなく、確定したときに変わった応答と終えた応答が1回ずつ出る。

mod common;

use sengen_egui::{ノード, 数値入力};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    変えた(f64),
    終えた(f64),
}

const 数値入力の位置: egui::Pos2 = egui::pos2(20.0, 18.0);

fn 数値入力の木() -> ノード<応答> {
    数値入力(1.0_f64, 0.0..=100.0, 応答::変えた)
        .操作を終えたら発する(応答::終えた)
        .into()
}

#[test]
fn ドラッグの間は変えた応答が出て放すと終えた応答が最後に出る() {
    let eguiの本体 = egui::Context::default();
    let 集まり = common::drag::左ドラッグする(
        &eguiの本体,
        数値入力の位置,
        数値入力の位置 + egui::vec2(60.0, 0.0),
        &数値入力の木,
    );
    assert!(
        集まり.iter().any(|応答| matches!(応答, 応答::変えた(_))),
        "{集まり:?}"
    );
    let 終えた数 = 集まり
        .iter()
        .filter(|応答| matches!(応答, 応答::終えた(_)))
        .count();
    assert_eq!(終えた数, 1, "{集まり:?}");
    assert!(matches!(集まり.last(), Some(応答::終えた(_))), "{集まり:?}");
}

#[test]
fn 直接入力は打鍵のたびでなく確定したときに1回ずつ応答が出る() {
    let eguiの本体 = egui::Context::default();
    let 編集を始めた = common::左クリックする(&eguiの本体, 数値入力の位置, &数値入力の木);
    assert!(
        編集を始めた
            .iter()
            .all(|応答| !matches!(応答, 応答::変えた(値) if *値 != 1.0)),
        "{編集を始めた:?}"
    );
    let 全部を選ぶ = common::キー押下(egui::Key::A, egui::Modifiers::COMMAND);
    let mut 打鍵中 = common::描画する(&eguiの本体, vec![全部を選ぶ], &数値入力の木);
    for 文字 in ["4", "2"] {
        打鍵中.extend(common::描画する(
            &eguiの本体,
            vec![common::文字入力(文字)],
            &数値入力の木,
        ));
    }
    assert!(打鍵中.is_empty(), "{打鍵中:?}");
    let 確定 = common::キー押下(egui::Key::Enter, egui::Modifiers::NONE);
    let 確定した = common::描画する(&eguiの本体, vec![確定], &数値入力の木);
    assert_eq!(確定した, vec![応答::変えた(42.0), 応答::終えた(42.0)]);
}
