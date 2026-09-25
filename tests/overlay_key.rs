//! 重ねる子の大きさの記憶が置いた位置ごとに分かれ、条件付きで別の位置の子を足し引きしても、
//! 残った子が別の子の大きさで寄せられてずれないことを確かめる。

mod common;

use std::cell::Cell;
use std::rc::Rc;

use common::output::入力を指定して描画する;
use sengen_egui::{
    ノード, ボタン, 大きさを決めた領域, 画素の組, 重ねる, 重ねる位置
};

#[test]
fn 前に置いた条件付きの子を外した回にも後の子は同じ位置に描かれる() {
    let eguiの本体 = egui::Context::default();
    let 左上を出す = Rc::new(Cell::new(true));
    let 出すか = Rc::clone(&左上を出す);
    let 木を組む = move || -> ノード<()> {
        let mut 重ね = 重ねる("重ね", 大きさを決めた領域(画素の組(300.0, 100.0), vec![]));
        if 出すか.get() {
            重ね = 重ね.上に置く(
                重ねる位置::左上,
                ボタン("左上に置いた長い見出しのボタン", ()),
            );
        }
        重ね.上に置く(重ねる位置::右下, ボタン("右下", ())).into()
    };
    for _ in 0..2 {
        let _ =
            入力を指定して描画する(&eguiの本体, egui::RawInput::default(), &木を組む);
    }
    左上を出す.set(false);
    let 外した回 = 右下の文字の位置(&eguiの本体, &木を組む);
    let 次の回 = 右下の文字の位置(&eguiの本体, &木を組む);
    assert!(外した回.is_some());
    assert_eq!(外した回, 次の回);
}

fn 右下の文字の位置(
    eguiの本体: &egui::Context,
    木を組む: &dyn Fn() -> ノード<()>,
) -> Option<egui::Pos2> {
    let (_, 出力) =
        入力を指定して描画する(eguiの本体, egui::RawInput::default(), 木を組む);
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた図形| 文字の位置(&切り抜いた図形.shape, "右下"))
}

fn 文字の位置(図形: &egui::Shape, 文字列: &str) -> Option<egui::Pos2> {
    match 図形 {
        egui::Shape::Text(文字) if 文字.galley.text() == 文字列 => Some(文字.pos),
        egui::Shape::Vec(図形一覧) => {
            図形一覧.iter().find_map(|中| 文字の位置(中, 文字列))
        }
        _ => None,
    }
}
