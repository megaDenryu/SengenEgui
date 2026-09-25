//! 描き直しの予約。画面に何も描かず、指定の時間が経ったらもう一度描画させる。
//!
//! egui は入力が無ければ描き直さない。動画の再生のように入力が無くても画面を進めたい間だけ、
//! この部品を木へ置く（`条件付き表示(再生中, …)` の形）。木から外せば予約も止まる。
//! 描くたびに予約し直すため、置いている間は待つ時間ごとに描き直しが続く。

use std::time::Duration;

/// 描き直しの予約型とは、次の描き直しまでに待つ時間の記述のことである。
pub struct 描き直しの予約型 {
    待つ時間: Duration,
}

impl 描き直しの予約型 {
    pub(crate) fn 新規(待つ時間: Duration) -> Self {
        Self { 待つ時間 }
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.ctx().request_repaint_after(self.待つ時間);
        ui.response()
    }
}
