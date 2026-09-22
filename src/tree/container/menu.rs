//! メニューバーとメニュー。メニューは押すと子の一覧を吹き出しで開くボタンであり、
//! メニューの中に置いたメニューは入れ子の小メニューになる。
//! メニューの中の部品が応答を発行したら、そのフレームでメニューを閉じる。

use std::rc::Rc;

use crate::tree::{ノード, 子を順に描画する};

/// メニューバー型とは、メニューを横に並べる帯の記述のことである。
pub struct メニューバー型<M> {
    子一覧: Vec<ノード<M>>,
}

impl<M> メニューバー型<M> {
    pub(crate) fn 新規(子一覧: Vec<ノード<M>>) -> Self {
        Self { 子一覧 }
    }
}

impl<M: Clone> メニューバー型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        egui::MenuBar::new()
            .ui(ui, |内側| {
                子を順に描画する(&self.子一覧, 内側, 発行した応答)
            })
            .response
    }
}

impl<M: 'static> メニューバー型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> メニューバー型<N> {
        メニューバー型 {
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}

/// メニュー型とは、押すと子の一覧を吹き出しで開くボタンの記述のことである。
pub struct メニュー型<M> {
    表示文字列: String,
    子一覧: Vec<ノード<M>>,
}

impl<M> メニュー型<M> {
    pub(crate) fn 新規(表示文字列: String, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            表示文字列, 子一覧
        }
    }
}

impl<M: Clone> メニュー型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        ui.menu_button(self.表示文字列.clone(), |内側| {
            let 開く前の件数 = 発行した応答.len();
            子を順に描画する(&self.子一覧, 内側, 発行した応答);
            if 発行した応答.len() > 開く前の件数 {
                内側.close();
            }
        })
        .response
    }
}

impl<M: 'static> メニュー型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> メニュー型<N> {
        メニュー型 {
            表示文字列: self.表示文字列,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}
