//! 重ねる子の一覧。重ねる容器が下地の上へ重ねる子を、置き方と組にして書いた順に持ち、下地を描いた後に描く。
//!
//! 測った大きさは、利用する側が渡す識別子と、寄せる位置と、同じ位置の中での番号の組を鍵に記憶する。兄弟の部品が出入りしても鍵は変わらず、
//! 条件付きで重ねる子を足し引きしても、別の位置の子の鍵は変わらない。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::container::overlay_place::重ねる置き方;
use crate::tree::ノード;

/// 重ねる子の一覧とは、下地の上へ重ねる子と、その置き方の組を、書いた順に並べたもののことである。
pub(super) struct 重ねる子の一覧<M> {
    子一覧: Vec<(重ねる置き方, ノード<M>)>,
}

impl<M> 重ねる子の一覧<M> {
    pub(super) fn 空() -> Self {
        Self {
            子一覧: Vec::new()
        }
    }

    /// 子を一覧の最後へ足す。後に足した子ほど上へ重なる。
    pub(super) fn 足す(&mut self, 置き方: 重ねる置き方, 子: ノード<M>) {
        self.子一覧.push((置き方, 子));
    }

    /// 覆って中央に置く子があるか。あれば下地を無効で描く。
    pub(super) fn 覆っているか(&self) -> bool {
        self.子一覧
            .iter()
            .any(|(置き方, _)| matches!(置き方, 重ねる置き方::覆って中央に置く(_)))
    }
}

impl<M: Clone> 重ねる子の一覧<M> {
    /// 子を書いた順に、下地の矩形の上へ描く。
    pub(super) fn 描く(
        &self,
        ui: &mut egui::Ui,
        重ねるの識別子: &str,
        下地の矩形: egui::Rect,
        端からの間隔: 論理画素,
        発行した応答: &mut Vec<M>,
    ) {
        let mut 数えた区分一覧 = Vec::with_capacity(self.子一覧.len());
        for (置き方, 子) in &self.子一覧 {
            let 区分 = 置き方.記憶の区分();
            let 同じ区分の番号 = 数えた区分一覧
                .iter()
                .filter(|数えた| **数えた == 区分)
                .count();
            数えた区分一覧.push(区分);
            let 鍵 = egui::Id::new(("重ねる", 重ねるの識別子, 区分, 同じ区分の番号));
            置き方.子を描く(ui, 鍵, 下地の矩形, 端からの間隔, 子, 発行した応答);
        }
    }
}

impl<M: 'static> 重ねる子の一覧<M> {
    pub(super) fn 写す<N: 'static>(
        self,
        応答を変換する: &Rc<dyn Fn(M) -> N>,
    ) -> 重ねる子の一覧<N> {
        重ねる子の一覧 {
            子一覧: self
                .子一覧
                .into_iter()
                .map(|(置き方, 子)| (置き方, 子.rcで写す(Rc::clone(応答を変換する))))
                .collect(),
        }
    }
}
