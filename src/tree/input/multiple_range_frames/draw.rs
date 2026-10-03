//! 複数の範囲枠の描画と操作の読み取りの順。下地を描いてから、基準の矩形の周りの余白を含む全体で押下とドラッグを受け取る
//! 反応を作る。egui は後に登録した部品を上として押下を配るため、反応を下地の後に作り、下地の中の部品より先に押されるようにする。
//! 枠は反応の後に描いて下地の上に重ね、操作は枠ごとの応答を作る関数へ渡す。

use super::drag::複数の範囲枠のドラッグ;
use super::hit::{画面上の枠, 画面上の枠の並び};
use super::paint::複数の範囲枠の描き方;
use super::{
    つまみの当たりの半径, 複数の範囲枠の周りの余白, 複数の範囲枠型
};

impl<M: Clone> 複数の範囲枠型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 余白 = egui::Vec2::splat(複数の範囲枠の周りの余白.eguiへ渡す値());
        let 大きさ = self.寸法.eguiへ渡す値() + 余白 * 2.0;
        let (確保した矩形, 確保した領域の応答) =
            ui.allocate_exact_size(大きさ, egui::Sense::hover());
        let 基準の矩形 = 確保した矩形.shrink2(余白);
        self.下地を描く(ui, 基準の矩形, 発行した応答);
        let 反応 = ui.interact(
            確保した矩形,
            確保した領域の応答.id.with("複数の範囲枠"),
            egui::Sense::drag(),
        );
        let 枠の並び = self.画面上の枠の並びを作る(基準の矩形);
        複数の範囲枠の描き方::装飾と見た目から作る(self.装飾値, ui.visuals())
            .描く(ui.painter(), &枠の並び);
        let ドラッグ = 複数の範囲枠のドラッグ {
            反応: &反応,
            基準の矩形,
            枠の並び: &枠の並び,
            縦横比の指定: self.縦横比の指定,
            つまみの当たりの半径,
        };
        ドラッグ.カーソルを変える(ui);
        for (番号, 操作) in ドラッグ.操作を読み取る(ui) {
            if let Some(枠) = self.枠一覧.get(番号.並びの位置()) {
                発行した応答.push((枠.操作から応答を作る)(操作));
            }
        }
        反応
    }

    /// 下地を基準の矩形へ描く。はみ出して描く分は基準の矩形で切り取る。
    fn 下地を描く(
        &self,
        ui: &mut egui::Ui,
        基準の矩形: egui::Rect,
        発行した応答: &mut Vec<M>,
    ) {
        let 下地の領域の設定 = egui::UiBuilder::new()
            .max_rect(基準の矩形)
            .layout(egui::Layout::top_down(egui::Align::Min));
        let mut 下地の領域 = ui.new_child(下地の領域の設定);
        下地の領域.set_clip_rect(基準の矩形.intersect(ui.clip_rect()));
        self.下地.描画する(&mut 下地の領域, 発行した応答);
    }

    fn 画面上の枠の並びを作る(
        &self,
        基準の矩形: egui::Rect,
    ) -> 画面上の枠の並び {
        画面上の枠の並び::置いた順の一覧から作る(
            self.枠一覧
                .iter()
                .map(|枠| 画面上の枠 {
                    矩形: 枠.矩形.画面の矩形へ写す(基準の矩形),
                    選び: 枠.選び,
                })
                .collect(),
        )
    }
}
