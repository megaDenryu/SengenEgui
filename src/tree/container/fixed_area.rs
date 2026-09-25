//! 大きさを決めた領域。指定の幅と高さをちょうど占め、その中へ子を上から並べる。
//! 子が小さくても領域の大きさは変わらないため、映像の欄の全体に覆いやボタンを重ねるときの下地に使う。
//! 子が領域からはみ出す分は領域で切り取る。

use std::rc::Rc;

use crate::measure::縦横の論理画素;
use crate::style::スタイル;
use crate::tree::{ノード, 子を順に描画する};

/// 大きさを決めた領域型とは、占める幅と高さと、その中に並べる子の一覧の記述のことである。
pub struct 大きさを決めた領域型<M> {
    寸法: 縦横の論理画素,
    子一覧: Vec<ノード<M>>,
    中央寄せ指定: bool,
    装飾値: スタイル,
}

impl<M> 大きさを決めた領域型<M> {
    pub(crate) fn 新規(寸法: 縦横の論理画素, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            寸法,
            子一覧,
            中央寄せ指定: false,
            装飾値: スタイル::無指定,
        }
    }

    /// 子を左右の中央へ寄せる。上下の中央へ寄せるには、子の高さから計算した余白を先頭に置く。
    pub fn 中央寄せ(mut self) -> Self {
        self.中央寄せ指定 = true;
        self
    }

    /// 装飾を適用する。使う項目は背景色だけであり、領域の全体を塗る。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl<M: Clone> 大きさを決めた領域型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let (矩形, 反応) = ui.allocate_exact_size(self.寸法.eguiへ渡す値(), egui::Sense::hover());
        if let Some(色) = self.装飾値.背景色 {
            ui.painter().rect_filled(矩形, 0.0, 色);
        }
        let 寄せ = if self.中央寄せ指定 {
            egui::Align::Center
        } else {
            egui::Align::Min
        };
        let 作り = egui::UiBuilder::new()
            .max_rect(矩形)
            .layout(egui::Layout::top_down(寄せ));
        let mut 子の領域 = ui.new_child(作り);
        子の領域.set_clip_rect(矩形.intersect(ui.clip_rect()));
        子を順に描画する(&self.子一覧, &mut 子の領域, 発行した応答);
        反応
    }
}

impl<M: 'static> 大きさを決めた領域型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 大きさを決めた領域型<N> {
        大きさを決めた領域型 {
            寸法: self.寸法,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
            中央寄せ指定: self.中央寄せ指定,
            装飾値: self.装飾値,
        }
    }
}
