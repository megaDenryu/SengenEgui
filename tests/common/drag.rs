//! ドラッグの出来事の合成。押す・動かす・放すのフレームを順に回し、発した応答を集める。

use sengen_egui::ノード;

use super::{ボタンの出来事, 描画する};

/// 最初のフレームで配置を決め、始点でボタンを押し、中間点を経て終点で放すまでの全フレームの応答を返す。
pub fn ドラッグする<M: Clone>(
    eguiの本体: &egui::Context,
    始点: egui::Pos2,
    終点: egui::Pos2,
    ボタン: egui::PointerButton,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    let mut 集まり = 描画する(eguiの本体, vec![], 木を組む);
    let 押す = vec![
        egui::Event::PointerMoved(始点),
        ボタンの出来事(始点, ボタン, true),
    ];
    集まり.extend(描画する(eguiの本体, 押す, 木を組む));
    for 位置 in [始点.lerp(終点, 0.5), 終点] {
        let 出来事一覧 = vec![egui::Event::PointerMoved(位置)];
        集まり.extend(描画する(eguiの本体, 出来事一覧, 木を組む));
    }
    let 放す = ボタンの出来事(終点, ボタン, false);
    集まり.extend(描画する(eguiの本体, vec![放す], 木を組む));
    集まり
}

/// 左ボタンでのドラッグの全フレームを回し、発した応答を返す。
pub fn 左ドラッグする<M: Clone>(
    eguiの本体: &egui::Context,
    始点: egui::Pos2,
    終点: egui::Pos2,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    ドラッグする(
        eguiの本体,
        始点,
        終点,
        egui::PointerButton::Primary,
        木を組む,
    )
}

/// 修飾キーを押したまま、左ボタンでのドラッグの全フレームを回し、発した応答を返す。
/// どのフレームの入力にも、押している修飾キーとして同じ修飾キーを入れる。
pub fn 修飾キーを押して左ドラッグする<M: Clone>(
    eguiの本体: &egui::Context,
    始点: egui::Pos2,
    終点: egui::Pos2,
    修飾キー: egui::Modifiers,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    let 一フレーム描く = |出来事一覧: Vec<egui::Event>| {
        let 入力 = egui::RawInput {
            events: 出来事一覧,
            modifiers: 修飾キー,
            ..Default::default()
        };
        super::output::入力を指定して描画する(eguiの本体, 入力, 木を組む).0
    };
    let ボタンを押すか放す = |位置: egui::Pos2, 押した: bool| egui::Event::PointerButton {
        pos: 位置,
        button: egui::PointerButton::Primary,
        pressed: 押した,
        modifiers: 修飾キー,
    };
    let mut 集まり = 一フレーム描く(vec![]);
    集まり.extend(一フレーム描く(vec![
        egui::Event::PointerMoved(始点),
        ボタンを押すか放す(始点, true),
    ]));
    for 位置 in [始点.lerp(終点, 0.5), 終点] {
        集まり.extend(一フレーム描く(vec![egui::Event::PointerMoved(位置)]));
    }
    集まり.extend(一フレーム描く(vec![ボタンを押すか放す(
        終点, false,
    )]));
    集まり
}
