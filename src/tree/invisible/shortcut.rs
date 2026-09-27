//! キー操作。画面に何も描かず、キーの組が押されたら応答を発行する。振る舞いは次のとおり。
//!
//! - 修飾キーは厳密に一致させる（Ctrl+S は Ctrl+Shift+S では発行しない）。一致した押下の事象は消費する
//! - Ctrl・Command・Alt を含まない組（文字キー・Enter・Backspace 等）は、文字を打つ部品（一行・複数行のテキスト入力と
//!   数値入力の直接入力）にフォーカスがある間は見張らず、事象も消費しない。ボタン等の文字を打たない部品のフォーカスでは見張る。
//!   egui は Space と Enter をフォーカスを持つボタンの押下として扱うため、ボタンのフォーカスで見張らないと、Space の組を
//!   置いてもボタンが押されてしまう。文字を打つ部品かは、egui がその部品のテキスト編集の状態を覚えているかで見分ける
//! - 見張っている組が押されたとき、フォーカスを持つ部品（修飾キーを含む組なら入力欄も、含まない組ならボタン等）があればそのフォーカスを手放させ、
//!   応答は次の描画の回に発行する。確定時のみ発行の入力欄はフォーカスが外れた回に確定の応答を出すため、
//!   入力欄の確定がキー操作の応答より先に並ぶ。フォーカスを持つ部品が無ければその回に発行する
//! - `入力欄にフォーカスがある間は発しない` を指定した組は、修飾キーを含んでいても、入力欄にフォーカスがある間は見張らない。
//!   Ctrl+Z のように入力欄が自分で扱う組を、入力欄の編集の間は入力欄に任せるためである
//! - 木の中でこの部品より先に描画された部品が事象を消費していれば、その押下は見えない

/// キーの組とは、修飾キーとキーを合わせた1つの押し方のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct キーの組(egui::KeyboardShortcut);

impl キーの組 {
    /// 修飾キー（Ctrl・Shift・Alt 等）とキーから組を作る。修飾キー無しなら `修飾キー::NONE`。
    pub const fn 生成する(修飾キー: egui::Modifiers, キー: egui::Key) -> Self {
        Self(egui::KeyboardShortcut::new(修飾キー, キー))
    }

    /// 修飾キーを押さない単独のキーの組。
    pub const fn 単独(キー: egui::Key) -> Self {
        Self::生成する(egui::Modifiers::NONE, キー)
    }

    fn 修飾キーを含むか(self) -> bool {
        let 修飾 = self.0.modifiers;
        修飾.ctrl || 修飾.command || 修飾.mac_cmd || 修飾.alt
    }

    /// 厳密に一致する押下の事象を1つ消費し、あったかを返す。
    fn 押下を消費する(self, 入力: &mut egui::InputState) -> bool {
        let 一致する = |事象: &egui::Event| {
            matches!(事象, egui::Event::Key { key, modifiers, pressed: true, .. }
                if *key == self.0.logical_key && modifiers.matches_exact(self.0.modifiers))
        };
        let Some(位置) = 入力.events.iter().position(一致する) else {
            return false;
        };
        入力.events.remove(位置);
        true
    }
}

/// 入力欄との関係とは、入力欄にフォーカスがある間にこのキー操作を見張るかの区別のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum 入力欄との関係 {
    修飾キーの組なら入力欄から奪う,
    入力欄に任せる,
}

/// キー操作型とは、キーの組と押されたときの応答と、入力欄にフォーカスがある間の振る舞いの組の記述のことである。
pub struct キー操作型<M> {
    キーの組: キーの組,
    応答: M,
    入力欄との関係: 入力欄との関係,
}

impl<M> キー操作型<M> {
    pub(crate) fn 新規(組: キーの組, 応答: M) -> Self {
        Self {
            キーの組: 組,
            応答,
            入力欄との関係:
                入力欄との関係::修飾キーの組なら入力欄から奪う,
        }
    }

    /// 修飾キーを含む組でも、入力欄にフォーカスがある間は発せず、押下の事象も消費しない。
    /// 入力欄が自分で扱う組(Ctrl+Z の取り消し等)を、入力欄の編集の間は入力欄に任せるときに使う。
    pub fn 入力欄にフォーカスがある間は発しない(mut self) -> Self {
        self.入力欄との関係 = 入力欄との関係::入力欄に任せる;
        self
    }

    /// 文字を打つ部品にフォーカスがある間に、この組を見張らないか。
    fn 入力欄に譲るか(&self, 打っているか: bool) -> bool {
        打っているか
            && (self.入力欄との関係 == 入力欄との関係::入力欄に任せる
                || !self.キーの組.修飾キーを含むか())
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> キー操作型<N> {
        キー操作型 {
            キーの組: self.キーの組,
            応答: 応答を変換する(self.応答),
            入力欄との関係: self.入力欄との関係,
        }
    }
}

impl<M: Clone> キー操作型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 保留の識別子 = ui.make_persistent_id(("キー操作の保留", self.キーの組));
        let 保留していた = ui
            .data_mut(|記憶| 記憶.remove_temp::<bool>(保留の識別子))
            .is_some();
        if 保留していた {
            発行した応答.push(self.応答.clone());
            return ui.response();
        }
        if self.入力欄に譲るか(文字を打つ部品にフォーカスがあるか(
            ui.ctx(),
        )) {
            return ui.response();
        }
        if !ui.input_mut(|入力| self.キーの組.押下を消費する(入力)) {
            return ui.response();
        }
        match ui.memory(|記憶| 記憶.focused()) {
            None => 発行した応答.push(self.応答.clone()),
            Some(フォーカスの識別子) => {
                ui.memory_mut(|記憶| 記憶.surrender_focus(フォーカスの識別子));
                ui.data_mut(|記憶| 記憶.insert_temp(保留の識別子, true));
                ui.ctx().request_repaint();
            }
        }
        ui.response()
    }
}

/// フォーカスを持つ部品が、文字を打つ部品（テキスト入力と、直接入力中の数値入力）か。egui の `wants_keyboard_input` は
/// フォーカスを持つ部品があれば真になり、ボタンのフォーカスと区別できないため使わない。
/// egui のテキスト編集は描くたびに自分の状態を識別子の下へ保存するため、状態が残っている識別子はテキスト編集である。
/// Tab でフォーカスを移した直後の回も、移る前の回に描いた時点で状態が保存されているため、最初の打鍵から入力欄に任せる。
fn 文字を打つ部品にフォーカスがあるか(eguiの本体: &egui::Context) -> bool {
    eguiの本体
        .memory(|記憶| 記憶.focused())
        .is_some_and(|識別子| egui::text_edit::TextEditState::load(eguiの本体, 識別子).is_some())
}
