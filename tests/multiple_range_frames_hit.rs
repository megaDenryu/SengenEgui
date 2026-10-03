//! 複数の範囲枠の掴む順を確かめる。選んでいる枠の隅のつまみは手前の枠の内側より先に掴み、選んでいない枠の隅のつまみは掴まない。
//! どの枠にも当たらない位置と主ボタン以外のドラッグには反応しない。

mod common;

use common::multiple_range_frames::{
    二つの枠の木, 基準の矩形の中の位置, 置いた枠, 選んでいる枠
};
use sengen_egui::{範囲枠の掴んだ部分, 複数の範囲枠の操作};

#[test]
fn 選んでいる奥の枠の隅のつまみは手前の枠の内側にあっても先に掴む() {
    let eguiの本体 = egui::Context::default();
    let 始点 = 基準の矩形の中の位置(&eguiの本体, 0.6, 0.6);
    let 終点 = 始点 + egui::vec2(10.0, 10.0);
    let 集まり = common::drag::左ドラッグする(&eguiの本体, 始点, 終点, &|| {
        二つの枠の木(選んでいる枠::奥)
    });
    assert_eq!(
        集まり.first(),
        Some(&(
            置いた枠::奥,
            複数の範囲枠の操作::掴み始めた {
                掴んだ部分: 範囲枠の掴んだ部分::右下の隅
            }
        )),
        "{集まり:?}"
    );
}

#[test]
fn 選んでいない枠の隅のつまみの位置は掴めず枠の外側なら何も発しない() {
    let eguiの本体 = egui::Context::default();
    let 始点 = 基準の矩形の中の位置(&eguiの本体, 0.9, 0.9) + egui::vec2(3.0, 3.0);
    let 終点 = 始点 + egui::vec2(-40.0, -20.0);
    let 集まり = common::drag::左ドラッグする(&eguiの本体, 始点, 終点, &|| {
        二つの枠の木(選んでいる枠::奥)
    });
    assert!(集まり.is_empty(), "{集まり:?}");
}

#[test]
fn どの枠にも当たらない位置で始めたドラッグは操作を発しない() {
    let eguiの本体 = egui::Context::default();
    let 始点 = 基準の矩形の中の位置(&eguiの本体, 0.95, 0.05);
    let 終点 = 始点 + egui::vec2(-30.0, 30.0);
    let 集まり = common::drag::左ドラッグする(&eguiの本体, 始点, 終点, &|| {
        二つの枠の木(選んでいる枠::無し)
    });
    assert!(集まり.is_empty(), "{集まり:?}");
}

#[test]
fn 右ボタンと中ボタンのドラッグは操作を発しない() {
    let eguiの本体 = egui::Context::default();
    let 始点 = 基準の矩形の中の位置(&eguiの本体, 0.2, 0.2);
    let 終点 = 始点 + egui::vec2(20.0, 10.0);
    for ボタン in [egui::PointerButton::Secondary, egui::PointerButton::Middle] {
        let 木を組む = || 二つの枠の木(選んでいる枠::無し);
        let 集まり = common::drag::ドラッグする(&eguiの本体, 始点, 終点, ボタン, &木を組む);
        assert!(集まり.is_empty(), "{ボタン:?} {集まり:?}");
    }
}
