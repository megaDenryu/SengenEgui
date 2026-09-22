//! 説明（ツールチップ）。子の上にポインタが留まったときに文を吹き出しで出す。

use std::rc::Rc;

use crate::tree::ノード;

/// 説明型とは、子とその上に出す説明の文の組の記述のことである。
pub struct 説明型<M> {
    子: ノード<M>,
    文: String,
}

impl<M> 説明型<M> {
    pub(crate) fn 新規(子: ノード<M>, 文: String) -> Self {
        Self { 子, 文 }
    }
}

impl<M: Clone> 説明型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        self.子
            .描画する(ui, 発行した応答)
            .on_hover_text(self.文.clone())
    }
}

impl<M: 'static> 説明型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 説明型<N> {
        説明型 {
            子: self.子.rcで写す(応答を変換する),
            文: self.文,
        }
    }
}
