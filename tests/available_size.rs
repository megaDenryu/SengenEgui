//! 使える大きさから組む容器が、置いた位置から親の領域の端までの大きさを子を組む手続きへ渡すことを確かめる。

mod common;

use std::cell::Cell;
use std::rc::Rc;

use common::output::入力を指定して描画する;
use sengen_egui::{
    ノード, 使える大きさから組む, 子, 文字表示, 無し, 縦横の論理画素, 縦積み
};

fn 画面が400かける300の入力() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 300.0),
        )),
        ..Default::default()
    }
}

#[test]
fn 子を組む手続きは置いた位置から親の領域の右端と下端までの大きさを受け取る() {
    let eguiの本体 = egui::Context::default();
    let 受け取った大きさ = Rc::new(Cell::new(None::<縦横の論理画素>));
    let 記録先 = Rc::clone(&受け取った大きさ);
    let 木を組む = move || -> ノード<()> {
        let 記録先 = Rc::clone(&記録先);
        縦積み(子![
            文字表示("見出し"),
            使える大きさから組む(move |大きさ| {
                記録先.set(Some(大きさ));
                無し()
            }),
        ])
        .into()
    };
    let _ = 入力を指定して描画する(&eguiの本体, 画面が400かける300の入力(), &木を組む);
    let Some(大きさ) = 受け取った大きさ.get() else {
        panic!("子を組む手続きが呼ばれていない");
    };
    let 余白の幅 = 16.0;
    assert_eq!(大きさ.横.eguiへ渡す値(), 400.0 - 余白の幅);
    assert!(
        大きさ.縦.eguiへ渡す値() < 300.0 - 余白の幅,
        "見出しの分だけ高さが減る"
    );
}
