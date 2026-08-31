//! 縦スクロール領域。スクロール位置は egui が識別子ごとに内部保持するため、
//! バインディングは持たない。識別子は同一画面内で重複させない。

use crate::{
    containers::子束,
    node::{ノード, ノード箱},
};

pub struct 縦スクロールノード {
    識別子: String,
    最大高さ: Option<f32>,
    末尾追従指定: bool,
    子束: 子束,
}

impl 縦スクロールノード {
    pub(crate) fn 新規(識別子: String) -> Self {
        Self {
            識別子,
            最大高さ: None,
            末尾追従指定: false,
            子束: 子束::空(),
        }
    }

    /// 子ノード列を設定する。`子!` マクロで組んだ列を渡す。
    pub fn 子(mut self, 一覧: Vec<ノード箱>) -> Self {
        self.子束.置き換える(一覧);
        self
    }

    pub fn 最大高さ(mut self, 高さ: f32) -> Self {
        self.最大高さ = Some(高さ);
        self
    }

    /// 末尾追従を有効にする。記録表示のように末尾へ追記される内容に使う。
    pub fn 末尾追従(mut self) -> Self {
        self.末尾追従指定 = true;
        self
    }
}

impl ノード for 縦スクロールノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let mut 領域 = egui::ScrollArea::vertical()
            .id_salt(self.識別子.clone())
            .auto_shrink(false)
            .stick_to_bottom(self.末尾追従指定);
        if let Some(高さ) = self.最大高さ {
            領域 = 領域.max_height(高さ);
        }
        let 子束 = &mut self.子束;
        領域.show(ui, |内側| 子束.順に描画する(内側));
    }
}
