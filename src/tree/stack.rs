//! 縦積みと横並び。子を並べる向きだけが違うので1つの型で持つ。

use crate::style::スタイル;
use crate::tree::{ノード, 子を順に描画する};

pub(crate) enum 積む向き {
    縦,
    横,
}

pub struct 積み型<M> {
    向き: 積む向き,
    子一覧: Vec<ノード<M>>,
    装飾値: スタイル,
    中央寄せ指定: bool,
}

impl<M> 積み型<M> {
    pub(crate) fn 新規(向き: 積む向き, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            向き,
            子一覧,
            装飾値: スタイル::無指定,
            中央寄せ指定: false,
        }
    }

    /// 縦積みでは子を左右中央へ、横並びでは子を上下中央へ寄せる。
    pub fn 中央寄せ(mut self) -> Self {
        self.中央寄せ指定 = true;
        self
    }

    /// 装飾を適用する。装飾は名前付き定数として構造の外で定義する（lib.rs 方針3）。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl<M: Clone> 積み型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let 装飾 = self.装飾値;
        装飾.枠で包んで描画する(ui, |内側| match (&self.向き, self.中央寄せ指定) {
            (積む向き::縦, false) => {
                内側.vertical(|内側| 子を順に描画する(&self.子一覧, 内側, 集配));
            }
            (積む向き::縦, true) => {
                内側.vertical_centered(|内側| 子を順に描画する(&self.子一覧, 内側, 集配));
            }
            (積む向き::横, false) => {
                内側.horizontal(|内側| 子を順に描画する(&self.子一覧, 内側, 集配));
            }
            (積む向き::横, true) => {
                内側.horizontal_centered(|内側| 子を順に描画する(&self.子一覧, 内側, 集配));
            }
        });
    }
}

impl<M: 'static> 積み型<M> {
    pub(crate) fn 写す<N: 'static>(self, 変換: std::rc::Rc<dyn Fn(M) -> N>) -> 積み型<N> {
        積み型 {
            向き: self.向き,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &変換),
            装飾値: self.装飾値,
            中央寄せ指定: self.中央寄せ指定,
        }
    }
}
