//! 重ねる位置と置き方と、重ねる子を寄せ先へ描く手順。重ねる容器が、子を下地のどこへ置くかを決める。
//! 覆う置き方は、覆いを塗って下地への押下を受け止めることもここで行う。
//!
//! 重ねる子の大きさは描いてみるまで分からないため、前の回に描いた大きさを egui の一時記憶に置き、
//! その大きさで寄せ先を決める。記憶が無い回（鍵ごとに最初に描く1回）だけは見えない状態で大きさを測り、描き直しを求める。
//! 記憶は木から外した後も egui に残るため、2回目以降は出した回から見えて押せる。
//! 子には寄せ先に関わらず範囲と同じ幅と高さを使わせる。幅いっぱいに広がる子も初めて測る回に範囲の幅で測られ、
//! 次の回には正しい寄せ先に収まる。はみ出して描く分は範囲で切り取る。

use crate::measure::論理画素;
use crate::tree::container::overlay_idle::ポインタが止まると隠す指定;
use crate::tree::ノード;

/// 重ねる位置とは、下地の矩形の中の9つの寄せ先の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    /// 下地の上にマウスがある間だけ、`寄せて置く` と同じ位置へ置く。
    マウスを乗せている間だけ寄せて置く(重ねる位置),
    /// 下地の上でポインタが動いてから一定時間だけ、`寄せて置く` と同じ位置へ置く。
    ポインタが止まると隠して寄せて置く(重ねる位置, ポインタが止まると隠す指定),
    /// 下地の全体を色で塗って覆い、真ん中へ置く。覆いは下地への押下を受け止め、下地へ届かせない。
    覆って中央に置く(egui::Color32),
}

impl 重ねる置き方 {
    /// 重ねる子の大きさを記憶する鍵の区分。寄せる位置と、覆う置き方かの組である。
    /// マウスを乗せている間だけ置く子と、ポインタが止まると隠す子は、寄せて置く子と同じ区分に入る。
    pub(super) fn 記憶の区分(self) -> (重ねる位置, bool) {
        match self {
            Self::寄せて置く(位置)
            | Self::マウスを乗せている間だけ寄せて置く(位置)
            | Self::ポインタが止まると隠して寄せて置く(位置, _) => (位置, false),
            Self::覆って中央に置く(_) => (重ねる位置::中央, true),
        }
    }

    /// 置き方から寄せ先と範囲を決めて子を描く。マウスを乗せている間だけ置く子は、マウスが下地の外にあれば描かない。
    /// ポインタが止まると隠す子は、出し入れの規則（`overlay_idle`）が隠すと決めた回は描かない。
    pub(super) fn 子を描く<M: Clone>(
        self,
        ui: &mut egui::Ui,
        鍵: egui::Id,
        下地の矩形: egui::Rect,
        端からの間隔: 論理画素,
        子: &ノード<M>,
        発行した応答: &mut Vec<M>,
    ) {
        let 端から空けた範囲 = 下地の矩形.shrink(端からの間隔.eguiへ渡す値());
        let (位置, 範囲) = match self {
            Self::寄せて置く(位置) => (位置, 端から空けた範囲),
            Self::マウスを乗せている間だけ寄せて置く(位置) => {
                if !ui.rect_contains_pointer(下地の矩形) {
                    return;
                }
                (位置, 端から空けた範囲)
            }
            Self::ポインタが止まると隠して寄せて置く(位置, 指定) => {
                let 子の矩形 = 置く矩形を求める(ui, 鍵, 位置, 端から空けた範囲);
                if !指定.出すかを決める(ui, 鍵, 下地の矩形, 子の矩形) {
                    return;
                }
                (位置, 端から空けた範囲)
            }
            Self::覆って中央に置く(色) => {
                ui.painter().rect_filled(下地の矩形, 0.0, 色);
                let 覆いの感じ方 = egui::Sense::click_and_drag();
                let _ = ui.interact(下地の矩形, 鍵.with("覆い"), 覆いの感じ方);
                (重ねる位置::中央, 下地の矩形)
            }
        };
        寄せて描く(ui, 鍵, 位置, 範囲, 子, 発行した応答);
    }
}

/// 前の回に測った子の大きさで寄せ先の矩形を決めて子を描き、今回の大きさを記憶する。
/// 親の並びの位置を進めないよう、親に場所を取らせない子の領域へ描く。
fn 寄せて描く<M: Clone>(
    ui: &mut egui::Ui,
    鍵: egui::Id,
    位置: 重ねる位置,
    範囲: egui::Rect,
    子: &ノード<M>,
    発行した応答: &mut Vec<M>,
) {
    let 前回の大きさ = ui.data(|記憶域| 記憶域.get_temp::<egui::Vec2>(鍵));
    let 置く矩形 = 置く矩形を求める(ui, 鍵, 位置, 範囲);
    let mut 作り = egui::UiBuilder::new()
        .max_rect(egui::Rect::from_min_size(置く矩形.min, 範囲.size()))
        .layout(egui::Layout::top_down(egui::Align::Min));
    if 前回の大きさ.is_none() {
        作り = 作り.sizing_pass().invisible();
    }
    let mut 子の領域 = ui.new_child(作り);
    子の領域.set_clip_rect(範囲.intersect(ui.clip_rect()));
    子.描画する(&mut 子の領域, 発行した応答);
    let 今回の大きさ = 子の領域.min_rect().size();
    if 前回の大きさ != Some(今回の大きさ) {
        ui.data_mut(|記憶域| 記憶域.insert_temp(鍵, 今回の大きさ));
        ui.ctx().request_repaint();
    }
}

/// 前の回に測った子の大きさを範囲の中の寄せ先へ置いた矩形。測っていなければ大きさ0の矩形である。
fn 置く矩形を求める(
    ui: &egui::Ui,
    鍵: egui::Id,
    位置: 重ねる位置,
    範囲: egui::Rect,
) -> egui::Rect {
    let 前回の大きさ = ui.data(|記憶域| 記憶域.get_temp::<egui::Vec2>(鍵));
    位置
        .eguiの寄せへ変換する()
        .align_size_within_rect(前回の大きさ.unwrap_or(egui::Vec2::ZERO), 範囲)
}
