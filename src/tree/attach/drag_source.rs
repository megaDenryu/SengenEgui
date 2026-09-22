//! ドラッグ元。子をドラッグで運び出せるようにし、運ぶ値を egui の運搬記憶へ置く。
//! 運ぶ値の型はノードへ変換する時点で閉包へ閉じ込め、ノードの列挙は型引数 M だけを持つ。
//! ドラッグ中は egui が子の見た目をポインタの位置へ複写して描く。

use std::any::Any;
use std::rc::Rc;

use crate::tree::ノード;

type 運びながら描画する手続き =
    Rc<dyn Fn(&mut egui::Ui, egui::Id, &mut dyn FnMut(&mut egui::Ui)) -> egui::Response>;

/// ドラッグ元型とは、子と、ドラッグで運ぶ値の組の記述のことである。
pub struct ドラッグ元型<M> {
    識別子: String,
    子: ノード<M>,
    運びながら描画する: 運びながら描画する手続き,
}

impl<M> ドラッグ元型<M> {
    pub(crate) fn 新規<運ぶ値: Clone + Any + Send + Sync>(
        識別子: String,
        運ぶ値: 運ぶ値,
        子: ノード<M>,
    ) -> Self {
        Self {
            識別子,
            子,
            運びながら描画する: Rc::new(move |ui, 鍵, 中身を描く| {
                ui.dnd_drag_source(鍵, 運ぶ値.clone(), |内側| 中身を描く(内側))
                    .response
            }),
        }
    }
}

impl<M: Clone> ドラッグ元型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 鍵 = ui.make_persistent_id(&self.識別子);
        (self.運びながら描画する)(ui, 鍵, &mut |内側| {
            self.子.描画する(内側, 発行した応答);
        })
    }
}

impl<M: 'static> ドラッグ元型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> ドラッグ元型<N> {
        ドラッグ元型 {
            識別子: self.識別子,
            子: self.子.rcで写す(応答を変換する),
            運びながら描画する: self.運びながら描画する,
        }
    }
}
