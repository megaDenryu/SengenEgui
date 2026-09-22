//! 見えない部品の族。画面に何も描かず、そのフレームの入力の事象を見張って応答を発する部品のまとまりである。

mod shortcut;

use std::rc::Rc;

pub use shortcut::{キーの組, キー操作型};

/// 見えない部品とは、何も描かずに入力を見張る部品の区別のことである。
pub enum 見えない部品<M> {
    /// キーの組が押されたら応答を発する。
    キー操作(キー操作型<M>),
}

impl<M: Clone> 見えない部品<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        match self {
            Self::キー操作(中身) => 中身.描画する(ui, 発行した応答),
        }
    }
}

impl<M: 'static> 見えない部品<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 見えない部品<N> {
        match self {
            Self::キー操作(中身) => {
                見えない部品::キー操作(中身.写す(&*応答を変換する))
            }
        }
    }
}

ノードへ変換する!(見えない, 見えない部品::キー操作, キー操作型<M>);
