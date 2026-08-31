//! 縦積みと横並び。子を並べる向きだけが違うので1つの型で持つ。

use crate::style::スタイル;
use crate::tree::ノード;

pub(crate) enum 積む向き {
    縦,
    横,
}

pub struct 積み型<M> {
    向き: 積む向き,
    子一覧: Vec<ノード<M>>,
    装飾値: スタイル,
}

impl<M> 積み型<M> {
    pub(crate) fn 新規(向き: 積む向き, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            向き,
            子一覧,
            装飾値: スタイル::無指定,
        }
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
        装飾.枠で包んで描画する(ui, |内側| match self.向き {
            積む向き::縦 => {
                内側.vertical(|内側| self.子を順に描画する(内側, 集配));
            }
            積む向き::横 => {
                内側.horizontal(|内側| self.子を順に描画する(内側, 集配));
            }
        });
    }

    fn 子を順に描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        for 子 in &self.子一覧 {
            子.描画する(ui, 集配);
        }
    }
}
