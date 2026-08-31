//! 縦積み。子を上から下へ並べる。

use crate::{
    containers::子束,
    node::{ノード, ノード箱},
    style::スタイル,
};

pub struct 縦積みノード {
    子束: 子束,
    装飾値: スタイル,
}

impl 縦積みノード {
    pub(crate) fn 新規() -> Self {
        Self {
            子束: 子束::空(),
            装飾値: スタイル::無指定,
        }
    }

    /// 子ノード列を設定する。`子!` マクロで組んだ列を渡す。
    pub fn 子(mut self, 一覧: Vec<ノード箱>) -> Self {
        self.子束.置き換える(一覧);
        self
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }
}

impl ノード for 縦積みノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let 装飾 = self.装飾値;
        let 子束 = &mut self.子束;
        装飾.枠で包んで描画する(ui, |内側| {
            内側.vertical(|内側| 子束.順に描画する(内側));
        });
    }
}
