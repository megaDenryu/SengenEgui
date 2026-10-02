//! モーダル。画面全体を薄く覆い、その上に子を置く。開閉はアプリ状態で制御する。
//! 覆いの外側を押す・Escape を押す等で閉じようとしたときは、閉じたら の応答を発行する
//! （egui 自身は閉じない。利用側が応答を状態へ適用して次のフレームで閉じる）。
//! `画面に収める` を指定すると、中身の幅を画面の幅に抑え、画面の高さに収まらない中身を縦にスクロールさせる。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::{ノード, 子を順に描画する};

/// モーダル型とは、画面を覆って操作を独占する区画の記述のことである。
pub struct モーダル型<M> {
    識別子: String,
    開いている: bool,
    閉じたら指定: Option<M>,
    画面の端との間: Option<論理画素>,
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
            画面の端との間: None,
            子一覧,
        }
    }

    /// 外側を押す・Escape で閉じようとしたときに発行する応答を設定する。
    pub fn 閉じたら(mut self, 応答: M) -> Self {
        self.閉じたら指定 = Some(応答);
        self
    }

    /// モーダルの枠を、画面の4辺から指定の間を空けた範囲に収める。中身の幅はその範囲の幅を上限にし(文字は折り返す)、
    /// 中身の高さがその範囲の高さを超えるときは、中身を縦にスクロールさせる。行の多い設定の画面を小さなウインドウで開くときに使う。
    pub fn 画面に収める(mut self, 画面の端との間: 論理画素) -> Self {
        self.画面の端との間 = Some(画面の端との間);
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
        let 結果 = egui::Modal::new(egui::Id::new(&self.識別子)).show(ui.ctx(), |内側| match self
            .画面の端との間
        {
            None => 子を順に描画する(&self.子一覧, 内側, 発行した応答),
            Some(間) => self.画面に収めて描画する(内側, 間, 発行した応答),
        });
        if 結果.should_close()
            && let Some(応答) = &self.閉じたら指定
        {
            発行した応答.push(応答.clone());
        }
        結果.response
    }

    /// 中身の幅を画面に収まる幅までに抑え、中身を縦のスクロール領域に入れて描く。
    /// 収まる大きさは、画面の大きさから4辺の間と、モーダルの枠(egui の既定の `Frame::popup`)の余白と線を引いたものである。
    fn 画面に収めて描画する(
        &self,
        内側: &mut egui::Ui,
        間: 論理画素,
        発行した応答: &mut Vec<M>,
    ) {
        let 枠の余白 = egui::Frame::popup(内側.style()).total_margin().sum();
        let 収まる大きさ = (内側.ctx().screen_rect().size()
            - egui::Vec2::splat(2.0 * 間.eguiへ渡す値())
            - 枠の余白)
            .max(egui::Vec2::ZERO);
        内側.set_max_width(内側.max_rect().width().min(収まる大きさ.x));
        egui::ScrollArea::vertical()
            .id_salt(format!("{}の中身", self.識別子))
            .max_height(収まる大きさ.y)
            .auto_shrink([true, true])
            .show(内側, |内側| {
                子を順に描画する(&self.子一覧, 内側, 発行した応答)
            });
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
            画面の端との間: self.画面の端との間,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}
