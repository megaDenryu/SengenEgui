//! コンテナノード。子束（子ノード列）を持ち、自分の並べ方の中で順に再生する。

mod conditional;
mod hstack;
mod scroll_view;
mod vstack;

pub use conditional::条件ノード;
pub use hstack::横並びノード;
pub use scroll_view::縦スクロールノード;
pub use vstack::縦積みノード;

use crate::node::{ノード, ノード箱};

/// 子束とは、コンテナが保持する子ノード列のことである。各コンテナの共通部。
pub(crate) struct 子束 {
    一覧: Vec<ノード箱>,
}

impl 子束 {
    pub(crate) fn 空() -> Self {
        Self { 一覧: Vec::new() }
    }

    pub(crate) fn 置き換える(&mut self, 一覧: Vec<ノード箱>) {
        self.一覧 = 一覧;
    }

    pub(crate) fn 順に描画する(&mut self, ui: &mut egui::Ui) {
        for 子 in &mut self.一覧 {
            子.描画する(ui);
        }
    }
}
