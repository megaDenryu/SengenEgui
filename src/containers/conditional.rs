//! 動的な条件分岐。両側を構築時に作っておき、描画のたびに判定で選ぶ。
//! 構造は永続のままで表示だけが切り替わる。構築時に決まる条件には空ノードを使う。

use crate::node::{ノード, ノード箱};

pub struct 条件ノード {
    判定: Box<dyn Fn() -> bool>,
    真側: ノード箱,
    偽側: Option<ノード箱>,
}

impl 条件ノード {
    pub(crate) fn 新規(
        判定: Box<dyn Fn() -> bool>,
        真側: ノード箱,
        偽側: Option<ノード箱>,
    ) -> Self {
        Self {
            判定, 真側, 偽側
        }
    }
}

impl ノード for 条件ノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        if (self.判定)() {
            self.真側.描画する(ui);
        } else if let Some(偽側) = self.偽側.as_mut() {
            偽側.描画する(ui);
        }
    }
}
