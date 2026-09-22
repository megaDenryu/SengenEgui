//! モーダルの Escape による「閉じたら」が1回だけ出ることを確かめる。

mod common;

use sengen_egui::{ノード, モーダル, 子, 文字表示, 縦積み};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    閉じた,
}

fn 開いた覆い() -> ノード<応答> {
    縦積み(子![
        モーダル("覆い", true, 子![文字表示("覆い")]).閉じたら(応答::閉じた)
    ])
    .into()
}

#[test]
fn escapeを押すと閉じたらの応答が1回だけ出る() {
    let 文脈 = egui::Context::default();
    assert!(common::描画する(&文脈, vec![], &開いた覆い).is_empty());
    let mut 集まり = common::描画する(
        &文脈,
        vec![common::キー押下(
            egui::Key::Escape,
            egui::Modifiers::NONE,
        )],
        &開いた覆い,
    );
    集まり.extend(common::描画する(&文脈, vec![], &開いた覆い));
    assert_eq!(集まり, vec![応答::閉じた]);
}
