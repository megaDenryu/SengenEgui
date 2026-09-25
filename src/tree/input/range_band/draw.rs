//! 区間の帯の描画。場所を確保して帯を描き、カーソルの形を変え、このフレームの操作を読み取って応答へ変える。
//! 描いてから操作を読み取るため、ドラッグで変わった値は次のフレームの描画に出る（範囲枠付き画像と同じ）。

use super::drag::区間の帯のドラッグ;
use super::layout::{区間の帯の配置, 帯の段の高さ, 頭の段の高さ};
use super::paint::区間の帯の描き方;
use super::{区間の帯型, 帯の幅};

impl<M> 区間の帯型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 幅 = match self.幅 {
            帯の幅::使える幅いっぱい => ui.available_width(),
            帯の幅::指定(幅) => 幅.eguiへ渡す値(),
        };
        let 高さ = (頭の段の高さ + 帯の段の高さ).eguiへ渡す値();
        let (矩形, 反応) = ui.allocate_exact_size(egui::vec2(幅, 高さ), egui::Sense::drag());
        let 配置 = 区間の帯の配置::確保した矩形を分ける(矩形, self.全体);
        区間の帯の描き方::装飾と見た目から作る(self.装飾値, ui.visuals()).描く(
            ui.painter(),
            &配置,
            self.今の値,
        );
        if let Some(乗っている位置) = 反応.hover_pos()
            && let Some(始め) = self.今の値.掴み始めを判定する(&配置, 乗っている位置)
        {
            ui.ctx().set_cursor_icon(始め.掴んだもの.カーソルの形());
        }
        let ドラッグ = 区間の帯のドラッグ {
            反応: &反応,
            配置,
            今の値: self.今の値,
        };
        発行した応答.extend(
            ドラッグ
                .操作を読み取る(ui)
                .into_iter()
                .map(&self.操作から応答を作る),
        );
        反応
    }
}
