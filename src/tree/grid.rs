//! 格子。子を左から右へ、指定した列数で折り返して並べる。表形式の設定画面等に使う。

use crate::tree::ノード;

pub struct 格子型<M> {
    識別子: String,
    列数: usize,
    子一覧: Vec<ノード<M>>,
}

impl<M> 格子型<M> {
    pub(crate) fn 新規(識別子: String, 列数: usize, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            識別子,
            列数: 列数.max(1),
            子一覧,
        }
    }
}

impl<M: Clone> 格子型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        egui::Grid::new(self.識別子.clone())
            .num_columns(self.列数)
            .show(ui, |内側| {
                for (位置, 子) in self.子一覧.iter().enumerate() {
                    子.描画する(内側, 集配);
                    if (位置 + 1) % self.列数 == 0 {
                        内側.end_row();
                    }
                }
            });
    }
}

impl<M: 'static> 格子型<M> {
    pub(crate) fn 写す<N: 'static>(self, 変換: std::rc::Rc<dyn Fn(M) -> N>) -> 格子型<N> {
        格子型 {
            識別子: self.識別子,
            列数: self.列数,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &変換),
        }
    }
}
