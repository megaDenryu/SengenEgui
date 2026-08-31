//! 折り畳み見出し。開閉状態は egui が識別子ごとに内部保持する
//! （スクロール位置と同じ扱い。アプリ状態で制御したい場合は条件ノードを使う）。

use crate::tree::{ノード, 子を順に描画する};

pub struct 折り畳み型<M> {
    見出し: String,
    識別子指定: Option<String>,
    既定で開く指定: bool,
    子一覧: Vec<ノード<M>>,
}

impl<M> 折り畳み型<M> {
    pub(crate) fn 新規(見出し: String, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            見出し,
            識別子指定: None,
            既定で開く指定: false,
            子一覧,
        }
    }

    /// 同じ見出しの折り畳みが同一画面に並ぶときに、開閉状態を区別する鍵を与える。
    pub fn 識別子(mut self, 識別子: impl Into<String>) -> Self {
        self.識別子指定 = Some(識別子.into());
        self
    }

    pub fn 既定で開く(mut self) -> Self {
        self.既定で開く指定 = true;
        self
    }
}

impl<M: Clone> 折り畳み型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 見出し =
            egui::CollapsingHeader::new(self.見出し.clone()).default_open(self.既定で開く指定);
        if let Some(識別子) = &self.識別子指定 {
            見出し = 見出し.id_salt(識別子.clone());
        }
        見出し.show(ui, |内側| {
            子を順に描画する(&self.子一覧, 内側, 集配)
        });
    }
}

impl<M: 'static> 折り畳み型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        変換: std::rc::Rc<dyn Fn(M) -> N>,
    ) -> 折り畳み型<N> {
        折り畳み型 {
            見出し: self.見出し,
            識別子指定: self.識別子指定,
            既定で開く指定: self.既定で開く指定,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &変換),
        }
    }
}
