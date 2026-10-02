//! キーの組。修飾キーとキーを合わせた1つの押し方の値であり、キー操作が見張り、キーの組の受け取りが押されたものから作る。
//! 文字の表記との相互の変換は `key_notation.rs` が持つ。

/// キーの組とは、修飾キーとキーを合わせた1つの押し方のことである。
/// 不変条件: Ctrl と Command(macOS)はどちらも `修飾キー::COMMAND` にそろえて持つ(`ctrl` と `mac_cmd` は立てない)。
/// egui は Windows で Ctrl を押すと ctrl と command の両方を立てるため、そろえないと同じ押し方の組が等しくならないためである。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct キーの組(egui::KeyboardShortcut);

impl キーの組 {
    /// 修飾キー（Ctrl・Shift・Alt 等）とキーから組を作る。修飾キー無しなら `修飾キー::NONE`。
    /// Ctrl と Command はどちらも `修飾キー::COMMAND` にそろえる(`修飾キー::CTRL` で作っても `修飾キー::COMMAND` で作った組と等しい)。
    pub const fn 生成する(修飾キー: egui::Modifiers, キー: egui::Key) -> Self {
        let そろえた修飾キー = egui::Modifiers {
            alt: 修飾キー.alt,
            shift: 修飾キー.shift,
            command: 修飾キー.command || 修飾キー.ctrl || 修飾キー.mac_cmd,
            ctrl: false,
            mac_cmd: false,
        };
        Self(egui::KeyboardShortcut::new(そろえた修飾キー, キー))
    }

    /// 修飾キーを押さない単独のキーの組。
    pub const fn 単独(キー: egui::Key) -> Self {
        Self::生成する(egui::Modifiers::NONE, キー)
    }

    /// 組の修飾キー。Ctrl と Command は `修飾キー::COMMAND` にそろえてある。
    pub(crate) const fn 修飾キー(self) -> egui::Modifiers {
        self.0.modifiers
    }

    /// 組のキー(修飾キーを除いたもの)。利用する側が、修飾キーに関わらず特定のキーを含む組を見分けるときに使う。
    pub const fn キー(self) -> egui::Key {
        self.0.logical_key
    }

    pub(crate) fn 修飾キーを含むか(self) -> bool {
        let 修飾 = self.0.modifiers;
        修飾.command || 修飾.alt
    }

    /// 厳密に一致する押下の事象を1つ消費し、あったかを返す。押下の修飾キーも Ctrl と Command をそろえてから比べる
    /// (Ctrl だけを立てた押下も、ctrl と command の両方を立てた押下も、`修飾キー::COMMAND` の組に一致する)。
    pub(crate) fn 押下を消費する(self, 入力: &mut egui::InputState) -> bool {
        let 一致する = |事象: &egui::Event| {
            matches!(事象, egui::Event::Key { key, modifiers, pressed: true, .. }
                if Self::生成する(*modifiers, *key) == self)
        };
        let Some(位置) = 入力.events.iter().position(一致する) else {
            return false;
        };
        入力.events.remove(位置);
        true
    }
}
