//! 数値入力。ドラッグと直接入力で数を変える。数の型は egui の `Numeric` を満たす型で汎用である。
//! ノードへ変換する時点で数の型を閉包へ閉じ込める（`erased` 参照）。

use std::ops::RangeInclusive;

use crate::theme::強調色まわりの色;
use crate::tree::input::erased::{型を消した入力型, 描画の結果};
use crate::tree::input::入力の部品;
use crate::tree::ノード;

/// 数値入力型とは、範囲内の数をドラッグと直接入力で編集する部品の記述のことである。
pub struct 数値入力型<M, 数: egui::emath::Numeric> {
    値: 数,
    範囲: RangeInclusive<数>,
    新しい値から応答を作る: Box<dyn Fn(数) -> M>,
    刻み指定: Option<数>,
    前置き指定: Option<String>,
    後置き指定: Option<String>,
    小数点以下の桁数指定: Option<usize>,
}

impl<M, 数: egui::emath::Numeric> 数値入力型<M, 数> {
    pub(crate) fn 新規(
        値: 数,
        範囲: RangeInclusive<数>,
        新しい値から応答を作る: Box<dyn Fn(数) -> M>,
    ) -> Self {
        Self {
            値,
            範囲,
            新しい値から応答を作る,
            刻み指定: None,
            前置き指定: None,
            後置き指定: None,
            小数点以下の桁数指定: None,
        }
    }

    /// 1論理画素のドラッグで変わる量を指定する。
    pub fn 刻み(mut self, 刻み: 数) -> Self {
        self.刻み指定 = Some(刻み);
        self
    }

    /// 数の前に添える文字を指定する。
    pub fn 前置き(mut self, 文字: impl Into<String>) -> Self {
        self.前置き指定 = Some(文字.into());
        self
    }

    /// 数の後に添える文字（単位等）を指定する。
    pub fn 後置き(mut self, 文字: impl Into<String>) -> Self {
        self.後置き指定 = Some(文字.into());
        self
    }

    /// 小数点以下を常にこの桁数で表示する。秒を 1.50 のように揃えて見せるときに使う。
    pub fn 小数点以下の桁数(mut self, 桁数: usize) -> Self {
        self.小数点以下の桁数指定 = Some(桁数);
        self
    }

    fn 描画する(&self, ui: &mut egui::Ui) -> 描画の結果<M> {
        let mut 値 = self.値;
        let mut 部品 = egui::DragValue::new(&mut 値).range(self.範囲.clone());
        if let Some(刻み) = self.刻み指定 {
            部品 = 部品.speed(刻み.to_f64());
        }
        if let Some(文字) = &self.前置き指定 {
            部品 = 部品.prefix(文字);
        }
        if let Some(文字) = &self.後置き指定 {
            部品 = 部品.suffix(文字);
        }
        if let Some(桁数) = self.小数点以下の桁数指定 {
            部品 = 部品.fixed_decimals(桁数);
        }
        let 反応 = 強調色まわりの色::読む(ui).範囲選択の地を差し替えて描く(ui, 部品);
        描画の結果 {
            発行する応答: 反応
                .changed()
                .then(|| (self.新しい値から応答を作る)(値))
                .into_iter()
                .collect(),
            反応,
        }
    }
}

impl<M: 'static, 数: egui::emath::Numeric> From<数値入力型<M, 数>> for ノード<M> {
    fn from(値: 数値入力型<M, 数>) -> Self {
        let 消去済み = 型を消した入力型::新規(Box::new(move |ui| 値.描画する(ui)));
        Self::入力(入力の部品::数値入力(消去済み))
    }
}
