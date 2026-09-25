//! フォントのバイト列。置き場所のファイルから読んだバイト列のうち、先頭の4バイトが TrueType・OpenType・その集まりの形式を示すものだけを受け取る。
//! egui はフォントでないバイト列を受け取ると、登録した次のフレームで停止する。そのため形式の確認は登録より前に行う。

use std::path::Path;

use super::フォントを使えなかった理由;

/// egui のフォントの一覧へ登録するときの名前。egui は同じ名前のフォントを2回目以降は足さない。
const 登録する名前: &str = "日本語の字形";

/// フォントのバイト列とは、先頭の4バイトでフォントの形式だと確かめたバイト列のことである。
pub(super) struct フォントのバイト列(Vec<u8>);

/// フォントの形式を示す先頭の4バイト。TrueType（0x00010000 と 'true'）・OpenType（'OTTO'）・フォントの集まり（'ttcf'）である。
const フォントの先頭一覧: [[u8; 4]; 4] = [[0, 1, 0, 0], *b"true", *b"OTTO", *b"ttcf"];

impl フォントのバイト列 {
    /// 置き場所のファイルを読み、先頭の4バイトがフォントの形式を示していれば受け取る。
    pub(super) fn 置き場所から読む(
        置き場所: &Path,
    ) -> Result<Self, フォントを使えなかった理由> {
        let バイト列 = std::fs::read(置き場所).map_err(フォントを使えなかった理由::読めなかった)?;
        let 先頭 = バイト列.get(..4);
        if フォントの先頭一覧
            .iter()
            .any(|形式| Some(形式.as_slice()) == 先頭)
        {
            Ok(Self(バイト列))
        } else {
            Err(フォントを使えなかった理由::フォントの形式でない)
        }
    }

    /// egui の文字の族（通常と等幅）の最後へ足す。既定のフォントに無い字形だけがこのフォントで描かれる。
    pub(super) fn 日本語の字形として足す(self, eguiの本体: &egui::Context) {
        let 足す族 = [egui::FontFamily::Proportional, egui::FontFamily::Monospace].map(|族| {
            egui::epaint::text::InsertFontFamily {
                family: 族,
                priority: egui::epaint::text::FontPriority::Lowest,
            }
        });
        eguiの本体.add_font(egui::epaint::text::FontInsert::new(
            登録する名前,
            egui::FontData::from_owned(self.0),
            足す族.to_vec(),
        ));
    }
}
