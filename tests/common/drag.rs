//! ドラッグの出来事の合成。押す・動かす・放すのフレームを順に回し、発した応答を集める。

use sengen_egui::ノード;

use super::{ボタンの出来事, 描画する};

/// 最初のフレームで配置を決め、始点でボタンを押し、中間点を経て終点で放すまでの全フレームの応答を返す。
pub fn ドラッグする<M: Clone>(
    文脈: &egui::Context,
    始点: egui::Pos2,
    終点: egui::Pos2,
    ボタン: egui::PointerButton,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    let mut 集まり = 描画する(文脈, vec![], 木を組む);
    let 押す = vec![
        egui::Event::PointerMoved(始点),
        ボタンの出来事(始点, ボタン, true),
    ];
    集まり.extend(描画する(文脈, 押す, 木を組む));
    for 位置 in [始点.lerp(終点, 0.5), 終点] {
        let 出来事一覧 = vec![egui::Event::PointerMoved(位置)];
        集まり.extend(描画する(文脈, 出来事一覧, 木を組む));
    }
    let 放す = ボタンの出来事(終点, ボタン, false);
    集まり.extend(描画する(文脈, vec![放す], 木を組む));
    集まり
}

/// 左ボタンでのドラッグの全フレームを回し、発した応答を返す。
pub fn 左ドラッグする<M: Clone>(
    文脈: &egui::Context,
    始点: egui::Pos2,
    終点: egui::Pos2,
    木を組む: &dyn Fn() -> ノード<M>,
) -> Vec<M> {
    ドラッグする(文脈, 始点, 終点, egui::PointerButton::Primary, 木を組む)
}
