//! 縁のパネルの大きさの指定。縁のドラッグで大きさを変えられるかと、幅と高さそれぞれの寸法の指定を持ち、
//! その指定どおりの egui の縁のパネルを作る。
//! 左右のパネルには幅の指定だけを、上下のパネルには高さの指定だけを写す。中央のパネルにはどちらも写さない。
//!
//! 注意: 幅と高さを1つの寸法の指定で共有してはならない。共有すると、上のパネルに書いた幅の範囲が高さの範囲として効く。
//! 注意: egui は範囲を最初の寸法より後に適用し、最初の寸法を範囲へ収める。記憶した寸法も毎フレーム範囲へ収めるため、
//! 範囲をフレームごとに変えても次のフレームから効く。寸法はパネルの装飾の内余白を含む。

use crate::measure::論理画素;
use crate::tree::container::panel_axis_size::一方向の寸法の指定;

/// パネルの大きさの指定とは、縁のパネルについて、変えられるかと、幅と高さそれぞれの寸法の指定を合わせたもののことである。
#[derive(Clone, Copy, Default)]
pub(super) struct パネルの大きさの指定 {
    変えられるか: bool,
    幅: 一方向の寸法の指定,
    高さ: 一方向の寸法の指定,
}

impl パネルの大きさの指定 {
    pub(super) fn 変えられるかを決める(self, 変えられるか: bool) -> Self {
        Self {
            変えられるか,
            ..self
        }
    }

    pub(super) fn 最初の幅を決める(self, 幅: 論理画素) -> Self {
        Self {
            幅: self.幅.最初の寸法を決める(幅),
            ..self
        }
    }

    pub(super) fn 幅の範囲を決める(
        self, 最小: 論理画素, 最大: 論理画素
    ) -> Self {
        Self {
            幅: self.幅.範囲を決める(最小, 最大),
            ..self
        }
    }

    pub(super) fn 最初の高さを決める(self, 高さ: 論理画素) -> Self {
        Self {
            高さ: self.高さ.最初の寸法を決める(高さ),
            ..self
        }
    }

    pub(super) fn 高さの範囲を決める(
        self, 最小: 論理画素, 最大: 論理画素
    ) -> Self {
        Self {
            高さ: self.高さ.範囲を決める(最小, 最大),
            ..self
        }
    }

    pub(super) fn 左右のパネルを作る(
        self,
        側: egui::panel::Side,
        識別子: egui::Id,
    ) -> egui::SidePanel {
        let mut パネル = egui::SidePanel::new(側, 識別子).resizable(self.変えられるか);
        if let Some(幅) = self.幅.eguiへ渡す最初の寸法() {
            パネル = パネル.default_width(幅);
        }
        if let Some(範囲) = self.幅.eguiへ渡す範囲() {
            パネル = パネル.width_range(範囲);
        }
        パネル
    }

    pub(super) fn 上下のパネルを作る(
        self,
        側: egui::panel::TopBottomSide,
        識別子: egui::Id,
    ) -> egui::TopBottomPanel {
        let mut パネル = egui::TopBottomPanel::new(側, 識別子).resizable(self.変えられるか);
        if let Some(高さ) = self.高さ.eguiへ渡す最初の寸法() {
            パネル = パネル.default_height(高さ);
        }
        if let Some(範囲) = self.高さ.eguiへ渡す範囲() {
            パネル = パネル.height_range(範囲);
        }
        パネル
    }
}
