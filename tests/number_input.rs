//! 数値入力の小数点以下の桁数の指定が、表示する数の桁を揃えることを確かめる。

mod common;

use common::output::{入力を指定して描画する, 文字を描いたか};
use sengen_egui::{ノード, 数値入力};

#[test]
fn 小数点以下の桁数を指定すると数をその桁数で表示する() {
    let eguiの本体 = egui::Context::default();
    let 木 = || -> ノード<f64> {
        数値入力(1.5_f64, 0.0..=10.0, |値| 値)
            .小数点以下の桁数(2)
            .into()
    };
    let (_, 出力) = 入力を指定して描画する(&eguiの本体, egui::RawInput::default(), &木);
    assert!(文字を描いたか(&出力, "1.50"));
}
