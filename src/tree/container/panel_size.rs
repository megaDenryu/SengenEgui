//! 縁のパネルの大きさの指定。縁のドラッグで大きさを変えられるかと、最初に表示したときの寸法と、寸法が取れる範囲を持ち、
//! その指定どおりの egui の縁のパネルを作る。
//! 左右のパネルでは幅、上下のパネルでは高さを指す。中央のパネルには写さない。
//!
//! 注意: egui は範囲を最初の寸法より後に適用し、最初の寸法を範囲へ収める。記憶した寸法も毎フレーム範囲へ収めるため、
//! 範囲をフレームごとに変えても次のフレームから効く。寸法はパネルの装飾の内余白を含む。

use crate::measure::論理画素;

/// 寸法の範囲とは、縁のパネルの寸法が取れる最小と最大の組のことである。最大は最小以上に揃えてある。
#[derive(Clone, Copy)]
struct 寸法の範囲 {
    最小: 論理画素,
    最大: 論理画素,
}

/// パネルの大きさの指定とは、縁のパネルの幅または高さについて、変えられるかと最初の寸法と範囲を合わせたもののことである。
#[derive(Clone, Copy, Default)]
pub(super) struct パネルの大きさの指定 {
    変えられるか: bool,
    最初の寸法: Option<論理画素>,
    範囲: Option<寸法の範囲>,
}

impl パネルの大きさの指定 {
    pub(super) fn 変えられるかを決める(self, 変えられるか: bool) -> Self {
        Self {
            変えられるか,
            ..self
        }
    }

    pub(super) fn 最初の寸法を決める(self, 寸法: 論理画素) -> Self {
        Self {
            最初の寸法: Some(寸法),
            ..self
        }
    }

    /// 最大が最小より小さいときは最小を優先する（CSS の min-width が max-width に勝つのと同じ）。
    pub(super) fn 範囲を決める(self, 最小: 論理画素, 最大: 論理画素) -> Self {
        let 範囲 = 寸法の範囲 {
            最小,
            最大: 最大.大きい方(最小),
        };
        Self {
            範囲: Some(範囲),
            ..self
        }
    }

    pub(super) fn 左右のパネルを作る(
        self,
        側: egui::panel::Side,
        識別子: egui::Id,
    ) -> egui::SidePanel {
        let mut パネル = egui::SidePanel::new(側, 識別子).resizable(self.変えられるか);
        if let Some(幅) = self.最初の寸法 {
            パネル = パネル.default_width(幅.eguiへ渡す値());
        }
        if let Some(範囲) = self.範囲 {
            パネル = パネル.width_range(範囲.最小.eguiへ渡す値()..=範囲.最大.eguiへ渡す値());
        }
        パネル
    }

    pub(super) fn 上下のパネルを作る(
        self,
        側: egui::panel::TopBottomSide,
        識別子: egui::Id,
    ) -> egui::TopBottomPanel {
        let mut パネル = egui::TopBottomPanel::new(側, 識別子).resizable(self.変えられるか);
        if let Some(高さ) = self.最初の寸法 {
            パネル = パネル.default_height(高さ.eguiへ渡す値());
        }
        if let Some(範囲) = self.範囲 {
            パネル = パネル.height_range(範囲.最小.eguiへ渡す値()..=範囲.最大.eguiへ渡す値());
        }
        パネル
    }
}
