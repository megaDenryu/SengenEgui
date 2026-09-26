//! 重ねる位置。重ねる容器が子を寄せる、下地の矩形の中の9つの寄せ先を区別する。

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
    fn eguiの寄せへ変換する(self) -> egui::Align2 {
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

    /// 前の回に測った子の大きさを、範囲の中のこの寄せ先へ置いた矩形。測っていなければ大きさ0の矩形である。
    pub(super) fn 前回の大きさで置く矩形(
        self,
        ui: &egui::Ui,
        大きさの鍵: egui::Id,
        範囲: egui::Rect,
    ) -> egui::Rect {
        let 前回の大きさ = ui.data(|記憶域| 記憶域.get_temp::<egui::Vec2>(大きさの鍵));
        self.eguiの寄せへ変換する()
            .align_size_within_rect(前回の大きさ.unwrap_or(egui::Vec2::ZERO), 範囲)
    }
}
