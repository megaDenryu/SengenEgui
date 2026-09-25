//! テストの共通部。egui の入力を合成して木を描画し、発した応答を集める。
//! 木は毎フレーム組み直すため、木を組む関数を受け取ってフレームごとに呼ぶ。

#![allow(dead_code)]

pub mod accent;
pub mod drag;
pub mod font_measure;
pub mod glyph;
pub mod image;
pub mod output;
pub mod overlay;
pub mod range_band;
pub mod range_frame;

use sengen_egui::ノード;

/// 出来事を入れて1フレーム描画し、発した応答を返す。
pub fn 描画する<M: Clone>(
    文脈: &egui::Context,
    出来事一覧: Vec<egui::Event>,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    let mut 集まり = Vec::new();
    let 入力 = egui::RawInput {
        events: 出来事一覧,
        ..Default::default()
    };
    let _ = 文脈.run(入力, |文脈| {
        egui::CentralPanel::default().show(文脈, |ui| {
            集まり.extend(木を組む().描画して集める(ui));
        });
    });
    集まり
}

/// ポインタのボタンの出来事を作る。
pub fn ボタンの出来事(
    位置: egui::Pos2,
    ボタン: egui::PointerButton,
    押した: bool,
) -> egui::Event {
    egui::Event::PointerButton {
        pos: 位置,
        button: ボタン,
        pressed: 押した,
        modifiers: egui::Modifiers::default(),
    }
}

/// 位置へ移動し、ボタンを押して放す3フレームを回し、全フレームで発した応答を返す。
/// 最初のフレームは配置を決めるためだけに描画する（egui は前フレームの配置で当たりを判定する）。
pub fn クリックする<M: Clone>(
    文脈: &egui::Context,
    位置: egui::Pos2,
    ボタン: egui::PointerButton,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    let mut 集まり = 描画する(文脈, vec![], 木を組む);
    集まり.extend(描画する(
        文脈,
        vec![
            egui::Event::PointerMoved(位置),
            ボタンの出来事(位置, ボタン, true),
        ],
        木を組む,
    ));
    集まり.extend(描画する(
        文脈,
        vec![ボタンの出来事(位置, ボタン, false)],
        木を組む,
    ));
    集まり
}

/// 左クリックの3フレームを回し、発した応答を返す。
pub fn 左クリックする<M: Clone>(
    文脈: &egui::Context,
    位置: egui::Pos2,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    クリックする(文脈, 位置, egui::PointerButton::Primary, 木を組む)
}

/// キーの押下の出来事を作る。
pub fn キー押下(キー: egui::Key, 修飾キー: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key: キー,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: 修飾キー,
    }
}

/// 文字入力の出来事を作る。
pub fn 文字入力(文字: &str) -> egui::Event {
    egui::Event::Text(文字.to_string())
}
