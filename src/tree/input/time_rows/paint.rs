//! 時間の行の並びの描き方。時間の軸の段の地と、行ごとの地（塊の無い行も描く）と、塊と、位置の線を、この順に重ねて描く。
//! 塊の描き方は `paint_block` に書く。位置の線は全体の範囲の外の値なら端に描く。色は、装飾の背景色を行の地へ、
//! 文字色を位置の線へ当て、指定の無い項目をテーマの見た目から取る。

use super::hit::画面上の塊の並び;
use super::layout::時間の行の並びの配置;
use super::paint_block::時間の塊の描き方;
use super::時間の塊の鍵;
use crate::measure::{帯の値, 画素, 論理画素};
use crate::style::スタイル;

const 行の地の上下の空き: 論理画素 = 画素(1.0);
const 位置の線の太さ: 論理画素 = 画素(2.0);
const 位置の線の頭の半幅: 論理画素 = 画素(5.0);

/// 時間の行の並びの描き方とは、軸と行の地の色と、塊の描き方と、位置の線の色の組のことである。
pub(super) struct 時間の行の並びの描き方 {
    軸の地の色: egui::Color32,
    行の地の色: egui::Color32,
    塊の描き方: 時間の塊の描き方,
    位置の線の色: egui::Color32,
}

impl 時間の行の並びの描き方 {
    pub(super) fn 装飾と見た目から作る(
        装飾: スタイル, 見た目: &egui::Visuals
    ) -> Self {
        Self {
            軸の地の色: 見た目.faint_bg_color,
            行の地の色: 装飾.背景色.unwrap_or(見た目.extreme_bg_color),
            塊の描き方: 時間の塊の描き方::装飾と見た目から作る(装飾, 見た目),
            位置の線の色: 装飾.文字色.unwrap_or(見た目.strong_text_color()),
        }
    }

    pub(super) fn 描く<鍵: 時間の塊の鍵>(
        &self,
        描き手: &egui::Painter,
        配置: &時間の行の並びの配置,
        塊の並び: &画面上の塊の並び<'_, 鍵>,
        位置: Option<帯の値>,
    ) {
        描き手.rect_filled(配置.軸の段, 0.0, self.軸の地の色);
        let 地の空き = egui::vec2(0.0, 行の地の上下の空き.eguiへ渡す値());
        for 行の矩形 in 配置.行の矩形を巡る() {
            描き手.rect_filled(行の矩形.shrink2(地の空き), 0.0, self.行の地の色);
        }
        self.塊の描き方.描く(描き手, 配置, 塊の並び);
        if let Some(位置) = 位置 {
            self.位置の線を描く(描き手, 配置, 位置);
        }
    }

    fn 位置の線を描く(
        &self,
        描き手: &egui::Painter,
        配置: &時間の行の並びの配置,
        位置: 帯の値,
    ) {
        let 横 = 配置.値の横の位置(位置);
        let 軸 = 配置.軸の段;
        let 線 = egui::Stroke::new(位置の線の太さ.eguiへ渡す値(), self.位置の線の色);
        描き手.vline(横, 軸.top()..=配置.行の並びの矩形.bottom(), 線);
        let 半幅 = 位置の線の頭の半幅.eguiへ渡す値();
        let 頭 = vec![
            egui::pos2(横 - 半幅, 軸.top()),
            egui::pos2(横 + 半幅, 軸.top()),
            egui::pos2(横, 軸.bottom()),
        ];
        描き手.add(egui::Shape::convex_polygon(
            頭,
            self.位置の線の色,
            egui::Stroke::NONE,
        ));
    }
}
