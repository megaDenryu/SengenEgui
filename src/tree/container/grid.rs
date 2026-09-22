//! 格子。子を左から右へ、指定した列数で折り返して並べる。表形式の設定画面等に使う。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::ノード;

/// 格子型とは、子を列数で折り返して表の形に並べる容器の記述のことである。
pub struct 格子型<M> {
    識別子: String,
    列数: usize,
    子一覧: Vec<ノード<M>>,
    縞模様指定: bool,
    列の間隔指定: Option<論理画素>,
    行の間隔指定: Option<論理画素>,
}

impl<M> 格子型<M> {
    pub(crate) fn 新規(識別子: String, 列数: usize, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            識別子,
            列数: 列数.max(1),
            子一覧,
            縞模様指定: false,
            列の間隔指定: None,
            行の間隔指定: None,
        }
    }

    /// 1行おきに背景を淡く塗り、行を目で追いやすくする。
    pub fn 縞模様(mut self) -> Self {
        self.縞模様指定 = true;
        self
    }

    /// 列と列の間の距離を指定する。
    pub fn 列の間隔(mut self, 間隔: 論理画素) -> Self {
        self.列の間隔指定 = Some(間隔);
        self
    }

    /// 行と行の間の距離を指定する。
    pub fn 行の間隔(mut self, 間隔: 論理画素) -> Self {
        self.行の間隔指定 = Some(間隔);
        self
    }
}

impl<M: Clone> 格子型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 既定の間隔 = ui.spacing().item_spacing;
        let 間隔 = egui::vec2(
            self.列の間隔指定
                .map_or(既定の間隔.x, 論理画素::eguiへ渡す値),
            self.行の間隔指定
                .map_or(既定の間隔.y, 論理画素::eguiへ渡す値),
        );
        egui::Grid::new(self.識別子.clone())
            .num_columns(self.列数)
            .striped(self.縞模様指定)
            .spacing(間隔)
            .show(ui, |内側| {
                for (位置, 子) in self.子一覧.iter().enumerate() {
                    子.描画する(内側, 発行した応答);
                    if (位置 + 1) % self.列数 == 0 {
                        内側.end_row();
                    }
                }
            })
            .response
    }
}

impl<M: 'static> 格子型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 格子型<N> {
        格子型 {
            識別子: self.識別子,
            列数: self.列数,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
            縞模様指定: self.縞模様指定,
            列の間隔指定: self.列の間隔指定,
            行の間隔指定: self.行の間隔指定,
        }
    }
}
