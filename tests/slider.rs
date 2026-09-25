//! スライダーの、使える幅いっぱいへ広げる指定と、操作を終えたときの応答を確かめる。

mod common;

use common::image::描画した矩形;
use sengen_egui::{スライダー, ノード};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    動かしている(f32),
    放した(f32),
}

fn 再生位置の帯() -> ノード<応答> {
    スライダー(0.0_f32, 0.0..=100.0, 応答::動かしている)
        .幅いっぱい()
        .数値を隠す()
        .操作を終えたら発する(応答::放した)
        .into()
}

#[test]
fn 数値を隠して幅いっぱいにした軌道は使える幅の全部を占める() {
    let eguiの本体 = egui::Context::default();
    let 使える幅 = {
        let mut 幅 = 0.0;
        let _ = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
            egui::CentralPanel::default().show(eguiの本体, |ui| 幅 = ui.available_width());
        });
        幅
    };
    assert_eq!(描画した矩形(&eguiの本体, 再生位置の帯()).width(), 使える幅);
}

#[test]
fn ドラッグの間は動かしている応答が出て放すと放した応答が最後に出る() {
    let eguiの本体 = egui::Context::default();
    let 軌道の中央 = egui::pos2(描画した矩形(&eguiの本体, 再生位置の帯()).center().x, 18.0);
    let 集まり = common::drag::左ドラッグする(
        &eguiの本体,
        軌道の中央,
        軌道の中央 + egui::vec2(200.0, 0.0),
        &再生位置の帯,
    );
    assert!(
        集まり
            .iter()
            .any(|応答| matches!(応答, 応答::動かしている(_))),
        "{集まり:?}"
    );
    assert!(matches!(集まり.last(), Some(応答::放した(_))), "{集まり:?}");
}
