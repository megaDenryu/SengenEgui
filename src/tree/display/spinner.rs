//! 読み込み中表示。処理中であることを回転する円弧で示す。

use crate::measure::論理画素;

/// 読み込み中表示型とは、処理中を回転で示す部品の記述のことである。
pub struct 読み込み中表示型 {
    大きさ指定: Option<論理画素>,
}

impl 読み込み中表示型 {
    pub(crate) fn 新規() -> Self {
        Self {
            大きさ指定: None
        }
    }

    /// 円の直径を指定する。未指定なら周囲の文字の高さに合わせる。
    pub fn 大きさ(mut self, 大きさ: 論理画素) -> Self {
        self.大きさ指定 = Some(大きさ);
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        let mut 部品 = egui::Spinner::new();
        if let Some(大きさ) = self.大きさ指定 {
            部品 = 部品.size(大きさ.eguiへ渡す値());
        }
        ui.add(部品)
    }
}
