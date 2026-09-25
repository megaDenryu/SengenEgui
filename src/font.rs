//! 日本語フォントの候補と、egui への設定。egui の既定のフォントは日本語の字形を持たないため、
//! 利用する側は起動の部分（窓を作る閉包の中）で1回、OS に入っているフォントを読み込んで足す。
//! ファイルを読むのは `最初に読めたものを設定する` の1箇所だけである。

use std::path::PathBuf;

/// egui のフォントの一覧へ登録するときの名前。egui は同じ名前のフォントを2回目以降は足さない。
const 登録する名前: &str = "日本語の字形";

/// OS に入っている日本語フォントの置き場所。前から順に試す。
/// Windows の游ゴシック・メイリオ・MS ゴシック、Linux の Noto Sans CJK、macOS のヒラギノの順である。
const 標準で入っている置き場所一覧: [&str; 7] = [
    "C:/Windows/Fonts/YuGothM.ttc",
    "C:/Windows/Fonts/meiryo.ttc",
    "C:/Windows/Fonts/msgothic.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
];

/// 日本語フォントの候補とは、日本語の字形を持つフォントのファイルの置き場所を、試す順に並べた一覧のことである。
pub struct 日本語フォントの候補 {
    置き場所一覧: Vec<PathBuf>,
}

/// 日本語フォントが見つからないとは、候補のどのファイルも読めなかったことを、試した置き場所の一覧とともに表す失敗のことである。
#[derive(Debug)]
pub struct 日本語フォントが見つからない {
    試した置き場所一覧: Vec<PathBuf>,
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

    /// 候補を前から順に読み、最初に読めたフォントを egui の文字の族（通常と等幅）の最後へ足す。
    /// 既定のフォントに無い字形だけがこのフォントで描かれる。足したフォントは次のフレームから効く。
    /// どれも読めなければ、試した置き場所を持つ失敗を返す。起動を止めるかどうかは利用する側が決める。
    pub fn 最初に読めたものを設定する(
        &self,
        eguiの本体: &egui::Context,
    ) -> Result<(), 日本語フォントが見つからない> {
        let Some(バイト列) = self
            .置き場所一覧
            .iter()
            .find_map(|置き場所| std::fs::read(置き場所).ok())
        else {
            return Err(日本語フォントが見つからない {
                試した置き場所一覧: self.置き場所一覧.clone(),
            });
        };
        let 足す族 = [egui::FontFamily::Proportional, egui::FontFamily::Monospace].map(|族| {
            egui::epaint::text::InsertFontFamily {
                family: 族,
                priority: egui::epaint::text::FontPriority::Lowest,
            }
        });
        eguiの本体.add_font(egui::epaint::text::FontInsert::new(
            登録する名前,
            egui::FontData::from_owned(バイト列),
            足す族.to_vec(),
        ));
        Ok(())
    }
}

impl std::fmt::Display for 日本語フォントが見つからない {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let 一覧: Vec<String> = self
            .試した置き場所一覧
            .iter()
            .map(|置き場所| format!("「{}」", 置き場所.display()))
            .collect();
        write!(
            出力,
            "日本語フォントを読めなかった。試した置き場所: {}",
            一覧.join("、")
        )
    }
}

impl std::error::Error for 日本語フォントが見つからない {}
