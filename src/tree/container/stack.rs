//! 縦積み・横並び・折り返す横並び。子を並べる向きだけが違うので1つの型で持つ。
//! 横の並びは egui の `horizontal` と同じく、行の初期高さを操作部品の高さで確保してから並べる。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::style::スタイル;
use crate::tree::container::layout::{寄せ, 積む向き};
use crate::tree::{ノード, 子を順に描画する};

/// 積み型とは、子を縦または横に順に並べる容器の記述のことである。
pub struct 積み型<M> {
    向き: 積む向き,
    子一覧: Vec<ノード<M>>,
    装飾値: スタイル,
    寄せ指定: 寄せ,
    有効指定: bool,
    幅指定: Option<論理画素>,
    最小幅指定: Option<論理画素>,
}

impl<M> 積み型<M> {
    pub(crate) fn 新規(向き: 積む向き, 子一覧: Vec<ノード<M>>) -> Self {
        Self {
            向き,
            子一覧,
            装飾値: スタイル::無指定,
            寄せ指定: 寄せ::既定,
            有効指定: true,
            幅指定: None,
            最小幅指定: None,
        }
    }

    /// 縦積みでは子を左右中央へ、横並びでは子を上下中央へ寄せる。
    pub fn 中央寄せ(mut self) -> Self {
        self.寄せ指定 = 寄せ::中央;
        self
    }

    /// 横並びで子を右から左へ詰める。縦積みでは子を右へ寄せる。
    pub fn 右寄せ(mut self) -> Self {
        self.寄せ指定 = 寄せ::右;
        self
    }

    /// 装飾を適用する。装飾は名前付き定数として構造の外で定義する（lib.rs 方針3）。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    /// 中の部品をまとめて有効・無効にする。false の間は淡色になり操作できない。
    pub fn 有効(mut self, 有効: bool) -> Self {
        self.有効指定 = 有効;
        self
    }

    /// 並びの幅を固定する。
    pub fn 幅(mut self, 幅: 論理画素) -> Self {
        self.幅指定 = Some(幅);
        self
    }

    /// 並びの幅の下限を指定する。
    pub fn 最小幅(mut self, 幅: 論理画素) -> Self {
        self.最小幅指定 = Some(幅);
        self
    }
}

impl<M: Clone> 積み型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 装飾 = self.装飾値;
        装飾.枠で包んで描画する(ui, |内側| {
            内側
                .add_enabled_ui(self.有効指定, |内側| self.並べる(内側, 発行した応答))
                .response
        })
    }

    fn 並べる(&self, ui: &mut egui::Ui, 発行した応答: &mut Vec<M>) -> egui::Response {
        let 配置 = self.向き.配置を決める(self.寄せ指定);
        let 中身を描く = |内側: &mut egui::Ui| {
            if let Some(幅) = self.幅指定 {
                内側.set_width(幅.eguiへ渡す値());
            }
            if let Some(幅) = self.最小幅指定 {
                内側.set_min_width(幅.eguiへ渡す値());
            }
            子を順に描画する(&self.子一覧, 内側, 発行した応答);
        };
        match self.向き.行の初期の大きさ(ui) {
            None => ui.with_layout(配置, 中身を描く).response,
            Some(大きさ) => {
                ui.allocate_ui_with_layout(大きさ, 配置, 中身を描く)
                    .response
            }
        }
    }
}

impl<M: 'static> 積み型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 積み型<N> {
        積み型 {
            向き: self.向き,
            子一覧: crate::tree::map::子一覧を写す(self.子一覧, &応答を変換する),
            装飾値: self.装飾値,
            寄せ指定: self.寄せ指定,
            有効指定: self.有効指定,
            幅指定: self.幅指定,
            最小幅指定: self.最小幅指定,
        }
    }
}
