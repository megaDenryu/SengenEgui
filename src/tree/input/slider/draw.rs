//! スライダーの描画と応答の判定。egui のスライダーを組んで描き、値が変わった応答と操作を終えた応答を決める。
//! 値が変わったかは、egui の変更の印に加えて値そのものを比べて決める。数値の欄の編集中は、値が変わらなくても
//! 文字の変更で egui が変更の印を付けるためである。

use super::track::軌道の横に並ぶもの;
use super::スライダー型;
use crate::tree::input::erased::描画の結果;

impl<M, 数: egui::emath::Numeric> スライダー型<M, 数> {
    pub(super) fn 描画する(&self, ui: &mut egui::Ui) -> 描画の結果<M> {
        let mut 値 = self.値;
        let mut 部品 =
            egui::Slider::new(&mut 値, self.範囲.clone()).show_value(self.数値を表示する);
        if let Some(文字) = &self.添える文字 {
            部品 = 部品.text(文字.clone());
        }
        if let Some(刻み) = self.刻み指定 {
            部品 = 部品.step_by(刻み.to_f64());
        }
        if self.操作を終えた値から応答を作る.is_some() {
            部品 = 部品.update_while_editing(false);
        }
        let 横に並ぶもの = 軌道の横に並ぶもの {
            数値を表示する: self.数値を表示する,
            添える文字: self.添える文字.as_deref(),
        };
        self.軌道の幅.間隔の設定へ書く(ui, 横に並ぶもの);
        let 反応 = ui.add(部品);
        let 値が変わった = 反応.changed() && 値 != self.値;
        let mut 発行する応答 = Vec::new();
        if 値が変わった {
            発行する応答.push((self.新しい値から応答を作る)(値));
        }
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
