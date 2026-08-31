//! 仮想化された縦のリスト。画面に見えている行だけを組んで描画するため、
//! 大量の行でも毎フレームの構築コストが表示行数に比例するだけで済む。
//! 行の高さは一定である前提を置く（egui の行位置計算がその前提を要求する）。

use crate::tree::ノード;

pub struct 仮想列型<M> {
    識別子: String,
    行高さ: f32,
    行数: usize,
    行を組む: Box<dyn Fn(usize) -> ノード<M>>,
    末尾追従指定: bool,
    最大高さ指定: Option<f32>,
}

impl<M> 仮想列型<M> {
    pub(crate) fn 新規(
        識別子: String,
        行高さ: f32,
        行数: usize,
        行を組む: Box<dyn Fn(usize) -> ノード<M>>,
    ) -> Self {
        Self {
            識別子,
            行高さ,
            行数,
            行を組む,
            末尾追従指定: false,
            最大高さ指定: None,
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

impl<M: Clone> 仮想列型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 領域 = egui::ScrollArea::vertical()
            .id_salt(self.識別子.clone())
            .auto_shrink(false)
            .stick_to_bottom(self.末尾追従指定);
        if let Some(高さ) = self.最大高さ指定 {
            領域 = 領域.max_height(高さ);
        }
        領域.show_rows(ui, self.行高さ, self.行数, |内側, 範囲| {
            for 行番号 in 範囲 {
                (self.行を組む)(行番号).描画する(内側, 集配);
            }
        });
    }
}

impl<M: 'static> 仮想列型<M> {
    pub(crate) fn 写す<N: 'static>(self, 変換: std::rc::Rc<dyn Fn(M) -> N>) -> 仮想列型<N> {
        let 元の行を組む = self.行を組む;
        仮想列型 {
            識別子: self.識別子,
            行高さ: self.行高さ,
            行数: self.行数,
            行を組む: Box::new(move |行番号| {
                元の行を組む(行番号).rcで写す(std::rc::Rc::clone(&変換))
            }),
            末尾追従指定: self.末尾追従指定,
            最大高さ指定: self.最大高さ指定,
        }
    }
}
