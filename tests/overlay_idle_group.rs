//! 重ねる容器の「ポインタが止まると隠して上に置く」子を組にすると、入れ子にした2つの重ねるの子が一緒に出し入れされることを確かめる。
//! 外側の重ねるの下の辺に「操作の欄」を、内側の重ねるの左上に「編集へ戻る」を置き、2つを同じ組にする。

mod common;

use std::time::Duration;

use common::output::文字を描いたか;
use common::overlay_idle::{
    ポインタの移動, 下地の真ん中, 描く, 操作の欄の真ん中
};
use sengen_egui::{
    ノード, ボタン, ポインタが止まると隠す指定, ポインタが止まると隠す組, 大きさを決めた領域,
    画素の組, 重ねる, 重ねる位置,
};

const 編集へ戻るの上: egui::Pos2 = egui::pos2(24.0, 24.0);

fn 入れ子の木() -> ノード<()> {
    let 指定 = ポインタが止まると隠す指定::作成する(Duration::from_secs(2))
        .隠している間はカーソルも隠す()
        .組にする(ポインタが止まると隠す組::作成する(
            "再生画面の操作",
        ));
    let 内側 = 重ねる("内側", 大きさを決めた領域(画素の組(300.0, 200.0), vec![]))
        .ポインタが止まると隠して上に置く(
            重ねる位置::左上,
            指定,
            ボタン("編集へ戻る", ()),
        );
    重ねる("外側", 内側)
        .ポインタが止まると隠して上に置く(
            重ねる位置::下,
            指定,
            ボタン("操作の欄", ()),
        )
        .into()
}

fn 入れ子の木を描く(
    eguiの本体: &egui::Context,
    秒: f64,
    出来事一覧: Vec<egui::Event>,
) -> egui::FullOutput {
    描く(eguiの本体, 秒, 出来事一覧, &入れ子の木)
}

fn 両方を描いたか(出力: &egui::FullOutput) -> (bool, bool) {
    (
        文字を描いたか(出力, "操作の欄"),
        文字を描いたか(出力, "編集へ戻る"),
    )
}

/// 位置へポインタを動かして止め、隠すまでの時間より長く経った回の出力を返す。最初の回は子の大きさを測る。
fn 止めて待つ(位置: egui::Pos2) -> egui::FullOutput {
    let eguiの本体 = egui::Context::default();
    let _ = 入れ子の木を描く(&eguiの本体, 0.0, ポインタの移動(下地の真ん中));
    let _ = 入れ子の木を描く(&eguiの本体, 0.1, ポインタの移動(位置));
    let _ = 入れ子の木を描く(&eguiの本体, 0.2, vec![]);
    入れ子の木を描く(&eguiの本体, 5.0, vec![])
}

#[test]
fn 外側の子の上で止めると両方とも出たままでカーソルも出たまま() {
    let 待った回 = 止めて待つ(操作の欄の真ん中);
    assert_eq!(両方を描いたか(&待った回), (true, true));
    assert_ne!(待った回.platform_output.cursor_icon, egui::CursorIcon::None);
}

#[test]
fn 内側の子の上で止めると両方とも出たままでカーソルも出たまま() {
    let 待った回 = 止めて待つ(編集へ戻るの上);
    assert_eq!(両方を描いたか(&待った回), (true, true));
    assert_ne!(待った回.platform_output.cursor_icon, egui::CursorIcon::None);
}

#[test]
fn どちらの子の上でもなく止めると両方同時に隠れてカーソルも隠れる() {
    let eguiの本体 = egui::Context::default();
    let _ = 入れ子の木を描く(&eguiの本体, 0.0, ポインタの移動(下地の真ん中));
    let 動かした回 = 入れ子の木を描く(
        &eguiの本体,
        0.1,
        ポインタの移動(下地の真ん中 + egui::vec2(4.0, 0.0)),
    );
    assert_eq!(両方を描いたか(&動かした回), (true, true));
    let 止まっている間 = 入れ子の木を描く(&eguiの本体, 1.9, vec![]);
    assert_eq!(両方を描いたか(&止まっている間), (true, true));
    let 過ぎた回 = 入れ子の木を描く(&eguiの本体, 2.3, vec![]);
    assert_eq!(両方を描いたか(&過ぎた回), (false, false));
    assert_eq!(過ぎた回.platform_output.cursor_icon, egui::CursorIcon::None);
}
