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
    let 放した回数 = 集まり
        .iter()
        .filter(|応答| matches!(応答, 応答::放した(_)))
        .count();
    assert_eq!(放した回数, 1, "{集まり:?}");
}

fn 刻みの付いた帯() -> ノード<応答> {
    スライダー(0.5_f32, 0.0..=1.0, 応答::動かしている)
        .刻み(0.25)
        .操作を終えたら発する(応答::放した)
        .into()
}

#[test]
fn フォーカスを持つ間に矢印キーで値を変えると変わった応答の後に終えた応答が出る() {
    let eguiの本体 = egui::Context::default();
    let _ = common::描画する(&eguiの本体, vec![], &刻みの付いた帯);
    let タブ = common::キー押下(egui::Key::Tab, egui::Modifiers::NONE);
    let _ = common::描画する(&eguiの本体, vec![タブ], &刻みの付いた帯);
    let 右 = common::キー押下(egui::Key::ArrowRight, egui::Modifiers::NONE);
    let 集まり = common::描画する(&eguiの本体, vec![右], &刻みの付いた帯);
    assert_eq!(集まり, vec![応答::動かしている(0.75), 応答::放した(0.75)]);
}

fn 数値の欄の付いた帯() -> ノード<応答> {
    スライダー(0.5_f32, 0.0..=1.0, 応答::動かしている)
        .幅(sengen_egui::画素(100.0))
        .操作を終えたら発する(応答::放した)
        .into()
}

#[test]
fn 数値の欄へ打った値は打鍵のたびでなく確定したときに1回だけ応答が出る() {
    let eguiの本体 = egui::Context::default();
    let 軌道の右端 = 描画した矩形(&eguiの本体, 数値の欄の付いた帯()).right();
    let 数値の欄 = egui::pos2(軌道の右端 - 10.0, 18.0);
    let 編集を始めた = common::左クリックする(&eguiの本体, 数値の欄, &数値の欄の付いた帯);
    assert!(編集を始めた.is_empty(), "{編集を始めた:?}");
    let 全部を選ぶ = common::キー押下(egui::Key::A, egui::Modifiers::COMMAND);
    let mut 打鍵中 = common::描画する(&eguiの本体, vec![全部を選ぶ], &数値の欄の付いた帯);
    for 文字 in ["0", ".", "2"] {
        打鍵中.extend(common::描画する(
            &eguiの本体,
            vec![common::文字入力(文字)],
            &数値の欄の付いた帯,
        ));
    }
    assert!(打鍵中.is_empty(), "{打鍵中:?}");
    let 確定 = common::キー押下(egui::Key::Enter, egui::Modifiers::NONE);
    let 確定した = common::描画する(&eguiの本体, vec![確定], &数値の欄の付いた帯);
    assert_eq!(確定した, vec![応答::動かしている(0.2), 応答::放した(0.2)]);
}
