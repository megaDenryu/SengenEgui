//! 区間の帯の描き方。帯の段に全体の地と区間の塗りと開始・終了のつまみを、上の段から帯の段の下端まで位置の印を描く。
//! 色は、装飾の指定があればそれを、無ければテーマの見た目から取る。区間の塗りはテーマの選択の地の色（強調色）である。
//! 開始が終了を超えた値を受け取ったときは区間を塗らず、2つのつまみだけを描く。部品は値を入れ替えて見せない。
//! 入れ替えて塗ると、利用する側の値が壊れていることが画面から分からなくなるためである。

use super::hit::帯に描く値;
use super::layout::区間の帯の配置;
use crate::measure::{画素, 論理画素};
use crate::style::スタイル;

const つまみの幅: 論理画素 = 画素(6.0);
const 位置の印の頭の半幅: 論理画素 = 画素(5.0);
const 位置の印の線の太さ: 論理画素 = 画素(2.0);
const 角の丸み: f32 = 2.0;

/// 区間の帯の描き方とは、帯の地・区間・つまみ・位置の印の色の組のことである。
pub(super) struct 区間の帯の描き方 {
    地の色: egui::Color32,
    区間の色: egui::Color32,
    つまみの色: egui::Color32,
    位置の印の色: egui::Color32,
}

impl 区間の帯の描き方 {
    /// 装飾の背景色を帯の地へ、枠線色をつまみへ、文字色を位置の印へ当て、指定の無い項目をテーマの見た目から埋める。
    pub(super) fn 装飾と見た目から作る(
        装飾: スタイル, 見た目: &egui::Visuals
    ) -> Self {
        Self {
            地の色: 装飾.背景色.unwrap_or(見た目.extreme_bg_color),
            区間の色: 見た目.selection.bg_fill,
            つまみの色: 装飾.枠線色.unwrap_or(見た目.strong_text_color()),
            位置の印の色: 装飾.文字色.unwrap_or(見た目.strong_text_color()),
        }
    }

    pub(super) fn 描く(
        &self, 描き手: &egui::Painter, 配置: &区間の帯の配置, 値: 帯に描く値
    ) {
        let 帯 = 配置.帯の段;
        描き手.rect_filled(帯, 角の丸み, self.地の色);
        let (開始の横, 終了の横) = (配置.値の横の位置(値.開始), 配置.値の横の位置(値.終了));
        if 値.開始 <= 値.終了 {
            let 区間 = egui::Rect::from_x_y_ranges(開始の横..=終了の横, 帯.y_range());
            描き手.rect_filled(区間, 0.0, self.区間の色);
        }
        for 横 in [開始の横, 終了の横] {
            let 大きさ = egui::vec2(つまみの幅.eguiへ渡す値(), 帯.height());
            let つまみ = egui::Rect::from_center_size(egui::pos2(横, 帯.center().y), 大きさ);
            描き手.rect_filled(つまみ, 角の丸み, self.つまみの色);
        }
        if let Some(位置) = 値.位置 {
            self.位置の印を描く(描き手, 配置, 配置.値の横の位置(位置));
        }
    }

    fn 位置の印を描く(
        &self, 描き手: &egui::Painter, 配置: &区間の帯の配置, 横: f32
    ) {
        let 頭 = 配置.頭の段;
        let 線 = egui::Stroke::new(位置の印の線の太さ.eguiへ渡す値(), self.位置の印の色);
        描き手.vline(横, 頭.bottom()..=配置.帯の段.bottom(), 線);
        let 半幅 = 位置の印の頭の半幅.eguiへ渡す値();
        let 三角 = vec![
            egui::pos2(横 - 半幅, 頭.top()),
            egui::pos2(横 + 半幅, 頭.top()),
            egui::pos2(横, 頭.bottom()),
        ];
        描き手.add(egui::Shape::convex_polygon(
            三角,
            self.位置の印の色,
            egui::Stroke::NONE,
        ));
    }
}
