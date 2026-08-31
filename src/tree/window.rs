//! ドラッグ可能な浮きウィンドウ。開閉はアプリ状態で制御する（状態は外部に置く方針）。
//! 閉じたら を設定すると閉じるボタンが付き、押されたときに応答を発行する。
//! 位置と大きさは egui が表題ごとに内部保持する。

use crate::tree::{ノード, 子を順に描画する};

pub struct ウィンドウ型<M> {
    表題: String,
    開いている: bool,
    閉じたら指定: Option<M>,
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
            子一覧,
        }
    }

    /// 閉じるボタンを付け、押されたときに発行する応答を設定する。
    pub fn 閉じたら(mut self, 応答: M) -> Self {
        self.閉じたら指定 = Some(応答);
        self
    }
}

impl<M: Clone> ウィンドウ型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        if !self.開いている {
            return;
        }
        let mut 開いたまま = true;
        let mut 窓 = egui::Window::new(self.表題.clone());
        if self.閉じたら指定.is_some() {
            窓 = 窓.open(&mut 開いたまま);
        }
        窓.show(ui.ctx(), |内側| {
            子を順に描画する(&self.子一覧, 内側, 集配)
        });
        if !開いたまま && let Some(応答) = &self.閉じたら指定 {
            集配.push(応答.clone());
        }
    }
}

impl<M: 'static> ウィンドウ型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        変換: std::rc::Rc<dyn Fn(M) -> N>,
    ) -> ウィンドウ型<N> {
        ウィンドウ型 {
            表題: self.表題,
            開いている: self.開いている,
            閉じたら指定: self.閉じたら指定.map(&*変換),
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &変換),
        }
    }
}
