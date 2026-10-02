//! キーの組の生成が Ctrl と Command をそろえることを確かめる。`生成する` で `修飾キー::CTRL` から作った組・`修飾キー::MAC_CMD` から作った組・
//! ctrl と command の両方を立てた修飾キーから作った組が、`修飾キー::COMMAND` から作った組と等しく、同じ表記になることと、
//! 定数の文脈でも作れることと、組のキーを修飾キーに関わらず取り出せることを確かめる。

use sengen_egui::{キー, キーの組, 修飾キー};

/// 定数の文脈で作った組。利用する側が既定のキーを const で書けることを確かめる。
const 定数で作った組: キーの組 = キーの組::生成する(修飾キー::CTRL, キー::S);

#[test]
fn ctrlとcommandのどれから作っても同じ組になる() {
    let 基準 = キーの組::生成する(修飾キー::COMMAND, キー::S);
    let windowsのctrl = 修飾キー {
        ctrl: true,
        command: true,
        ..修飾キー::NONE
    };
    for 修飾 in [修飾キー::CTRL, 修飾キー::MAC_CMD, windowsのctrl] {
        let 組 = キーの組::生成する(修飾, キー::S);
        assert_eq!(組, 基準, "{修飾:?}");
        assert_eq!(組.表記(), "Ctrl+S");
    }
    assert_eq!(定数で作った組, 基準);
}

#[test]
fn shiftとaltはそろえずに残す() {
    let シフト付き = 修飾キー {
        shift: true,
        ..修飾キー::CTRL
    };
    assert_ne!(
        キーの組::生成する(シフト付き, キー::Z),
        キーの組::生成する(修飾キー::CTRL, キー::Z)
    );
    assert_eq!(
        キーの組::生成する(シフト付き, キー::Z).表記(),
        "Ctrl+Shift+Z"
    );
    assert_eq!(キーの組::生成する(修飾キー::ALT, キー::Z).表記(), "Alt+Z");
}

#[test]
fn 組のキーは修飾キーに関わらず取り出せる() {
    assert_eq!(
        キーの組::生成する(修飾キー::SHIFT, キー::Escape).キー(),
        キー::Escape
    );
    assert_eq!(キーの組::単独(キー::Escape).キー(), キー::Escape);
}
