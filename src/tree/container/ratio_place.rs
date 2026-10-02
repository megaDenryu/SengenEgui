//! 割合の矩形へ置く容器。指定の幅と高さをちょうど占め、子を、容器の矩形に対する割合で表した矩形へ、書いた順に重ねて置く。
//! 子の位置と大きさは子の中身でなく矩形だけで決まるため、重ねる容器と違って大きさを測る回が無く、子は最初に出した回から見えて押せる。
//! 子にはその矩形を使える範囲として渡し、はみ出して描く分は矩形で切り取る。
//! egui は後に登録した部品を上として押下とドラッグを配るため、後に置いた子ほど上に重なり、重なった場所では先に押される。

use std::rc::Rc;

use crate::measure::{割合で表した矩形, 縦横の論理画素};
use crate::style::スタイル;
use crate::tree::ノード;

/// 割合の矩形へ置く型とは、占める幅と高さと、子とその子を置く割合で表した矩形の組を書いた順に並べたものの記述のことである。
pub struct 割合の矩形へ置く型<M> {
    寸法: 縦横の論理画素,
    子一覧: Vec<(割合で表した矩形, ノード<M>)>,
    装飾値: スタイル,
}

impl<M> 割合の矩形へ置く型<M> {
    pub(crate) fn 新規(寸法: 縦横の論理画素) -> Self {
        Self {
            寸法,
            子一覧: Vec::new(),
            装飾値: スタイル::無指定,
        }
    }

    /// 子を、容器の幅と高さを1とした割合で表した矩形へ置く。後に置いた子ほど上に重なる。
    /// 子が使える幅と高さは矩形の幅と高さであり、子の中で `使える大きさから組む` を使えば矩形いっぱいに描ける。
    pub fn 子を置く(
        mut self, 矩形: 割合で表した矩形, 子: impl Into<ノード<M>>
    ) -> Self {
        self.子一覧.push((矩形, 子.into()));
        self
    }

    /// 装飾を適用する。使う項目は背景色だけであり、子より先に容器の全体を塗る。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl<M: Clone> 割合の矩形へ置く型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let (容器の矩形, 反応) =
            ui.allocate_exact_size(self.寸法.eguiへ渡す値(), egui::Sense::hover());
        if let Some(色) = self.装飾値.背景色 {
            ui.painter().rect_filled(容器の矩形, 0.0, 色);
        }
        for (矩形, 子) in &self.子一覧 {
            let 子の矩形 = 矩形.画面の矩形へ写す(容器の矩形);
            let 作り = egui::UiBuilder::new()
                .max_rect(子の矩形)
                .layout(egui::Layout::top_down(egui::Align::Min));
            let mut 子の領域 = ui.new_child(作り);
            子の領域.set_clip_rect(子の矩形.intersect(ui.clip_rect()));
            子.描画する(&mut 子の領域, 発行した応答);
        }
        反応
    }
}

impl<M: 'static> 割合の矩形へ置く型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 割合の矩形へ置く型<N> {
        割合の矩形へ置く型 {
            寸法: self.寸法,
            子一覧: self
                .子一覧
                .into_iter()
                .map(|(矩形, 子)| (矩形, 子.rcで写す(Rc::clone(&応答を変換する))))
                .collect(),
            装飾値: self.装飾値,
        }
    }
}
