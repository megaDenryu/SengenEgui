//! 文字表示。静的な文字列と、クロージャで取りに行く動的な文字列の両方を扱う。

use crate::{node::ノード, source::文字列源, style::スタイル};

pub struct 文章ノード {
    源: 文字列源,
    装飾値: スタイル,
}

impl 文章ノード {
    pub(crate) fn 新規(源: 文字列源) -> Self {
        Self {
            源,
            装飾値: スタイル::無指定,
        }
    }

    /// 装飾を適用する。装飾は名前付き定数として構造の外で定義する（lib.rs 方針3）。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl ノード for 文章ノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.源.現在値()));
        ui.label(文字);
    }
}
