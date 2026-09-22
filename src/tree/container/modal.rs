//! モーダル。画面全体を薄く覆い、その上に子を置く。開閉はアプリ状態で制御する。
//! 覆いの外側を押す・Escape を押す等で閉じようとしたときは、閉じたら の応答を発行する
//! （egui 自身は閉じない。利用側が応答を状態へ適用して次のフレームで閉じる）。

use std::rc::Rc;

use crate::tree::{ノード, 子を順に描画する};

/// モーダル型とは、画面を覆って操作を独占する区画の記述のことである。
pub struct モーダル型<M> {
    識別子: String,
    開いている: bool,
    閉じたら指定: Option<M>,
    子一覧: Vec<ノード<M>>,
}

impl<M> モーダル型<M> {
    pub(crate) fn 新規(
        識別子: String, 開いている: bool, 子一覧: Vec<ノード<M>>
    ) -> Self {
        Self {
            識別子,
            開いている,
            閉じたら指定: None,
            子一覧,
        }
    }

    /// 外側を押す・Escape で閉じようとしたときに発行する応答を設定する。
    pub fn 閉じたら(mut self, 応答: M) -> Self {
        self.閉じたら指定 = Some(応答);
        self
    }
}

impl<M: Clone> モーダル型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        if !self.開いている {
            return ui.response();
        }
        let 結果 = egui::Modal::new(egui::Id::new(&self.識別子)).show(ui.ctx(), |内側| {
            子を順に描画する(&self.子一覧, 内側, 発行した応答)
        });
        if 結果.should_close()
            && let Some(応答) = &self.閉じたら指定
        {
            発行した応答.push(応答.clone());
        }
        結果.response
    }
}

impl<M: 'static> モーダル型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> モーダル型<N> {
        モーダル型 {
            識別子: self.識別子,
            開いている: self.開いている,
            閉じたら指定: self.閉じたら指定.map(&*応答を変換する),
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}
