//! 一定時間で消える通知。画面の下の中央に、他の部品の上へ重ねて文を出し、表示する長さが過ぎたら消す。
//!
//! 出した時刻は egui の一時記憶に置く。ノード木は毎フレーム捨てるため木の側には置けず、
//! 利用する側に時刻を持たせると、利用する側が時計と消す応答を扱うことになるためである。
//! 新しい通知かどうかは `通知の回` で見分ける。同じ文を続けて出しても、回を進めれば表示の時間が始めから数え直される。
//! 消えるまでの間は、消える時刻に描き直しを予約する。入力が無く描き直しが止まっている画面でも消えるようにするためである。

use std::time::Duration;

use crate::measure::{画素, 論理画素};
use crate::style::スタイル;

const 既定の表示する長さ: Duration = Duration::from_secs(3);
const 画面の下端からの間隔: 論理画素 = 画素(18.0);

/// 通知の回とは、何番目に出した通知かを数える値のことである。新しい通知を出すたびに `次` で進める。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct 通知の回(u64);

impl 通知の回 {
    /// 最初の回。
    pub const fn 最初() -> Self {
        Self(0)
    }

    /// この次の回。
    pub const fn 次(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

/// 一定時間で消える通知型とは、通知の文と、その回と、表示する長さの記述のことである。
pub struct 一定時間で消える通知型 {
    識別子: String,
    回: 通知の回,
    文: String,
    表示する長さ: Duration,
    装飾値: スタイル,
}

impl 一定時間で消える通知型 {
    pub(crate) fn 新規(識別子: String, 回: 通知の回, 文: String) -> Self {
        Self {
            識別子,
            回,
            文,
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
        let Some(残り) = self.残りの表示時間を求める(ui) else {
            return ui.response();
        };
        ui.ctx().request_repaint_after(残り);
        let 枠 = self.装飾値.枠の指定を重ねる(egui::Frame::popup(ui.style()));
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.文.clone()));
        egui::Area::new(self.記憶の鍵())
            .anchor(
                egui::Align2::CENTER_BOTTOM,
                egui::vec2(0.0, -画面の下端からの間隔.eguiへ渡す値()),
            )
            .order(egui::Order::Foreground)
            .interactable(false)
            .show(ui.ctx(), |内側| 枠.show(内側, |内側| 内側.label(文字)))
            .response
    }

    /// この回を初めて描いた時刻を記憶し、表示の残り時間を返す。表示する長さを過ぎていれば None を返す。
    fn 残りの表示時間を求める(&self, ui: &egui::Ui) -> Option<Duration> {
        let 鍵 = self.記憶の鍵();
        let 今 = ui.input(|入力| 入力.time);
        let 記憶 = ui.data(|記憶域| 記憶域.get_temp::<(通知の回, f64)>(鍵));
        let 出した時刻 = match 記憶 {
            Some((回, 時刻)) if 回 == self.回 => 時刻,
            _ => {
                ui.data_mut(|記憶域| 記憶域.insert_temp(鍵, (self.回, 今)));
                今
            }
        };
        let 残りの秒 = self.表示する長さ.as_secs_f64() - (今 - 出した時刻);
        (残りの秒 > 0.0).then(|| Duration::from_secs_f64(残りの秒))
    }

    fn 記憶の鍵(&self) -> egui::Id {
        egui::Id::new(("一定時間で消える通知", &self.識別子))
    }
}
