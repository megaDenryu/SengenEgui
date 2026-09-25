//! 縁のパネルの一方向の寸法の指定。幅または高さの片方について、最初に表示したときの寸法と寸法が取れる範囲を持ち、
//! egui の縁のパネルへ渡す値にする。どちらの向きかは持たず、保持する側（`panel_size`）が幅用と高さ用を別々に持つ。

use crate::measure::論理画素;

/// 寸法の範囲とは、縁のパネルの寸法が取れる最小と最大の組のことである。最大は最小以上に揃えてある。
#[derive(Clone, Copy)]
struct 寸法の範囲 {
    最小: 論理画素,
    最大: 論理画素,
}

/// 一方向の寸法の指定とは、幅または高さの片方について、最初の寸法と範囲を合わせたもののことである。
#[derive(Clone, Copy, Default)]
pub(super) struct 一方向の寸法の指定 {
    最初の寸法: Option<論理画素>,
    範囲: Option<寸法の範囲>,
}

impl 一方向の寸法の指定 {
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

    pub(super) fn eguiへ渡す最初の寸法(self) -> Option<f32> {
        self.最初の寸法.map(論理画素::eguiへ渡す値)
    }

    pub(super) fn eguiへ渡す範囲(self) -> Option<std::ops::RangeInclusive<f32>> {
        self.範囲
            .map(|範囲| 範囲.最小.eguiへ渡す値()..=範囲.最大.eguiへ渡す値())
    }
}
