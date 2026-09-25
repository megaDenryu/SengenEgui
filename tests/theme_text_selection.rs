//! テーマを適用した入力欄の範囲選択の地が、強調色でなく、強調色を地の色の方へ寄せた色で塗られることを、
//! 描いた文字の図形の頂点の色で確かめる。選択した文字は普段の文字色のまま描かれるため、地の側を差し替える。

mod common;

use common::accent::{地の色, 文字色, 琥珀, 琥珀のテーマ};
use common::glyph::文字の図形一覧;
use egui::Color32;
use sengen_egui::{ノード, 一行テキスト入力, 修飾キー};

#[test]
fn 入力欄の範囲選択の地は強調色でなく地の色の方へ寄せた色で塗られる() {
    let eguiの本体 = egui::Context::default();
    琥珀のテーマ(None).適用する(&eguiの本体);
    let 木を組む = || -> ノード<()> { 一行テキスト入力("選ぶ文字", |_| ()).into() };
    let _ = common::左クリックする(&eguiの本体, egui::pos2(20.0, 18.0), &木を組む);
    let 全選択 = common::キー押下(egui::Key::A, 修飾キー::COMMAND);
    let 入力 = egui::RawInput {
        events: vec![全選択],
        ..Default::default()
    };
    let (_, 出力) = common::output::入力を指定して描画する(&eguiの本体, 入力, &木を組む);
    let 地の色一覧: Vec<Color32> = 文字の図形一覧(&出力)
        .into_iter()
        .filter(|図形| 図形.galley.text() == "選ぶ文字")
        .flat_map(|図形| {
            図形
                .galley
                .rows
                .iter()
                .flat_map(|行| 行.row.visuals.mesh.vertices.clone())
                .collect::<Vec<_>>()
        })
        .map(|頂点| 頂点.color)
        .filter(|色| *色 != Color32::PLACEHOLDER && *色 != 文字色)
        .collect();
    let Some(地) = 地の色一覧.first().copied() else {
        panic!("範囲選択の地が描かれていない");
    };
    assert_ne!(地, 琥珀, "{地の色一覧:?}");
    let 間にある = 地
        .to_array()
        .into_iter()
        .zip(琥珀.to_array().into_iter().zip(地の色.to_array()))
        .all(|(値, (左, 右))| 左.min(右) <= 値 && 値 <= 左.max(右));
    assert!(間にある, "範囲選択の地 {地:?} が強調色と地の色の間にない");
}
