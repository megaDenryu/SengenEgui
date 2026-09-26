//! 重ねる容器の「ポインタが止まると隠して上に置く」子が、下地の上でポインタが動いてから指定の時間だけ描かれ、
//! 子の上にポインタがある間・子の上で押し始めて押し続けている間・出したままにする条件が真の間は隠れないことを確かめる。

mod common;

use std::time::Duration;

use common::output::{
    入力を指定して描画する, 描き直しまでの時間, 文字を描いたか
};
use sengen_egui::{
    ノード, ボタン, ポインタが止まると隠す指定, 大きさを決めた領域, 画素の組, 重ねる, 重ねる位置,
};

const 隠すまでの時間: Duration = Duration::from_secs(2);
const 下地の真ん中: egui::Pos2 = egui::pos2(158.0, 58.0);

fn 木(出したままにするか: bool) -> impl Fn() -> ノード<()> {
    move || {
        let 指定 = ポインタが止まると隠す指定::作成する(隠すまでの時間)
            .出したままにする(出したままにするか)
            .隠している間はカーソルも隠す();
        重ねる("重ね", 大きさを決めた領域(画素の組(300.0, 200.0), vec![]))
            .ポインタが止まると隠して上に置く(
                重ねる位置::下,
                指定,
                ボタン("操作の欄", ()),
            )
            .into()
    }
}

fn 描く(
    eguiの本体: &egui::Context,
    秒: f64,
    出来事一覧: Vec<egui::Event>,
    木を組む: &dyn Fn() -> ノード<()>,
) -> egui::FullOutput {
    let 入力 = egui::RawInput {
        time: Some(秒),
        events: 出来事一覧,
        ..Default::default()
    };
    入力を指定して描画する(eguiの本体, 入力, 木を組む).1
}

fn 動かす(位置: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerMoved(位置)]
}

/// 子の大きさを測る最初の回を済ませ、子の真ん中の位置を返す。子は下地の下の辺の中央に寄る。
fn 測りを済ませる(
    eguiの本体: &egui::Context,
    木を組む: &dyn Fn() -> ノード<()>,
) -> egui::Pos2 {
    let _ = 描く(eguiの本体, 0.0, 動かす(下地の真ん中), 木を組む);
    egui::pos2(158.0, 188.0)
}

#[test]
fn 下地の上で動かすと出し_止まって指定の時間が経つと隠してカーソルも隠す() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = 木(false);
    let _ = 測りを済ませる(&eguiの本体, &木を組む);
    let 動かした回 = 描く(&eguiの本体, 0.1, 動かす(下地の真ん中), &木を組む);
    assert!(文字を描いたか(&動かした回, "操作の欄"));
    assert!(描き直しまでの時間(&動かした回) <= 隠すまでの時間);
    let 止まっている間 = 描く(&eguiの本体, 1.9, vec![], &木を組む);
    assert!(文字を描いたか(&止まっている間, "操作の欄"));
    let 過ぎた回 = 描く(&eguiの本体, 2.2, vec![], &木を組む);
    assert!(!文字を描いたか(&過ぎた回, "操作の欄"));
    assert_eq!(過ぎた回.platform_output.cursor_icon, egui::CursorIcon::None);
    let また動かした回 = 描く(
        &eguiの本体,
        2.3,
        動かす(下地の真ん中 + egui::vec2(4.0, 0.0)),
        &木を組む,
    );
    assert!(文字を描いたか(&また動かした回, "操作の欄"));
}

#[test]
fn 子の上にポインタがある間は止まっていても隠さない() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = 木(false);
    let 子の真ん中 = 測りを済ませる(&eguiの本体, &木を組む);
    let _ = 描く(&eguiの本体, 0.1, 動かす(子の真ん中), &木を組む);
    let 過ぎた回 = 描く(&eguiの本体, 5.0, vec![], &木を組む);
    assert!(文字を描いたか(&過ぎた回, "操作の欄"));
}

#[test]
fn 子の上で押し始めたボタンを押し続けている間は子の外へ動かしても隠さない() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = 木(false);
    let 子の真ん中 = 測りを済ませる(&eguiの本体, &木を組む);
    let _ = 描く(&eguiの本体, 0.1, 動かす(子の真ん中), &木を組む);
    let 押す = common::ボタンの出来事(子の真ん中, egui::PointerButton::Primary, true);
    let _ = 描く(&eguiの本体, 0.2, vec![押す], &木を組む);
    let _ = 描く(&eguiの本体, 0.3, 動かす(下地の真ん中), &木を組む);
    let 押し続けている回 = 描く(&eguiの本体, 5.0, vec![], &木を組む);
    assert!(文字を描いたか(&押し続けている回, "操作の欄"));
    let 放す = common::ボタンの出来事(下地の真ん中, egui::PointerButton::Primary, false);
    let _ = 描く(&eguiの本体, 5.1, vec![放す], &木を組む);
    let 放して過ぎた回 = 描く(&eguiの本体, 7.5, vec![], &木を組む);
    assert!(!文字を描いたか(&放して過ぎた回, "操作の欄"));
}

#[test]
fn 出したままにする間は隠さず_偽に戻すと最後に真だった回から数え始める() {
    let eguiの本体 = egui::Context::default();
    let 出したまま = 木(true);
    let _ = 測りを済ませる(&eguiの本体, &出したまま);
    let 過ぎた回 = 描く(&eguiの本体, 10.0, vec![], &出したまま);
    assert!(文字を描いたか(&過ぎた回, "操作の欄"));
    assert_ne!(過ぎた回.platform_output.cursor_icon, egui::CursorIcon::None);
    let 戻した木 = 木(false);
    let 戻した直後 = 描く(&eguiの本体, 11.0, vec![], &戻した木);
    assert!(文字を描いたか(&戻した直後, "操作の欄"));
    let 戻して過ぎた回 = 描く(&eguiの本体, 12.5, vec![], &戻した木);
    assert!(!文字を描いたか(&戻して過ぎた回, "操作の欄"));
}

#[test]
fn 前の回に描いていなかった子は出した回から数え始める() {
    let eguiの本体 = egui::Context::default();
    let 木を組む = 木(false);
    let _ = 測りを済ませる(&eguiの本体, &木を組む);
    let 外した木 = || 大きさを決めた領域(画素の組(300.0, 200.0), vec![]).into();
    let _ = 描く(&eguiの本体, 1.0, vec![], &外した木);
    let 戻した回 = 描く(&eguiの本体, 30.0, vec![], &木を組む);
    assert!(文字を描いたか(&戻した回, "操作の欄"));
}
