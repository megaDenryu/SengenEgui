//! 範囲枠の描き方。枠の外側を暗くし、枠線と4隅のつまみを描く。
//! 枠線とつまみの色は、装飾の枠線色があればそれを、無ければテーマの選択の縁の色を使う。
//! テーマは選択の縁を強調色から導くため、テーマを適用した画面では強調色に揃う。

use crate::measure::{画素, 論理画素};
use crate::style::スタイル;

/// 枠の外側に重ねる半透明の黒。明暗のどちらの基調でも「外側は切り捨てる側」と読めるよう黒に固定する。
const 枠の外を暗くする色: egui::Color32 = egui::Color32::from_black_alpha(128);
const 既定の枠線の太さ: 論理画素 = 画素(2.0);
const つまみの一辺: 論理画素 = 画素(10.0);

/// 範囲枠の描き方とは、枠線とつまみを描くときに使う線と色の組のことである。
pub(super) struct 範囲枠の描き方 {
    枠線: egui::Stroke,
    つまみの色: egui::Color32,
}

impl 範囲枠の描き方 {
    /// 装飾の指定を優先し、指定の無い項目をテーマの見た目から埋めて描き方を作る。
    pub(super) fn 装飾と見た目から作る(
        装飾: スタイル, 見た目: &egui::Visuals
    ) -> Self {
        let 色 = 装飾.枠線色.unwrap_or(見た目.selection.stroke.color);
        let 太さ = 装飾.枠線太さ.unwrap_or(既定の枠線の太さ);
        Self {
            枠線: egui::Stroke::new(太さ.eguiへ渡す値(), 色),
            つまみの色: 色,
        }
    }

    /// 画像の矩形の上に、枠の外側の暗がり・枠線・4隅のつまみを描く。
    pub(super) fn 描く(
        &self,
        描き手: &egui::Painter,
        画像の矩形: egui::Rect,
        枠の矩形: egui::Rect,
    ) {
        for 暗くする矩形 in 枠の外側の矩形一覧(画像の矩形, 枠の矩形) {
            描き手.rect_filled(暗くする矩形, 0.0, 枠の外を暗くする色);
        }
        描き手.rect_stroke(枠の矩形, 0.0, self.枠線, egui::StrokeKind::Middle);
        let 一辺 = つまみの一辺.eguiへ渡す値();
        for 隅 in [
            枠の矩形.left_top(),
            枠の矩形.right_top(),
            枠の矩形.left_bottom(),
            枠の矩形.right_bottom(),
        ] {
            let つまみ = egui::Rect::from_center_size(隅, egui::vec2(一辺, 一辺));
            描き手.rect_filled(つまみ, 0.0, self.つまみの色);
        }
    }
}

/// 画像の矩形から枠の矩形を除いた領域を、重ならない4つの矩形（上・下・左・右）に分ける。
fn 枠の外側の矩形一覧(
    画像の矩形: egui::Rect, 枠の矩形: egui::Rect
) -> [egui::Rect; 4] {
    let 上 = egui::Rect::from_min_max(
        画像の矩形.left_top(),
        egui::pos2(画像の矩形.right(), 枠の矩形.top()),
    );
    let 下 = egui::Rect::from_min_max(
        egui::pos2(画像の矩形.left(), 枠の矩形.bottom()),
        画像の矩形.right_bottom(),
    );
    let 左 = egui::Rect::from_min_max(
        egui::pos2(画像の矩形.left(), 枠の矩形.top()),
        egui::pos2(枠の矩形.left(), 枠の矩形.bottom()),
    );
    let 右 = egui::Rect::from_min_max(
        egui::pos2(枠の矩形.right(), 枠の矩形.top()),
        egui::pos2(画像の矩形.right(), 枠の矩形.bottom()),
    );
    [上, 下, 左, 右]
}
