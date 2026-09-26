//! ポインタが止まると隠す置き方の出し入れの規則。重ねる子を、下地の上でポインタが動いてから一定時間だけ出す。
//! 動画の再生画面の操作の欄のように、見ている間は隠し、マウスを動かせば出てくる部品に使う。
//!
//! 最後にポインタが動いた時刻は、重ねる子の大きさと同じく egui の一時記憶に置く。子の上にポインタがある間と、
//! 子の上で押し始めたボタンを押し続けている間（つまみのドラッグ等）は、動いたものとして数え続け、隠さない。
//! 前の回に描いていなかった子（初めて出す回や、木から外した後に戻した回）は、出した回から数え始める。

use std::time::Duration;

/// ポインタが止まると隠す指定とは、重ねる子を、下地の上でポインタが動いてから一定時間だけ出すときの条件のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ポインタが止まると隠す指定 {
    隠すまでの時間: Duration,
    出したままにするか: bool,
    隠している間はカーソルも隠すか: bool,
}

impl ポインタが止まると隠す指定 {
    /// ポインタが最後に動いてから `隠すまでの時間` が経ったら子を隠す指定を作る。カーソルは隠さない。
    pub fn 作成する(隠すまでの時間: Duration) -> Self {
        Self {
            隠すまでの時間,
            出したままにするか: false,
            隠している間はカーソルも隠すか: false,
        }
    }

    /// 条件が真の間は、ポインタが止まっていても子を出したままにする。偽に戻すと、最後に真だった回から数え始める。
    /// 一時停止している間は操作の欄を出したままにする、のように利用する側の状態で決める条件に使う。
    pub fn 出したままにする(mut self, 条件: bool) -> Self {
        self.出したままにするか = 条件;
        self
    }

    /// 子を隠している間、ポインタが下地の上にあればマウスのカーソルも隠す。
    pub fn 隠している間はカーソルも隠す(mut self) -> Self {
        self.隠している間はカーソルも隠すか = true;
        self
    }

    /// 今回子を出すかを決めて記憶を更新する。出している間は隠す時刻に描き直しを予約し、
    /// 隠している間は指定があればカーソルを隠す。子の矩形は前の回に測った大きさで寄せた矩形である。
    pub(super) fn 出すかを決める(
        self,
        ui: &egui::Ui,
        鍵: egui::Id,
        下地の矩形: egui::Rect,
        子の矩形: egui::Rect,
    ) -> bool {
        let 様子 = ポインタの様子::読む(ui, 下地の矩形, 子の矩形);
        let 記憶の鍵 = 鍵.with("ポインタが最後に動いた時刻");
        let 今の回 = ui.ctx().cumulative_pass_nr();
        let 記憶 = ui.data(|記憶域| 記憶域.get_temp::<(f64, u64)>(記憶の鍵));
        let 前の回に描いた時刻 = 記憶
            .filter(|(_, 回)| 回.saturating_add(1) >= 今の回)
            .map(|(時刻, _)| 時刻);
        let 数え直すか = self.出したままにするか || 様子.止めずに数え直すか();
        let 最後に動いた時刻 = match 前の回に描いた時刻 {
            Some(時刻) if !数え直すか => 時刻,
            _ => 様子.今,
        };
        ui.data_mut(|記憶域| 記憶域.insert_temp(記憶の鍵, (最後に動いた時刻, 今の回)));
        let 残りの秒 = self.隠すまでの時間.as_secs_f64() - (様子.今 - 最後に動いた時刻);
        if 残りの秒 > 0.0 {
            if !self.出したままにするか {
                ui.ctx()
                    .request_repaint_after(Duration::from_secs_f64(残りの秒));
            }
            return true;
        }
        if self.隠している間はカーソルも隠すか && 様子.下地の上か {
            ui.ctx().set_cursor_icon(egui::CursorIcon::None);
        }
        false
    }
}

/// ポインタの様子とは、出し入れを決めるためにその回の入力から読み取った値の組のことである。
struct ポインタの様子 {
    今: f64,
    下地の上で動いたか: bool,
    下地の上か: bool,
    子の上か: bool,
    子の上で押し始めて押し続けているか: bool,
}

impl ポインタの様子 {
    fn 読む(ui: &egui::Ui, 下地の矩形: egui::Rect, 子の矩形: egui::Rect) -> Self {
        ui.input(|入力| {
            let 位置 = 入力.pointer.latest_pos();
            let 下地の上か = 位置.is_some_and(|位置| 下地の矩形.contains(位置));
            let 動いたか = 入力.events.iter().any(|出来事| {
                matches!(
                    出来事,
                    egui::Event::PointerMoved(_) | egui::Event::PointerButton { pressed: true, .. }
                )
            });
            Self {
                今: 入力.time,
                下地の上で動いたか: 動いたか && 下地の上か,
                下地の上か,
                子の上か: 位置.is_some_and(|位置| 子の矩形.contains(位置)),
                子の上で押し始めて押し続けているか: 入力.pointer.any_down()
                    && 入力
                        .pointer
                        .press_origin()
                        .is_some_and(|位置| 子の矩形.contains(位置)),
            }
        })
    }

    /// ポインタが止まっていても、最後に動いた時刻をこの回へ数え直すか。
    fn 止めずに数え直すか(&self) -> bool {
        self.下地の上で動いたか || self.子の上か || self.子の上で押し始めて押し続けているか
    }
}
