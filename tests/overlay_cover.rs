//! 重ねる容器の覆いが、下地への押下・ドラッグ・キーボードの操作を止め、覆いの上の子は押せることを確かめる。

mod common;

use common::image::横長のテクスチャを登録する;
use common::overlay::{下地の寸法, 下地の左上, 押せる下地, 覆い};
use sengen_egui::{スライダー, ボタン, 無し, 画素, 重ねる};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    下地を押した,
    重ねたボタンを押した,
    下地の値を変えた(f32),
}

#[test]
fn 覆っている間は下地を押せず中央に重ねたボタンは押せる() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 木を組む = || {
        重ねる("重ね", 押せる下地(&テクスチャ, 応答::下地を押した))
            .覆って中央に置く(覆い, ボタン("次へ", 応答::重ねたボタンを押した))
            .into()
    };
    let _ = common::描画する(&eguiの本体, vec![], &木を組む);
    let 左上の近く = 下地の左上 + egui::vec2(16.0, 16.0);
    assert!(common::左クリックする(&eguiの本体, 左上の近く, &木を組む).is_empty());
    let 中央 = 下地の左上 + egui::vec2(下地の寸法.0 / 2.0, 下地の寸法.1 / 2.0);
    let 中央の押下 = common::左クリックする(&eguiの本体, 中央, &木を組む);
    assert_eq!(中央の押下, vec![応答::重ねたボタンを押した]);
}

#[test]
fn 覆っている間は下地のスライダーをドラッグできない() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = || {
        重ねる(
            "重ね",
            スライダー(0.5_f32, 0.0..=1.0, 応答::下地の値を変えた).幅(画素(200.0)),
        )
        .覆って中央に置く(覆い, 無し())
        .into()
    };
    let _ = common::描画する(&eguiの本体, vec![], &木を組む);
    let 集まり = common::drag::左ドラッグする(
        &eguiの本体,
        egui::pos2(20.0, 18.0),
        egui::pos2(180.0, 18.0),
        &木を組む,
    );
    assert!(集まり.is_empty(), "{集まり:?}");
}

#[test]
fn 覆っている間は下地のスライダーをキーボードでも操作できない() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = || {
        重ねる(
            "重ね",
            スライダー(0.5_f32, 0.0..=1.0, 応答::下地の値を変えた).刻み(0.25),
        )
        .覆って中央に置く(覆い, 無し())
        .into()
    };
    let _ = common::描画する(&eguiの本体, vec![], &木を組む);
    let タブ = common::キー押下(egui::Key::Tab, egui::Modifiers::NONE);
    let _ = common::描画する(&eguiの本体, vec![タブ], &木を組む);
    let 右 = common::キー押下(egui::Key::ArrowRight, egui::Modifiers::NONE);
    let 集まり = common::描画する(&eguiの本体, vec![右], &木を組む);
    assert!(集まり.is_empty(), "{集まり:?}");
}
