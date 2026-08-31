//! チェックボックス。値の読みと書きを別クロージャで渡す双方向バインディング。

use crate::{node::ノード, source::文字列源, style::スタイル};

pub struct 切り替えノード {
    表示: 文字列源,
    読み: Box<dyn Fn() -> bool>,
    書き: Box<dyn FnMut(bool)>,
    装飾値: スタイル,
}

impl 切り替えノード {
    pub(crate) fn 新規(
        表示: 文字列源,
        読み: Box<dyn Fn() -> bool>,
        書き: Box<dyn FnMut(bool)>,
    ) -> Self {
        Self {
            表示,
            読み,
            書き,
            装飾値: スタイル::無指定,
        }
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl ノード for 切り替えノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let mut 値 = (self.読み)();
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.表示.現在値()));
        if ui.checkbox(&mut 値, 文字).changed() {
            (self.書き)(値);
        }
    }
}
