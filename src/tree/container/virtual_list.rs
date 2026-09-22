//! 仮想化された縦のリスト。画面に見えている行だけを組んで描画するため、
//! 大量の行でも毎フレームの構築コストが表示行数に比例するだけで済む。
//! 行の高さは一定である前提を置く（egui の行位置計算がその前提を要求する）。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::ノード;

/// 仮想列型とは、見えている行だけを行番号から組んで描画する縦のリストの記述のことである。
pub struct 仮想列型<M> {
    識別子: String,
    行高さ: 論理画素,
    行数: usize,
    行を組む: Box<dyn Fn(usize) -> ノード<M>>,
    末尾追従指定: bool,
    最大高さ指定: Option<論理画素>,
}

impl<M> 仮想列型<M> {
    pub(crate) fn 新規(
        識別子: String,
        行高さ: 論理画素,
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

    /// 領域の高さの上限を指定する。
    pub fn 最大高さ(mut self, 高さ: 論理画素) -> Self {
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
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let mut 領域 = egui::ScrollArea::vertical()
            .id_salt(self.識別子.clone())
            .auto_shrink(false)
            .stick_to_bottom(self.末尾追従指定);
        if let Some(高さ) = self.最大高さ指定 {
            領域 = 領域.max_height(高さ.eguiへ渡す値());
        }
        ui.scope(|内側| {
            領域.show_rows(
                内側,
                self.行高さ.eguiへ渡す値(),
                self.行数,
                |内側, 範囲| {
                    for 行番号 in 範囲 {
                        (self.行を組む)(行番号).描画する(内側, 発行した応答);
                    }
                },
            );
        })
        .response
    }
}

impl<M: 'static> 仮想列型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 仮想列型<N> {
        let 元の行を組む = self.行を組む;
        仮想列型 {
            識別子: self.識別子,
            行高さ: self.行高さ,
            行数: self.行数,
            行を組む: Box::new(move |行番号| {
                元の行を組む(行番号).rcで写す(Rc::clone(&応答を変換する))
            }),
            末尾追従指定: self.末尾追従指定,
            最大高さ指定: self.最大高さ指定,
        }
    }
}
