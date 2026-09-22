//! 進捗バー。割合を横棒で示す。割合の値はこのフレームの値を渡す。

use crate::measure::{割合, 論理画素};

/// 進捗バー型とは、割合を横棒で示す部品の記述のことである。
pub struct 進捗バー型 {
    割合: 割合,
    添える文字: Option<String>,
    割合を表示する指定: bool,
    幅指定: Option<論理画素>,
}

impl 進捗バー型 {
    pub(crate) fn 新規(割合: 割合) -> Self {
        Self {
            割合,
            添える文字: None,
            割合を表示する指定: false,
            幅指定: None,
        }
    }

    /// 棒の上に重ねて表示する文字を添える。
    pub fn 文字を添える(mut self, 文字: impl Into<String>) -> Self {
        self.添える文字 = Some(文字.into());
        self
    }

    /// 棒の上に百分率の数値を重ねて表示する。
    pub fn 割合を表示する(mut self) -> Self {
        self.割合を表示する指定 = true;
        self
    }

    /// 棒の幅を指定する。未指定なら利用できる幅いっぱいに広げる。
    pub fn 幅(mut self, 幅: 論理画素) -> Self {
        self.幅指定 = Some(幅);
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        let mut 部品 = egui::ProgressBar::new(self.割合.eguiへ渡す値());
        if let Some(文字) = &self.添える文字 {
            部品 = 部品.text(文字.clone());
        }
        if self.割合を表示する指定 {
            部品 = 部品.show_percentage();
        }
        if let Some(幅) = self.幅指定 {
            部品 = 部品.desired_width(幅.eguiへ渡す値());
        }
        ui.add(部品)
    }
}
