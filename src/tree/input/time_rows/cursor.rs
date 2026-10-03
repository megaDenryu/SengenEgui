//! 時間の行の並びのカーソルの形。掴んでいる間は掴んだものの形に、それ以外はポインタが乗っている所の形にして、
//! 押す前に何を掴むかを見せる。塊は掴む形（掴んでいる間は握った形）、時間の軸は横に動かす形である。

use super::drag::時間の行の並びのドラッグの読み取り手;
use super::hold::時間の行の並びを掴んでいる間の記憶;
use super::press::押した所;
use super::時間の塊の鍵;

impl<鍵: 時間の塊の鍵> 時間の行の並びのドラッグの読み取り手<'_, 鍵> {
    // カーソルの形を、掴んでいる間は掴んだものに、それ以外はポインタが乗っている所に合わせる。
    pub(super) fn カーソルを変える(&self, ui: &egui::Ui) {
        let 掴んでいるものの形 = self.記憶の置き場().読む(ui).map(|記憶| match 記憶 {
            時間の行の並びを掴んでいる間の記憶::塊(_) => {
                egui::CursorIcon::Grabbing
            }
            時間の行の並びを掴んでいる間の記憶::位置の線(_) => {
                egui::CursorIcon::ResizeHorizontal
            }
        });
        let 乗っている所の形 =
            self.反応
                .hover_pos()
                .and_then(|座標| match self.押した所を判定する(座標) {
                    押した所::塊 { .. } => Some(egui::CursorIcon::Grab),
                    押した所::時間の軸 => Some(egui::CursorIcon::ResizeHorizontal),
                    押した所::何も無い所 { .. } => None,
                });
        if let Some(形) = 掴んでいるものの形.or(乗っている所の形) {
            ui.ctx().set_cursor_icon(形);
        }
    }
}
