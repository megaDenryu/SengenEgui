//! 数値入力。ドラッグと直接入力で数を変える。数の型は egui の `Numeric` を満たす型で汎用である。
//! ノードへ変換する時点で数の型を閉包へ閉じ込める（`erased` 参照）。

use std::ops::RangeInclusive;

use crate::theme::選ばれた項目と範囲選択の色;
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
    操作を終えた値から応答を作る: Option<Box<dyn Fn(数) -> M>>,
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
            操作を終えた値から応答を作る: None,
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

    /// 操作を終えたとき（ドラッグを放したとき、またはドラッグ以外の操作で値が変わったとき）に、その時点の値から作った応答を発する。
    /// ドラッグ以外の操作とは、直接入力の確定（Enter かフォーカスが外れたとき）と、フォーカスを持つ間の矢印キーである。
    /// この指定があると、直接入力は打鍵のたびでなく確定したときだけ値を変える。値が変わるたびの応答も今までどおり発し、
    /// 同じ回に両方が出るときは変わった応答が先に並ぶ。利用する側は、ドラッグの間の値の変化を1つの操作にまとめる区切りに使える。
    pub fn 操作を終えたら発する(
        mut self,
        値から応答を作る: impl Fn(数) -> M + 'static,
    ) -> Self {
        self.操作を終えた値から応答を作る = Some(Box::new(値から応答を作る));
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
        if self.操作を終えた値から応答を作る.is_some() {
            部品 = 部品.update_while_editing(false);
        }
        let 反応 = 選ばれた項目と範囲選択の色::一時記憶から読む(ui)
            .範囲選択の地を差し替えて描く(ui, 部品);
        // 直接入力の編集中は、値が変わらなくても文字の変更で egui が変更の印を付けるため、値そのものも比べる。
        let 値が変わった = 反応.changed() && 値 != self.値;
        let mut 発行する応答: Vec<M> = 値が変わった
            .then(|| (self.新しい値から応答を作る)(値))
            .into_iter()
            .collect();
        if let Some(作る) = &self.操作を終えた値から応答を作る
            && (反応.drag_stopped() || (値が変わった && !反応.dragged()))
        {
            発行する応答.push(作る(値));
        }
        描画の結果 {
            発行する応答, 反応
        }
    }
}

impl<M: 'static, 数: egui::emath::Numeric> From<数値入力型<M, 数>> for ノード<M> {
    fn from(値: 数値入力型<M, 数>) -> Self {
        let 消去済み = 型を消した入力型::新規(Box::new(move |ui| 値.描画する(ui)));
        Self::入力(入力の部品::数値入力(消去済み))
    }
}
