//! ポインタが止まると隠す子の出し入れを決めるために、その回の入力から読み取るポインタとキーの様子。
//! 組にした子の分をまとめて読み、下地の上か・子の上かは組の全ての子の矩形の和で判定する。
//! ここでは入力をそのまま読むだけであり、前の回と比べて「動いたか」を決めるのは `idle_memory` である。

use std::time::Duration;

/// 重ねた子の矩形とは、ポインタが止まると隠す子1つの、下地の矩形と、前の回に測った大きさで寄せた子の矩形の組のことである。
#[derive(Clone, Copy, Debug)]
pub(super) struct 重ねた子の矩形 {
    pub(super) 下地: egui::Rect,
    pub(super) 子: egui::Rect,
}

/// 入力の時刻とは、egui が入力に付ける、アプリを起動してからの時刻のことである。
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct 入力の時刻(f64);

impl 入力の時刻 {
    /// 前の時刻から今の時刻までに経った時間。時刻が戻っていれば0とする。
    pub(super) fn 前の時刻から経った時間(self, 前の時刻: Self) -> Duration {
        Duration::try_from_secs_f64(self.0 - 前の時刻.0).unwrap_or(Duration::ZERO)
    }
}

/// ポインタの様子とは、出し入れを決めるためにその回の入力から読み取った値の組のことである。
pub(super) struct ポインタの様子 {
    pub(super) 今: 入力の時刻,
    pub(super) 位置: Option<egui::Pos2>, // ウィンドウの外へ出た後は無い
    pub(super) 下地の上か: bool,
    pub(super) 下地の上で押したか: bool,
    pub(super) 子の上で押し始めて押し続けているか: bool,
    pub(super) 指定のキーを押しているか: bool,
}

impl ポインタの様子 {
    /// 組の全ての子の矩形について、ポインタが下地の上か・子の上かを読み、指定のキーを押しているかを読む。
    /// キーは押下の出来事でなく押している状態で読む。キー操作の部品が先に押下の出来事を消費しても、押している状態は残るためである。
    pub(super) fn 読む(
        ui: &egui::Ui,
        矩形一覧: &[重ねた子の矩形],
        出すキー一覧: &[egui::Key],
    ) -> Self {
        ui.input(|入力| {
            let 位置 = 入力.pointer.latest_pos();
            let 下地の上か =
                位置.is_some_and(|位置| 矩形一覧.iter().any(|矩形| 矩形.下地.contains(位置)));
            let 子の上か = |位置: egui::Pos2| 矩形一覧.iter().any(|矩形| 矩形.子.contains(位置));
            let 押したか = 入力
                .events
                .iter()
                .any(|出来事| matches!(出来事, egui::Event::PointerButton { pressed: true, .. }));
            Self {
                今: 入力の時刻(入力.time),
                位置,
                下地の上か,
                下地の上で押したか: 押したか && 下地の上か,
                子の上で押し始めて押し続けているか: 入力.pointer.any_down()
                    && 入力.pointer.press_origin().is_some_and(子の上か),
                指定のキーを押しているか: 出すキー一覧
                    .iter()
                    .any(|キー| 入力.key_down(*キー)),
            }
        })
    }
}
