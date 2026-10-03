//! 帯の幅。値の全体の範囲を横の帯に写す部品（区間の帯・時間の行の並び）が、横に占める幅をどう決めるかを持つ。

use crate::measure::論理画素;

// 帯の幅とは、区間の帯と時間の行の並びが横に占める幅をどう決めるかの区別のことである。
#[derive(Clone, Copy)]
pub(in crate::tree::input) enum 帯の幅 {
    使える幅いっぱい,
    指定(論理画素),
}

impl 帯の幅 {
    // このフレームで横に占める幅。egui へ確保を頼む境界でだけ使う。
    pub(in crate::tree::input) fn 占める幅(self, ui: &egui::Ui) -> f32 {
        match self {
            Self::使える幅いっぱい => ui.available_width(),
            Self::指定(幅) => 幅.eguiへ渡す値(),
        }
    }
}
