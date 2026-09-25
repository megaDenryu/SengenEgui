//! 明暗。テーマの基調として、画面全体を濃色にするか淡色にするかを決める。

/// 明暗とは、画面全体の基調の区別のことである。
#[derive(Clone, Copy)]
pub enum 明暗 {
    /// 暗い地に明るい文字を置く基調。
    濃色,
    /// 明るい地に暗い文字を置く基調。
    淡色,
}

impl 明暗 {
    pub(super) fn eguiの明暗へ変換する(self) -> egui::Theme {
        match self {
            明暗::濃色 => egui::Theme::Dark,
            明暗::淡色 => egui::Theme::Light,
        }
    }
}
