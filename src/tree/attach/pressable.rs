//! 押せる領域。子の描いた範囲の全体を押せるようにし、押されたら応答を発する。
//! 一覧の中の1枚の札のように、中にボタンや入力欄を持つまとまりの全体を押して選ぶときに使う。
//!
//! 押下を受ける面は、子を描く前に egui の領域の背面として登録する。egui は後に登録した部品を上として押下を配るため、
//! 中のボタンや入力欄を押したときはその部品だけが押され、この領域の応答は発しない。

use std::rc::Rc;

use crate::tree::ノード;

/// 押せる領域型とは、子と、子の範囲が押されたときの応答の組の記述のことである。
pub struct 押せる領域型<M> {
    子: ノード<M>,
    応答: M,
}

impl<M> 押せる領域型<M> {
    pub(crate) fn 新規(子: ノード<M>, 応答: M) -> Self {
        Self { 子, 応答 }
    }
}

impl<M: Clone> 押せる領域型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 作り = egui::UiBuilder::new().sense(egui::Sense::click());
        let 反応 = ui
            .scope_builder(作り, |内側| self.子.描画する(内側, 発行した応答))
            .response;
        if 反応.clicked() {
            発行した応答.push(self.応答.clone());
        }
        反応
    }
}

impl<M: 'static> 押せる領域型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 押せる領域型<N> {
        押せる領域型 {
            応答: 応答を変換する(self.応答),
            子: self.子.rcで写す(応答を変換する),
        }
    }
}
