//! 重ねる容器。下地の子を描いてから、その矩形の上へ別の子を重ねて描く。映像の上のボタンや、
//! 半透明の覆いの上の案内のように、同じ場所に部品を重ねたいときに使う。
//!
//! 重ねる子を寄せ先へ描く手順は `overlay_place` が持つ。
//! egui は後に登録した部品を上として押下を配るため、重ねる子は下地より優先して押される。

use std::rc::Rc;

use crate::measure::{画素, 論理画素};
use crate::tree::container::overlay_place::{寄せて描く, 重ねる位置, 重ねる置き方};
use crate::tree::ノード;

const 既定の端からの間隔: 論理画素 = 画素(8.0);

/// 重ね型とは、下地の子と、その上へ重ねる子の一覧の記述のことである。
pub struct 重ね型<M> {
    下地: Box<ノード<M>>,
    重ねる子一覧: Vec<(重ねる置き方, ノード<M>)>,
    端からの間隔: 論理画素,
}

impl<M> 重ね型<M> {
    pub(crate) fn 新規(下地: ノード<M>) -> Self {
        Self {
            下地: Box::new(下地),
            重ねる子一覧: Vec::new(),
            端からの間隔: 既定の端からの間隔,
        }
    }

    /// 下地の端から間隔を空けた範囲の中の、指定の位置へ子を寄せて重ねる。書いた順に上へ重なる。
    pub fn 上に置く(mut self, 位置: 重ねる位置, 子: impl Into<ノード<M>>) -> Self {
        self.重ねる子一覧
            .push((重ねる置き方::寄せて置く(位置), 子.into()));
        self
    }

    /// 下地の全体を色で塗って覆い、その真ん中へ子を重ねる。覆っている間、下地は押せない。
    pub fn 覆って中央に置く(
        mut self,
        覆う色: egui::Color32,
        子: impl Into<ノード<M>>,
    ) -> Self {
        self.重ねる子一覧
            .push((重ねる置き方::覆って中央に置く(覆う色), 子.into()));
        self
    }

    /// `上に置く` で寄せるときに、下地の端から空ける間隔を指定する。既定は8論理画素である。
    pub fn 端からの間隔(mut self, 間隔: 論理画素) -> Self {
        self.端からの間隔 = 間隔;
        self
    }
}

impl<M: Clone> 重ね型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 下地の反応 = self.下地.描画する(ui, 発行した応答);
        let 下地の矩形 = 下地の反応.rect;
        for (番号, (置き方, 子)) in self.重ねる子一覧.iter().enumerate() {
            let 鍵 = 下地の反応.id.with(("重ねる子", 番号));
            let (位置, 範囲) = match *置き方 {
                重ねる置き方::寄せて置く(位置) => {
                    (位置, 下地の矩形.shrink(self.端からの間隔.eguiへ渡す値()))
                }
                重ねる置き方::覆って中央に置く(色) => {
                    ui.painter().rect_filled(下地の矩形, 0.0, 色);
                    let _ = ui.interact(下地の矩形, 鍵.with("覆い"), egui::Sense::click());
                    (重ねる位置::中央, 下地の矩形)
                }
            };
            寄せて描く(ui, 鍵, 位置, 範囲, 子, 発行した応答);
        }
        下地の反応
    }
}

impl<M: 'static> 重ね型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 重ね型<N> {
        重ね型 {
            下地: Box::new(self.下地.rcで写す(Rc::clone(&応答を変換する))),
            重ねる子一覧: self
                .重ねる子一覧
                .into_iter()
                .map(|(置き方, 子)| (置き方, 子.rcで写す(Rc::clone(&応答を変換する))))
                .collect(),
            端からの間隔: self.端からの間隔,
        }
    }
}
