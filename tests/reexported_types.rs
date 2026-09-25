//! 再公開した色・キー・修飾キーが egui の型そのものであり、利用する側が egui を名指さずにスタイルとキーの組を書けることを確かめる。

use sengen_egui::{キー, キーの組, スタイル, 修飾キー, 色};

const 琥珀の文字: スタイル = スタイル {
    文字色: Some(色::from_rgb(245, 158, 11)),
    ..スタイル::無指定
};

#[test]
fn 再公開した色でスタイルの定数を書ける() {
    assert_eq!(
        琥珀の文字.文字色,
        Some(egui::Color32::from_rgb(245, 158, 11))
    );
}

#[test]
fn 再公開したキーと修飾キーでキーの組を書ける() {
    assert_eq!(
        キーの組::生成する(修飾キー::CTRL, キー::S),
        キーの組::生成する(egui::Modifiers::CTRL, egui::Key::S)
    );
    assert_eq!(
        キーの組::単独(キー::Enter),
        キーの組::生成する(修飾キー::NONE, egui::Key::Enter)
    );
}
