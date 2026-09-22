//! ドラッグ可能な浮きウィンドウ。開閉はアプリ状態で制御する（状態は外部に置く方針）。
//! 閉じたら を設定すると閉じるボタンが付き、押されたときに応答を発行する。
//! 位置と大きさは egui が表題ごとに内部保持し、既定の値は最初のフレームだけに効く。

use std::rc::Rc;

use crate::measure::縦横の論理画素;
use crate::tree::{ノード, 子を順に描画する};

/// ウィンドウ型とは、画面の上に浮かぶドラッグ可能な窓の記述のことである。
pub struct ウィンドウ型<M> {
    表題: String,
    開いている: bool,
    閉じたら指定: Option<M>,
    大きさを変えられる指定: bool,
    折り畳める指定: bool,
    既定の大きさ指定: Option<縦横の論理画素>,
    既定の位置指定: Option<縦横の論理画素>,
    子一覧: Vec<ノード<M>>,
}

impl<M> ウィンドウ型<M> {
    pub(crate) fn 新規(
        表題: String, 開いている: bool, 子一覧: Vec<ノード<M>>
    ) -> Self {
        Self {
            表題,
            開いている,
            閉じたら指定: None,
            大きさを変えられる指定: true,
            折り畳める指定: true,
            既定の大きさ指定: None,
            既定の位置指定: None,
            子一覧,
        }
    }

    /// 閉じるボタンを付け、押されたときに発行する応答を設定する。
    pub fn 閉じたら(mut self, 応答: M) -> Self {
        self.閉じたら指定 = Some(応答);
        self
    }

    /// 縁のドラッグで大きさを変えられるかを指定する。既定は変えられる。
    pub fn 大きさを変えられる(mut self, 変えられる: bool) -> Self {
        self.大きさを変えられる指定 = 変えられる;
        self
    }

    /// 表題の三角で中身を折り畳めるかを指定する。既定は折り畳める。
    pub fn 折り畳める(mut self, 折り畳める: bool) -> Self {
        self.折り畳める指定 = 折り畳める;
        self
    }

    /// 最初に開いたときの幅と高さを指定する。
    pub fn 既定の大きさ(mut self, 大きさ: 縦横の論理画素) -> Self {
        self.既定の大きさ指定 = Some(大きさ);
        self
    }

    /// 最初に開いたときの画面左上からの位置を指定する。
    pub fn 既定の位置(mut self, 位置: 縦横の論理画素) -> Self {
        self.既定の位置指定 = Some(位置);
        self
    }
}

impl<M: Clone> ウィンドウ型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        if !self.開いている {
            return ui.response();
        }
        let mut 開いたまま = true;
        let mut 窓 = egui::Window::new(self.表題.clone())
            .resizable(self.大きさを変えられる指定)
            .collapsible(self.折り畳める指定);
        if self.閉じたら指定.is_some() {
            窓 = 窓.open(&mut 開いたまま);
        }
        if let Some(大きさ) = self.既定の大きさ指定 {
            窓 = 窓.default_size(大きさ.eguiへ渡す値());
        }
        if let Some(位置) = self.既定の位置指定 {
            窓 = 窓.default_pos(位置.eguiの位置へ渡す値());
        }
        let 表示結果 = 窓.show(ui.ctx(), |内側| {
            子を順に描画する(&self.子一覧, 内側, 発行した応答)
        });
        if !開いたまま && let Some(応答) = &self.閉じたら指定 {
            発行した応答.push(応答.clone());
        }
        表示結果.map_or_else(|| ui.response(), |結果| 結果.response)
    }
}

impl<M: 'static> ウィンドウ型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> ウィンドウ型<N> {
        ウィンドウ型 {
            表題: self.表題,
            開いている: self.開いている,
            閉じたら指定: self.閉じたら指定.map(&*応答を変換する),
            大きさを変えられる指定: self.大きさを変えられる指定,
            折り畳める指定: self.折り畳める指定,
            既定の大きさ指定: self.既定の大きさ指定,
            既定の位置指定: self.既定の位置指定,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
        }
    }
}
