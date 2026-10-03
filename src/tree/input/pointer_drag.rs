//! 入力の部品が共有するドラッグの読み取りの手順。部品は主ボタンのドラッグだけを扱い、ドラッグを始めた回に前の記憶を
//! 捨てて押した座標を求め、掴んでいる間の記憶を egui の一時記憶に反応の識別子から作った識別子で置く。
//! 記憶を一時記憶に置くのは、ノード木は毎フレーム捨てるため木の側には置けず、egui 自身もドラッグ中の部品を識別子で
//! 覚えているためである。押した座標に今のポインタの位置でなく押した位置を使うのは、egui がポインタが少し動いてから
//! ドラッグの開始とすることがあるためである。

use std::marker::PhantomData;

// 右ボタンと中ボタンは右クリックメニュー等の別の操作に使われるため、ドラッグとして扱わない。
const 主ボタン: egui::PointerButton = egui::PointerButton::Primary;

// ドラッグの段階とは、掴み始めた後の1フレームで、egui のドラッグが続いているか、放したか、どちらとも判定できずに
// 終わったか（ポインタを見失った等）の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tree::input) enum ドラッグの段階 {
    続けている,
    放した,
    放したと判定できずに終わった,
}

impl ドラッグの段階 {
    pub(in crate::tree::input) fn 反応から読む(反応: &egui::Response) -> Self {
        if 反応.drag_stopped_by(主ボタン) {
            Self::放した
        } else if 反応.dragged_by(主ボタン) {
            Self::続けている
        } else {
            Self::放したと判定できずに終わった
        }
    }
}

// 掴んでいる間に置く記憶とは、部品が掴んでいる間フレームをまたいで egui の一時記憶に置く値の型が満たす条件のことである。
// 一時記憶の識別子を作るための名前を、記憶の型ごとに関連定数として持つ。
pub(in crate::tree::input) trait 掴んでいる間に置く記憶:
    Clone + Send + Sync + 'static
{
    const 記憶の名前: &'static str;
}

// 掴んでいる間の記憶の置き場とは、部品が掴んでいる間の記憶を置く egui の一時記憶の識別子のことである。
pub(in crate::tree::input) struct 掴んでいる間の記憶の置き場<記憶> {
    識別子: egui::Id,
    記憶の型: PhantomData<fn() -> 記憶>,
}

impl<記憶: 掴んでいる間に置く記憶> 掴んでいる間の記憶の置き場<記憶> {
    // 反応の識別子と、記憶の型が持つ名前から作る。
    pub(in crate::tree::input) fn 反応から作る(反応: &egui::Response) -> Self {
        Self {
            識別子: 反応.id.with(記憶::記憶の名前),
            記憶の型: PhantomData,
        }
    }

    // この回に主ボタンのドラッグを始めたなら、前の記憶を捨てて押した座標を返す。始めていないか、押した座標が
    // 分からないなら None を返す。
    pub(in crate::tree::input) fn ドラッグを始めた回なら押した座標を返す(
        &self,
        ui: &egui::Ui,
        反応: &egui::Response,
    ) -> Option<egui::Pos2> {
        if !反応.drag_started_by(主ボタン) {
            return None;
        }
        self.捨てる(ui);
        ui.input(|入力| 入力.pointer.press_origin())
            .or(反応.interact_pointer_pos())
    }

    pub(in crate::tree::input) fn 読む(&self, ui: &egui::Ui) -> Option<記憶> {
        ui.data(|記憶域| 記憶域.get_temp::<記憶>(self.識別子))
    }

    pub(in crate::tree::input) fn 置く(&self, ui: &egui::Ui, 置く記憶: 記憶) {
        ui.data_mut(|記憶域| 記憶域.insert_temp(self.識別子, 置く記憶));
    }

    pub(in crate::tree::input) fn 捨てる(&self, ui: &egui::Ui) {
        ui.data_mut(|記憶域| 記憶域.remove::<記憶>(self.識別子));
    }

    // 次のフレームへ持ち越す記憶があれば置き、無ければ捨てる。
    pub(in crate::tree::input) fn 次の記憶にする(
        &self,
        ui: &egui::Ui,
        次の記憶: Option<記憶>,
    ) {
        match 次の記憶 {
            Some(次の記憶) => self.置く(ui, 次の記憶),
            None => self.捨てる(ui),
        }
    }
}

// 今のポインタの位置。ポインタの位置が分からないフレーム（ポインタがウィンドウの外へ出た等）は None を返す。
pub(in crate::tree::input) fn 今のポインタの座標(ui: &egui::Ui) -> Option<egui::Pos2> {
    ui.input(|入力| 入力.pointer.latest_pos())
}
