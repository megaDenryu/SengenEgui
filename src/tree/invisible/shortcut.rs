//! キー操作。画面に何も描かず、キーの組が押されたら応答を発行する。振る舞いは次のとおり。
//!
//! - 修飾キーは厳密に一致させる（Ctrl+S は Ctrl+Shift+S では発行しない）。一致した押下の事象は消費する
//! - Ctrl・Command・Alt を含まない組（文字キー・Enter・Backspace 等）は、どこかの部品がキー入力を
//!   欲しがっている間（`wants_keyboard_input`。入力欄にフォーカスがある間）は見張らず、事象も消費しない
//! - 修飾キーを含む組が押されたとき、フォーカスを持つ部品があればそのフォーカスを手放させ、
//!   応答は次の描画の回に発行する。確定時のみ発行の入力欄はフォーカスが外れた回に確定の応答を出すため、
//!   入力欄の確定がキー操作の応答より先に並ぶ。フォーカスを持つ部品が無ければその回に発行する
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

/// キー操作型とは、キーの組と押されたときの応答の組の記述のことである。
pub struct キー操作型<M> {
    キーの組: キーの組,
    応答: M,
}

impl<M> キー操作型<M> {
    pub(crate) fn 新規(組: キーの組, 応答: M) -> Self {
        Self {
            キーの組: 組, 応答
        }
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> キー操作型<N> {
        キー操作型 {
            キーの組: self.キーの組,
            応答: 応答を変換する(self.応答),
        }
    }
}

impl<M: Clone> キー操作型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 保留の鍵 = ui.make_persistent_id(("キー操作の保留", self.キーの組));
        let 保留していた = ui
            .data_mut(|記憶| 記憶.remove_temp::<bool>(保留の鍵))
            .is_some();
        if 保留していた {
            発行した応答.push(self.応答.clone());
            return ui.response();
        }
        if !self.キーの組.修飾キーを含むか() && ui.ctx().wants_keyboard_input() {
            return ui.response();
        }
        if !ui.input_mut(|入力| self.キーの組.押下を消費する(入力)) {
            return ui.response();
        }
        match ui.memory(|記憶| 記憶.focused()) {
            None => 発行した応答.push(self.応答.clone()),
            Some(フォーカスの識別子) => {
                ui.memory_mut(|記憶| 記憶.surrender_focus(フォーカスの識別子));
                ui.data_mut(|記憶| 記憶.insert_temp(保留の鍵, true));
                ui.ctx().request_repaint();
            }
        }
        ui.response()
    }
}
