//! ポインタが止まると隠す子の出し入れを決めるために、その回の入力から読み取るポインタの様子。
//! 組にした子の分をまとめて読み、下地の上か・子の上かは組の全ての子の矩形の和で判定する。

/// 重ねた子の矩形とは、ポインタが止まると隠す子1つの、下地の矩形と、前の回に測った大きさで寄せた子の矩形の組のことである。
#[derive(Clone, Copy, Debug)]
pub(super) struct 重ねた子の矩形 {
    pub(super) 下地: egui::Rect,
    pub(super) 子: egui::Rect,
}

/// ポインタの様子とは、出し入れを決めるためにその回の入力から読み取った値の組のことである。
pub(super) struct ポインタの様子 {
    pub(super) 今: f64,
    下地の上で動いたか: bool,
    pub(super) 下地の上か: bool,
    子の上か: bool,
    子の上で押し始めて押し続けているか: bool,
}

impl ポインタの様子 {
    /// 組の全ての子の矩形について、ポインタが下地の上か・子の上かを読む。
    pub(super) fn 読む(ui: &egui::Ui, 矩形一覧: &[重ねた子の矩形]) -> Self {
        ui.input(|入力| {
            let 位置 = 入力.pointer.latest_pos();
            let 下地の上か =
                位置.is_some_and(|位置| 矩形一覧.iter().any(|矩形| 矩形.下地.contains(位置)));
            let 子の上か = |位置: egui::Pos2| 矩形一覧.iter().any(|矩形| 矩形.子.contains(位置));
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
                子の上か: 位置.is_some_and(子の上か),
                子の上で押し始めて押し続けているか: 入力.pointer.any_down()
                    && 入力.pointer.press_origin().is_some_and(子の上か),
            }
        })
    }

    /// ポインタが止まっていても、最後に動いた時刻をこの回へ数え直すか。
    pub(super) fn 止めずに数え直すか(&self) -> bool {
        self.下地の上で動いたか || self.子の上か || self.子の上で押し始めて押し続けているか
    }
}
