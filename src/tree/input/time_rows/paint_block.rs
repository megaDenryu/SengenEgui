//! 時間の塊の描き方。選んでいない塊を先に、選んでいる塊を後に描き、掴む順（選んでいる塊が先）と見た目の重なりを揃える。
//! 塊は行の並びの矩形で切り取り、全体の範囲からはみ出した部分を描かない。塊に付けた等しい間隔の区切りには細い縦の線を引く。
//! 色は、装飾の枠線色と枠線太さを選んでいる塊の枠線へ当て、指定の無い項目をテーマの見た目から取る。

use super::block::時間の塊;
use super::hit::画面上の塊の並び;
use super::layout::時間の行の並びの配置;
use super::row::塊の選択状態;
use super::時間の塊の鍵;
use crate::measure::{画素, 論理画素};
use crate::style::スタイル;

const 選んでいる塊の既定の枠線の太さ: 論理画素 = 画素(2.0);
const 区切りの線の太さ: 論理画素 = 画素(1.0);
const 角の丸み: 論理画素 = 画素(3.0);

// 時間の塊の描き方とは、選んでいない塊と選んでいる塊の塗りと枠線と、区切りの線の組のことである。
pub(super) struct 時間の塊の描き方 {
    塊の色: egui::Color32,
    塊の枠線: egui::Stroke,
    選んでいる塊の色: egui::Color32,
    選んでいる塊の枠線: egui::Stroke,
    区切りの線: egui::Stroke,
}

impl 時間の塊の描き方 {
    pub(super) fn 装飾と見た目から作る(
        装飾: スタイル, 見た目: &egui::Visuals
    ) -> Self {
        let 選んでいる枠線の太さ = 装飾.枠線太さ.unwrap_or(選んでいる塊の既定の枠線の太さ);
        let 選んでいる枠線の色 = 装飾.枠線色.unwrap_or(見た目.selection.stroke.color);
        let 区切りの色 = 見た目.strong_text_color().gamma_multiply(0.5);
        Self {
            塊の色: 見た目.widgets.inactive.bg_fill,
            塊の枠線: 見た目.widgets.noninteractive.bg_stroke,
            選んでいる塊の色: 見た目.selection.bg_fill,
            選んでいる塊の枠線: egui::Stroke::new(
                選んでいる枠線の太さ.eguiへ渡す値(),
                選んでいる枠線の色,
            ),
            区切りの線: egui::Stroke::new(区切りの線の太さ.eguiへ渡す値(), 区切りの色),
        }
    }

    // 選んでいない塊を置いた順に描き、その上に選んでいる塊を置いた順に描く。
    pub(super) fn 描く<鍵: 時間の塊の鍵>(
        &self,
        描き手: &egui::Painter,
        配置: &時間の行の並びの配置,
        塊の並び: &画面上の塊の並び<'_, 鍵>,
    ) {
        let 塊の描き手 = 描き手.with_clip_rect(配置.行の並びの矩形.intersect(描き手.clip_rect()));
        for 選択状態 in [塊の選択状態::選んでいない, 塊の選択状態::選んでいる]
        {
            for 塊 in 塊の並び
                .置いた順に巡る()
                .filter(|塊| 塊.置いた塊.選択状態 == 選択状態)
            {
                self.一つの塊を描く(
                    &塊の描き手,
                    配置,
                    塊.描く矩形,
                    塊.置いた塊.塊,
                    選択状態,
                );
            }
        }
    }

    fn 一つの塊を描く(
        &self,
        描き手: &egui::Painter,
        配置: &時間の行の並びの配置,
        矩形: egui::Rect,
        塊: 時間の塊,
        選択状態: 塊の選択状態,
    ) {
        let (色, 枠線) = match 選択状態 {
            塊の選択状態::選んでいない => (self.塊の色, self.塊の枠線),
            塊の選択状態::選んでいる => (self.選んでいる塊の色, self.選んでいる塊の枠線),
        };
        let 丸み = 角の丸み.eguiへ渡す値();
        描き手.rect_filled(矩形, 丸み, 色);
        for 境目 in 塊.区切りの値を巡る() {
            描き手.vline(
                配置
                    .目盛り
                    .値から範囲へ収めずに横の位置を求める(境目)
                    .eguiへ渡す値(),
                矩形.y_range(),
                self.区切りの線,
            );
        }
        描き手.rect_stroke(矩形, 丸み, 枠線, egui::StrokeKind::Inside);
    }
}
