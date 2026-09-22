//! スクロール領域。縦・横・両方向を選べる。スクロール位置は egui が識別子ごとに内部保持する。
//! 識別子は同一画面内で重複させない。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::{ノード, 子を順に描画する};

/// スクロールの向きとは、領域がどの方向へ転がせるかの区別のことである。
#[derive(Clone, Copy)]
pub enum スクロールの向き {
    /// 上下にだけ転がせる。
    縦,
    /// 左右にだけ転がせる。
    横,
    /// 上下と左右の両方に転がせる。
    両方向,
}

/// スクロール型とは、内容が収まらないときに転がせる領域の記述のことである。
pub struct スクロール型<M> {
    識別子: String,
    向き: スクロールの向き,
    最大幅指定: Option<論理画素>,
    最大高さ指定: Option<論理画素>,
    末尾追従指定: bool,
    子一覧: Vec<ノード<M>>,
}

impl<M> スクロール型<M> {
    pub(crate) fn 新規(
        識別子: String, 向き: スクロールの向き, 子一覧: Vec<ノード<M>>
    ) -> Self {
        Self {
            識別子,
            向き,
            最大幅指定: None,
            最大高さ指定: None,
            末尾追従指定: false,
            子一覧,
        }
    }

    /// 領域の幅の上限を指定する。
    pub fn 最大幅(mut self, 幅: 論理画素) -> Self {
        self.最大幅指定 = Some(幅);
        self
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

impl<M: Clone> スクロール型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let mut 領域 = match self.向き {
            スクロールの向き::縦 => egui::ScrollArea::vertical(),
            スクロールの向き::横 => egui::ScrollArea::horizontal(),
            スクロールの向き::両方向 => egui::ScrollArea::both(),
        }
        .id_salt(self.識別子.clone())
        .auto_shrink(false)
        .stick_to_bottom(self.末尾追従指定);
        if let Some(幅) = self.最大幅指定 {
            領域 = 領域.max_width(幅.eguiへ渡す値());
        }
        if let Some(高さ) = self.最大高さ指定 {
            領域 = 領域.max_height(高さ.eguiへ渡す値());
        }
        ui.scope(|内側| {
            領域.show(内側, |内側| {
                子を順に描画する(&self.子一覧, 内側, 発行した応答)
            });
        })
        .response
    }
}

impl<M: 'static> スクロール型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> スクロール型<N> {
        スクロール型 {
            識別子: self.識別子,
            向き: self.向き,
            最大幅指定: self.最大幅指定,
            最大高さ指定: self.最大高さ指定,
            末尾追従指定: self.末尾追従指定,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}
