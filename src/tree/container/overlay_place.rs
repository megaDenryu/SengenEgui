//! 重ねる位置と置き方と、重ねる子を寄せ先へ描く手順。重ねる容器が、子を下地のどこへ置くかを決める。
//!
//! 重ねる子の大きさは描いてみるまで分からないため、前の回に描いた大きさを egui の一時記憶に置き、
//! その大きさで寄せ先を決める。初めて描く回は見えない状態で大きさだけを測り、描き直しを求める。
//! そのため重ねる子を押せるのは、初めて描いた回の次の回からである。

use crate::tree::ノード;

/// 重ねる位置とは、下地の矩形の中の9つの寄せ先の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 重ねる位置 {
    /// 左上の隅。
    左上,
    /// 上の辺の中央。
    上,
    /// 右上の隅。
    右上,
    /// 左の辺の中央。
    左,
    /// 真ん中。
    中央,
    /// 右の辺の中央。
    右,
    /// 左下の隅。
    左下,
    /// 下の辺の中央。
    下,
    /// 右下の隅。
    右下,
}

impl 重ねる位置 {
    pub(super) fn eguiの寄せへ変換する(self) -> egui::Align2 {
        match self {
            Self::左上 => egui::Align2::LEFT_TOP,
            Self::上 => egui::Align2::CENTER_TOP,
            Self::右上 => egui::Align2::RIGHT_TOP,
            Self::左 => egui::Align2::LEFT_CENTER,
            Self::中央 => egui::Align2::CENTER_CENTER,
            Self::右 => egui::Align2::RIGHT_CENTER,
            Self::左下 => egui::Align2::LEFT_BOTTOM,
            Self::下 => egui::Align2::CENTER_BOTTOM,
            Self::右下 => egui::Align2::RIGHT_BOTTOM,
        }
    }
}

/// 重ねる置き方とは、子を下地の上へどう置くかの区別のことである。
#[derive(Clone, Copy)]
pub(super) enum 重ねる置き方 {
    /// 下地の端から間隔を空けた範囲の中の、指定の位置へ寄せて置く。
    寄せて置く(重ねる位置),
    /// 下地の全体を色で塗って覆い、真ん中へ置く。覆いは下地への押下を受け止め、下地へ届かせない。
    覆って中央に置く(egui::Color32),
}

/// 前の回に測った子の大きさで寄せ先の矩形を決めて子を描き、今回の大きさを記憶する。
/// 親の並びの位置を進めないよう、親に場所を取らせない子の領域へ描く。
pub(super) fn 寄せて描く<M: Clone>(
    ui: &mut egui::Ui,
    鍵: egui::Id,
    位置: 重ねる位置,
    範囲: egui::Rect,
    子: &ノード<M>,
    発行した応答: &mut Vec<M>,
) {
    let 前回の大きさ = ui.data(|記憶域| 記憶域.get_temp::<egui::Vec2>(鍵));
    let 置く矩形 = 位置
        .eguiの寄せへ変換する()
        .align_size_within_rect(前回の大きさ.unwrap_or(egui::Vec2::ZERO), 範囲);
    let mut 作り = egui::UiBuilder::new()
        .max_rect(egui::Rect::from_min_max(置く矩形.min, 範囲.max))
        .layout(egui::Layout::top_down(egui::Align::Min));
    if 前回の大きさ.is_none() {
        作り = 作り.sizing_pass().invisible();
    }
    let mut 子の領域 = ui.new_child(作り);
    子.描画する(&mut 子の領域, 発行した応答);
    let 今回の大きさ = 子の領域.min_rect().size();
    if 前回の大きさ != Some(今回の大きさ) {
        ui.data_mut(|記憶域| 記憶域.insert_temp(鍵, 今回の大きさ));
        ui.ctx().request_repaint();
    }
}
