//! 一定時間で消える通知。画面の下の中央に、他の部品の上へ重ねて文を出し、表示する長さが過ぎたら消す。
//! 複数の通知を渡すと、渡した順に上から縦に並べ、通知ごとに表示の時間を数えて過ぎたものから消す。
//! 続けて出た通知が、前の通知を読む前に置き換えないためである。
//!
//! 出した時刻は識別子ごとに1つの並び(回と時刻の組)として egui の一時記憶に置き、渡されなくなった回を捨てる。ノード木は毎フレーム捨てるため木の側には置けず、
//! 利用する側に時刻を持たせると、利用する側が時計と消す応答を扱うことになるためである。
//! 新しい通知かどうかは `通知の回` で見分ける。同じ文を続けて出しても、回を進めれば表示の時間が始めから数え直される。
//! egui の重ねる領域（Area）は、識別子ごとに最初に出した1回だけ大きさを測って見えない。2回目以降は出した回から見える。
//! 消えるまでの間は、消える時刻に描き直しを予約する。入力が無く描き直しが止まっている画面でも消えるようにするためである。

use std::time::Duration;

use crate::measure::{画素, 論理画素};
use crate::style::スタイル;

pub use super::notice_item::{並べる通知, 通知の回};

const 既定の表示する長さ: Duration = Duration::from_secs(3);
const 画面の下端からの間隔: 論理画素 = 画素(18.0);

/// 一定時間で消える通知型とは、並べる通知の列と、表示する長さの記述のことである。
pub struct 一定時間で消える通知型 {
    識別子: String,
    通知: Vec<並べる通知>,
    表示する長さ: Duration,
    装飾値: スタイル,
}

impl 一定時間で消える通知型 {
    pub(crate) fn 新規(識別子: String, 通知: Vec<並べる通知>) -> Self {
        Self {
            識別子,
            通知,
            表示する長さ: 既定の表示する長さ,
            装飾値: スタイル::無指定,
        }
    }

    /// 表示する長さを指定する。既定は3秒である。
    pub fn 表示する長さ(mut self, 長さ: Duration) -> Self {
        self.表示する長さ = 長さ;
        self
    }

    /// 装飾を適用する。文字系の指定は文へ、枠系の指定は吹き出しの枠（既定は egui の吹き出しの見た目）へ効く。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        let 今 = ui.input(|入力| 入力.time);
        let 出した時刻 = self.出した時刻を覚え直す(ui, 今);
        let 出ている: Vec<(&並べる通知, Duration)> = self
            .通知
            .iter()
            .zip(&出した時刻)
            .filter_map(|(通知, (_, 時刻))| {
                self.残りの表示時間を求める(*時刻, 今)
                    .map(|残り| (通知, 残り))
            })
            .collect();
        let Some(最も短い残り) = 出ている.iter().map(|(_, 残り)| *残り).min() else {
            return ui.response();
        };
        ui.ctx().request_repaint_after(最も短い残り);
        let 枠 = self.装飾値.枠の指定を重ねる(egui::Frame::popup(ui.style()));
        egui::Area::new(egui::Id::new(("一定時間で消える通知", &self.識別子)))
            .anchor(
                egui::Align2::CENTER_BOTTOM,
                egui::vec2(0.0, -画面の下端からの間隔.eguiへ渡す値()),
            )
            .order(egui::Order::Foreground)
            .interactable(false)
            .show(ui.ctx(), |内側| {
                for (通知, _) in &出ている {
                    let 文字 = self
                        .装飾値
                        .文字へ適用する(egui::RichText::new(通知.文.clone()));
                    枠.show(内側, |内側| 内側.label(文字));
                }
            })
            .response
    }

    /// 渡された通知の回ごとに、その回を初めて描いた時刻を一時記憶から読み、無い回には今の時刻を足して書き戻す。
    /// 記憶は識別子ごとに1つの並びに置き、渡されなくなった回を捨てる。回ごとに別の場所へ置くと、表示を終えた回の記憶が残り続けるためである。
    /// 返す並びは、渡された通知と同じ順に、回と初めて描いた時刻を並べたものである。
    fn 出した時刻を覚え直す(&self, ui: &egui::Ui, 今: f64) -> Vec<(通知の回, f64)> {
        let 鍵 = egui::Id::new(("一定時間で消える通知の回", &self.識別子));
        let 覚えていた: Vec<(通知の回, f64)> =
            ui.data(|記憶域| 記憶域.get_temp(鍵)).unwrap_or_default();
        let 覚え直した: Vec<(通知の回, f64)> = self
            .通知
            .iter()
            .map(|通知| {
                let 時刻 = 覚えていた
                    .iter()
                    .find(|(回, _)| *回 == 通知.回)
                    .map_or(今, |(_, 時刻)| *時刻);
                (通知.回, 時刻)
            })
            .collect();
        ui.data_mut(|記憶域| 記憶域.insert_temp(鍵, 覚え直した.clone()));
        覚え直した
    }

    /// 初めて描いた時刻から、表示の残り時間を求める。表示する長さを過ぎていれば None を返す。
    fn 残りの表示時間を求める(
        &self, 出した時刻: f64, 今: f64
    ) -> Option<Duration> {
        let 残りの秒 = self.表示する長さ.as_secs_f64() - (今 - 出した時刻);
        (残りの秒 > 0.0).then(|| Duration::from_secs_f64(残りの秒))
    }
}
