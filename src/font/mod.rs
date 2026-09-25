//! 日本語フォントの候補と、egui への設定。egui の既定のフォントは日本語の字形を持たないため、
//! 利用する側は起動の部分（窓を作る閉包の中）で1回、OS に入っているフォントを読み込んで足す。
//! ファイルを読むのは `フォントのバイト列::置き場所から読む` の1箇所だけである。

mod bytes;
mod failure;

use std::path::PathBuf;

use bytes::フォントのバイト列;
pub use failure::{
    フォントを使えなかった理由, 使えなかった置き場所, 日本語フォントが見つからない
};

/// OS に入っている日本語フォントの置き場所。前から順に試す。
/// Windows の游ゴシック・メイリオ・MS ゴシック、Linux の Noto Sans CJK、macOS のヒラギノ角ゴシックの順である。
const 標準で入っている置き場所一覧: [&str; 6] = [
    "C:/Windows/Fonts/YuGothM.ttc",
    "C:/Windows/Fonts/meiryo.ttc",
    "C:/Windows/Fonts/msgothic.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
];

/// 日本語フォントの候補とは、日本語の字形を持つフォントのファイルの置き場所を、試す順に並べた一覧のことである。
pub struct 日本語フォントの候補 {
    置き場所一覧: Vec<PathBuf>,
}

impl 日本語フォントの候補 {
    /// Windows・Linux・macOS に標準で入っている日本語フォントの置き場所を候補にする。
    pub fn 標準で入っている候補() -> Self {
        Self::置き場所の一覧から作る(標準で入っている置き場所一覧)
    }

    /// 利用する側が選んだフォントのファイルの置き場所を、試す順に並べて候補にする。
    pub fn 置き場所の一覧から作る(
        置き場所一覧: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Self {
        Self {
            置き場所一覧: 置き場所一覧.into_iter().map(Into::into).collect(),
        }
    }

    /// 候補を前から順に読み、読めて先頭のバイトがフォントの形式を示す最初のファイルを、egui の文字の族（通常と等幅）の最後へ足す。
    /// 既定のフォントに無い字形だけがこのフォントで描かれる。足したフォントは次のフレームから効く。
    /// どれも使えなければ、試した置き場所と理由を持つ失敗を返す。起動を止めるかどうかは利用する側が決める。
    ///
    /// 注意: 同じ egui の本体へ2回目以降に呼ぶと、egui は同じ名前のフォントを足さないため何も変わらずに Ok を返す。
    /// 別のフォントへ差し替える口ではない。
    pub fn 最初に読めたものを設定する(
        &self,
        eguiの本体: &egui::Context,
    ) -> Result<(), 日本語フォントが見つからない> {
        let mut 試した結果一覧 = Vec::new();
        for 置き場所 in &self.置き場所一覧 {
            match フォントのバイト列::置き場所から読む(置き場所) {
                Ok(フォント) => {
                    フォント.日本語の字形として足す(eguiの本体);
                    return Ok(());
                }
                Err(理由) => 試した結果一覧.push(使えなかった置き場所 {
                    置き場所: 置き場所.clone(),
                    理由,
                }),
            }
        }
        Err(日本語フォントが見つからない {
            試した結果一覧
        })
    }
}
