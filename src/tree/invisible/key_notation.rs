//! キーの組の表記。キーの組を「Ctrl+Shift+Z」の形の文字の表記にすることと、表記からキーの組を読むことを持つ。
//! 表記は人に見せる文字と、設定のファイルに保存する文字の両方に使えるよう、読むと同じ組へ戻る形にする。
//!
//! - 修飾キーは「Ctrl+」「Alt+」「Shift+」の順に前へ付ける。Ctrl と Command(macOS)はどちらも「Ctrl+」と書き、読むと `修飾キー::COMMAND` になる
//! - 矢印キーは「←」「→」「↑」「↓」と書く。egui の記号(「⏴」等)は日本語のフォントに字形が無いことがあるためである
//! - それ以外のキーは egui の記号か名前(「[」「Space」「F1」「A」等)で書く。読むときは egui の名前(「ArrowLeft」「OpenBracket」等)も受け付ける

use std::fmt;

use super::key_combo::キーの組;

/// 修飾キーの表記と、表記から読んだときに立てる修飾キー。書くときもこの順に並べる。
const 修飾キーの表記: [(&str, egui::Modifiers); 3] = [
    ("Ctrl+", egui::Modifiers::COMMAND),
    ("Alt+", egui::Modifiers::ALT),
    ("Shift+", egui::Modifiers::SHIFT),
];

/// 矢印キーの表記。
const 矢印キーの表記: [(egui::Key, &str); 4] = [
    (egui::Key::ArrowLeft, "←"),
    (egui::Key::ArrowRight, "→"),
    (egui::Key::ArrowUp, "↑"),
    (egui::Key::ArrowDown, "↓"),
];

/// キーの組の表記の誤りとは、文字の表記からキーの組を読めなかった理由のことである。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum キーの組の表記の誤り {
    /// 修飾キーを除いた残りが、知っているキーの表記でない。中身は読めなかった表記の全体である。
    知らないキー(String),
}

impl fmt::Display for キーの組の表記の誤り {
    fn fmt(&self, 書き手: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::知らないキー(表記) => {
                write!(書き手, "キーの組の表記 {表記:?} のキーを読めない")
            }
        }
    }
}

impl std::error::Error for キーの組の表記の誤り {}

impl キーの組 {
    /// 「Ctrl+Shift+Z」の形の表記にする。`表記から読む` で同じ組へ戻る(Ctrl と Command の区別だけは `修飾キー::COMMAND` にそろう)。
    pub fn 表記(self) -> String {
        let 修飾キー = self.修飾キー();
        let 立っているか = [
            修飾キー.command || 修飾キー.ctrl || 修飾キー.mac_cmd,
            修飾キー.alt,
            修飾キー.shift,
        ];
        let 前置き: String = 修飾キーの表記
            .iter()
            .zip(立っているか)
            .filter(|(_, 立っている)| *立っている)
            .map(|((表記, _), _)| *表記)
            .collect();
        format!("{前置き}{}", キーの表記(self.キー()))
    }

    /// 「Ctrl+Shift+Z」の形の表記からキーの組を読む。修飾キーの前置きはどの順でもよい。
    pub fn 表記から読む(表記: &str) -> Result<Self, キーの組の表記の誤り> {
        let mut 修飾キー = egui::Modifiers::NONE;
        let mut 残り = 表記;
        while let Some((前置きの後, 印)) = 修飾キーの表記
            .iter()
            .find_map(|(前置き, 印)| 残り.strip_prefix(前置き).map(|後| (後, *印)))
            .filter(|(後, _)| !後.is_empty())
        {
            修飾キー |= 印;
            残り = 前置きの後;
        }
        let キー = 表記からキーを読む(残り)
            .ok_or_else(|| キーの組の表記の誤り::知らないキー(表記.to_string()))?;
        Ok(Self::生成する(修飾キー, キー))
    }
}

/// キー1つの表記。矢印キーは矢印の文字、それ以外は egui の記号か名前である。
fn キーの表記(キー: egui::Key) -> &'static str {
    矢印キーの表記
        .iter()
        .find(|(候補, _)| *候補 == キー)
        .map_or_else(|| キー.symbol_or_name(), |(_, 表記)| *表記)
}

/// キー1つの表記からキーを読む。矢印の文字と、egui が読める記号と名前を受け付ける。
fn 表記からキーを読む(表記: &str) -> Option<egui::Key> {
    矢印キーの表記
        .iter()
        .find(|(_, 候補)| *候補 == 表記)
        .map(|(キー, _)| *キー)
        .or_else(|| egui::Key::from_name(表記))
}
