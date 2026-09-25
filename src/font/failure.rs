//! 日本語フォントを設定できなかったときの失敗。試した置き場所ごとに、読めなかった理由を持つ。

use std::path::{Path, PathBuf};

/// フォントを使えなかった理由とは、1つの置き場所のファイルをフォントとして使えなかった理由の区別のことである。
#[derive(Debug)]
pub enum フォントを使えなかった理由 {
    /// ファイルを読めなかった（置き場所に無い・権限が無い等）。
    読めなかった(std::io::Error),
    /// 読めたが、先頭のバイトが TrueType・OpenType・その集まりの形式を示していない。
    フォントの形式でない,
}

/// 使えなかった置き場所とは、試した置き場所と、そこのファイルを使えなかった理由の組のことである。
#[derive(Debug)]
pub struct 使えなかった置き場所 {
    pub(super) 置き場所: PathBuf,
    pub(super) 理由: フォントを使えなかった理由,
}

impl 使えなかった置き場所 {
    /// 試した置き場所。
    pub fn 置き場所(&self) -> &Path {
        &self.置き場所
    }

    /// 使えなかった理由。
    pub fn 理由(&self) -> &フォントを使えなかった理由 {
        &self.理由
    }
}

/// 日本語フォントが見つからないとは、候補のどのファイルもフォントとして使えなかったことを、試した順の置き場所と理由とともに表す失敗のことである。
#[derive(Debug)]
pub struct 日本語フォントが見つからない {
    pub(super) 試した結果一覧: Vec<使えなかった置き場所>,
}

impl 日本語フォントが見つからない {
    /// 試した置き場所と理由の一覧。候補の順に並ぶ。
    pub fn 試した結果一覧(&self) -> &[使えなかった置き場所] {
        &self.試した結果一覧
    }
}

impl std::fmt::Display for フォントを使えなかった理由 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::読めなかった(原因) => write!(出力, "読めなかった（{原因}）"),
            Self::フォントの形式でない => write!(出力, "フォントの形式でない"),
        }
    }
}

impl std::fmt::Display for 日本語フォントが見つからない {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let 一覧: Vec<String> = self
            .試した結果一覧
            .iter()
            .map(|結果| format!("「{}」は{}", 結果.置き場所.display(), 結果.理由))
            .collect();
        write!(
            出力,
            "日本語フォントを設定できなかった。{}",
            一覧.join("、")
        )
    }
}

impl std::error::Error for 日本語フォントが見つからない {}
