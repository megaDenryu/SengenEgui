//! 時間の行の並びのドラッグの読み取り。egui の反応から、押した所の判定（掴み始め）と、掴んでいる間の記憶の続きを読み、
//! そのフレームの出来事を返す。掴み始めの出来事と記憶の決め方は `grab`、カーソルの形の合わせ方は `cursor` に書く。
//! 記憶はドラッグの間フレームをまたいで保つ必要があるため、複数の範囲枠と同じく、反応の識別子から作った識別子で
//! egui の一時記憶に置く（ノード木は毎フレーム捨てるため木の側には置けない）。

use super::hit::画面上の塊の並び;
use super::hold::{
    ドラッグの今の様子, ドラッグの段階, 掴んでいる間の記憶, 読み取った出来事
};
use super::layout::時間の行の並びの配置;
use super::時間の塊の鍵;

/// 塊と位置の線を動かすボタン。右ボタンと中ボタンは右クリックメニュー等の別の操作に使われるため、ドラッグとして扱わない。
const 主ボタン: egui::PointerButton = egui::PointerButton::Primary;

/// 時間の行の並びのドラッグの読み取り手とは、1フレームの反応と、配置と、画面上の塊の並びと、位置の線を動かせるかの組のことである。
pub(super) struct 時間の行の並びのドラッグの読み取り手<'描画, 鍵> {
    pub(super) 反応: &'描画 egui::Response,
    pub(super) 配置: &'描画 時間の行の並びの配置,
    pub(super) 塊の並び: &'描画 画面上の塊の並び<'描画, 鍵>,
    pub(super) 位置の線を動かせるか: bool,
}

impl<鍵: 時間の塊の鍵> 時間の行の並びのドラッグの読み取り手<'_, 鍵> {
    /// このフレームで見つけた出来事を順に返す。押した位置が分からないまま始まったドラッグは無視する。
    pub(super) fn 出来事を読み取る(
        &self,
        ui: &egui::Ui,
    ) -> Vec<読み取った出来事<鍵>> {
        let 識別子 = self.記憶の識別子();
        let mut 出来事一覧 = Vec::new();
        if self.反応.drag_started_by(主ボタン) {
            ui.data_mut(|記憶域| 記憶域.remove::<掴んでいる間の記憶<鍵>>(識別子));
            let 押した位置 = ui
                .input(|入力| 入力.pointer.press_origin())
                .or(self.反応.interact_pointer_pos());
            if let Some(位置) = 押した位置 {
                let (掴み始めの出来事一覧, 記憶) = self.押した位置から掴む(位置);
                出来事一覧.extend(掴み始めの出来事一覧);
                if let Some(記憶) = 記憶 {
                    ui.data_mut(|記憶域| 記憶域.insert_temp(識別子, 記憶));
                }
            }
        }
        let Some(記憶) =
            ui.data(|記憶域| 記憶域.get_temp::<掴んでいる間の記憶<鍵>>(識別子))
        else {
            return 出来事一覧;
        };
        let 様子 = ドラッグの今の様子 {
            ポインタの位置: ui.input(|入力| 入力.pointer.latest_pos()),
            段階: self.ドラッグの段階(),
        };
        let 結果 = 記憶.続きを読む(様子, self.配置, self.塊の並び);
        ui.data_mut(|記憶域| match 結果.次の記憶 {
            Some(次の記憶) => 記憶域.insert_temp(識別子, 次の記憶),
            None => 記憶域.remove::<掴んでいる間の記憶<鍵>>(識別子),
        });
        出来事一覧.extend(結果.出来事一覧);
        出来事一覧
    }

    /// 押した位置から、掴み始めのフレームの出来事と、掴んだものの記憶を決める。
    fn 押した位置から掴む(
        &self,
        位置: egui::Pos2,
    ) -> (Vec<読み取った出来事<鍵>>, Option<掴んでいる間の記憶<鍵>>) {
        self.塊の並び
            .押した所を判定する(self.配置, 位置, self.位置の線を動かせるか)
            .掴み始める(位置, self.配置)
    }

    fn ドラッグの段階(&self) -> ドラッグの段階 {
        if self.反応.drag_stopped_by(主ボタン) {
            ドラッグの段階::放した
        } else if self.反応.dragged_by(主ボタン) {
            ドラッグの段階::続けている
        } else {
            ドラッグの段階::放したと判定できずに終わった
        }
    }

    pub(super) fn 記憶の識別子(&self) -> egui::Id {
        self.反応.id.with("時間の行の並びの掴んでいる間の記憶")
    }
}
