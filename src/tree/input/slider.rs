//! スライダー。範囲内の数をつまみで変える。数の型は egui の `Numeric` を満たす型で汎用である。
//! ノードへ変換する時点で数の型を閉包へ閉じ込める（`erased` 参照）。

use std::ops::RangeInclusive;

use crate::measure::論理画素;
use crate::tree::input::erased::{型を消した入力型, 描画の結果};
use crate::tree::input::入力の部品;
use crate::tree::ノード;

/// スライダー型とは、範囲内の数をつまみで編集する部品の記述のことである。
pub struct スライダー型<M, 数: egui::emath::Numeric> {
    値: 数,
    範囲: RangeInclusive<数>,
    新しい値から応答を作る: Box<dyn Fn(数) -> M>,
    添える文字: Option<String>,
    幅指定: Option<論理画素>,
}

impl<M, 数: egui::emath::Numeric> スライダー型<M, 数> {
    pub(crate) fn 新規(
        値: 数,
        範囲: RangeInclusive<数>,
        新しい値から応答を作る: Box<dyn Fn(数) -> M>,
    ) -> Self {
        Self {
            値,
            範囲,
            新しい値から応答を作る,
            添える文字: None,
            幅指定: None,
        }
    }

    /// つまみの右に添える文字を指定する。
    pub fn 文字を添える(mut self, 文字: impl Into<String>) -> Self {
        self.添える文字 = Some(文字.into());
        self
    }

    /// つまみが動く軌道の幅を指定する。
    pub fn 幅(mut self, 幅: 論理画素) -> Self {
        self.幅指定 = Some(幅);
        self
    }

    fn 描画する(&self, ui: &mut egui::Ui) -> 描画の結果<M> {
        let mut 値 = self.値;
        let mut 部品 = egui::Slider::new(&mut 値, self.範囲.clone());
        if let Some(文字) = &self.添える文字 {
            部品 = 部品.text(文字.clone());
        }
        if let Some(幅) = self.幅指定 {
            ui.spacing_mut().slider_width = 幅.eguiへ渡す値();
        }
        let 反応 = ui.add(部品);
        描画の結果 {
            発行する応答: 反応
                .changed()
                .then(|| (self.新しい値から応答を作る)(値)),
            反応,
        }
    }
}

impl<M: 'static, 数: egui::emath::Numeric> From<スライダー型<M, 数>> for ノード<M> {
    fn from(値: スライダー型<M, 数>) -> Self {
        let 消去済み =
            型を消した入力型::新規(
                Box::new(move |ui| ui.scope(|内側| 値.描画する(内側)).inner),
            );
        Self::入力(入力の部品::スライダー(消去済み))
    }
}
