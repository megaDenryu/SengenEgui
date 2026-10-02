//! キーの組。修飾キーとキーを合わせた1つの押し方の値であり、キー操作が見張り、キーの組の受け取りが押されたものから作る。
//! 文字の表記との相互の変換は `key_notation.rs` が持つ。

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

    /// 組の修飾キー。
    pub(crate) fn 修飾キー(self) -> egui::Modifiers {
        self.0.modifiers
    }

    /// 組のキー。
    pub(crate) fn キー(self) -> egui::Key {
        self.0.logical_key
    }

    /// 押されたキーと、そのとき押していた修飾キーから組を作る。Ctrl と Command(macOS)はどちらも `修飾キー::COMMAND` にそろえる。
    /// egui は Windows で Ctrl を押すと ctrl と command の両方を立てるため、そのまま組にすると `修飾キー::COMMAND` で書いた組と等しくならないためである。
    pub(crate) fn 押されたものから作る(
        修飾キー: egui::Modifiers,
        キー: egui::Key,
    ) -> Self {
        let そろえた修飾キー = egui::Modifiers {
            alt: 修飾キー.alt,
            shift: 修飾キー.shift,
            command: 修飾キー.command || 修飾キー.ctrl || 修飾キー.mac_cmd,
            ctrl: false,
            mac_cmd: false,
        };
        Self::生成する(そろえた修飾キー, キー)
    }

    pub(crate) fn 修飾キーを含むか(self) -> bool {
        let 修飾 = self.0.modifiers;
        修飾.ctrl || 修飾.command || 修飾.mac_cmd || 修飾.alt
    }

    /// 厳密に一致する押下の事象を1つ消費し、あったかを返す。
    pub(crate) fn 押下を消費する(self, 入力: &mut egui::InputState) -> bool {
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
