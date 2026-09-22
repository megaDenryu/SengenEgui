//! パネルの位置と、それを egui のパネルの種類へ写す対応表。パネル型の描画が使う。

/// パネルの位置とは、親の領域のどの縁に固定するか、または中央を埋めるかの区別のことである。
#[derive(Clone, Copy)]
pub enum パネルの位置 {
    /// 上の縁に固定する。
    上,
    /// 下の縁に固定する。
    下,
    /// 左の縁に固定する。
    左,
    /// 右の縁に固定する。
    右,
    /// 縁のパネルが残した領域を埋める。
    中央,
}

/// 縁の種類とは、パネルの位置を egui のパネルの種類へ写した区別のことである。
pub(super) enum 縁の種類 {
    左右(egui::panel::Side),
    上下(egui::panel::TopBottomSide),
    中央,
}

impl パネルの位置 {
    pub(super) fn 縁の種類へ変換する(self) -> 縁の種類 {
        match self {
            Self::左 => 縁の種類::左右(egui::panel::Side::Left),
            Self::右 => 縁の種類::左右(egui::panel::Side::Right),
            Self::上 => 縁の種類::上下(egui::panel::TopBottomSide::Top),
            Self::下 => 縁の種類::上下(egui::panel::TopBottomSide::Bottom),
            Self::中央 => 縁の種類::中央,
        }
    }
}
