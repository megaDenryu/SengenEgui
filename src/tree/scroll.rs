//! 縦スクロール領域。スクロール位置は egui が識別子ごとに内部保持する。
//! 識別子は同一画面内で重複させない。

use crate::tree::ノード;

pub struct スクロール型<M> {
    識別子: String,
    最大高さ指定: Option<f32>,
    末尾追従指定: bool,
    子一覧: Vec<ノード<M>>,
}

impl<M> スクロール型<M> {
    pub(crate) fn 新規(識別子: String, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            識別子,
            最大高さ指定: None,
            末尾追従指定: false,
            子一覧,
        }
    }

    pub fn 最大高さ(mut self, 高さ: f32) -> Self {
        self.最大高さ指定 = Some(高さ);
        self
    }

    /// 末尾追従を有効にする。記録表示のように末尾へ追記される内容に使う。
    pub fn 末尾追従(mut self) -> Self {
        self.末尾追従指定 = true;
        self
    }
}

impl<M: Clone> スクロール型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 領域 = egui::ScrollArea::vertical()
            .id_salt(self.識別子.clone())
            .auto_shrink(false)
            .stick_to_bottom(self.末尾追従指定);
        if let Some(高さ) = self.最大高さ指定 {
            領域 = 領域.max_height(高さ);
        }
        領域.show(ui, |内側| {
            for 子 in &self.子一覧 {
                子.描画する(内側, 集配);
            }
        });
    }
}
