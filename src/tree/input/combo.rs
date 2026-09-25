//! 選択欄。押すと選択肢の一覧が開き、1つ選ぶとその選択肢の応答を発行する。
//! 選ばれている選択肢は egui が保持せず、利用側が現在の表示文字列をこのフレームの値として渡す。
//! 一覧の選ばれた項目の文字は、テーマが決めた強調色の上の文字色で描く（`選ばれた項目と範囲選択の色` 参照）。

use crate::measure::論理画素;
use crate::theme::選ばれた項目と範囲選択の色;

/// 選択肢とは、選択欄に並ぶ1項目の、表示文字列と選んだときの応答の組のことである。
pub struct 選択肢<M> {
    表示文字列: String,
    応答: M,
}

impl<M> 選択肢<M> {
    pub(crate) fn 新規(表示文字列: String, 応答: M) -> Self {
        Self {
            表示文字列, 応答
        }
    }
}

/// 選択欄型とは、選択肢の一覧から1つ選ぶ部品の記述のことである。
pub struct 選択欄型<M> {
    識別子: String,
    現在の表示文字列: String,
    選択肢一覧: Vec<選択肢<M>>,
    幅指定: Option<論理画素>,
}

impl<M> 選択欄型<M> {
    pub(crate) fn 新規(
        識別子: String,
        現在の表示文字列: String,
        選択肢一覧: Vec<選択肢<M>>,
    ) -> Self {
        Self {
            識別子,
            現在の表示文字列,
            選択肢一覧,
            幅指定: None,
        }
    }

    /// 閉じているときの欄の幅を指定する。
    pub fn 幅(mut self, 幅: 論理画素) -> Self {
        self.幅指定 = Some(幅);
        self
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> 選択欄型<N> {
        選択欄型 {
            識別子: self.識別子,
            現在の表示文字列: self.現在の表示文字列,
            選択肢一覧: self
                .選択肢一覧
                .into_iter()
                .map(|項目| 選択肢::新規(項目.表示文字列, 応答を変換する(項目.応答)))
                .collect(),
            幅指定: self.幅指定,
        }
    }
}

impl<M: Clone> 選択欄型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let mut 欄 = egui::ComboBox::from_id_salt(self.識別子.clone())
            .selected_text(self.現在の表示文字列.clone());
        if let Some(幅) = self.幅指定 {
            欄 = 欄.width(幅.eguiへ渡す値());
        }
        let 選択の色 = 選ばれた項目と範囲選択の色::一時記憶から読む(ui);
        欄.show_ui(ui, |内側| {
            for 項目 in &self.選択肢一覧 {
                let 選ばれている = 項目.表示文字列 == self.現在の表示文字列;
                let 文字 = 選択の色.選択できる項目の文字(&項目.表示文字列, 選ばれている);
                if 内側.selectable_label(選ばれている, 文字).clicked() {
                    発行した応答.push(項目.応答.clone());
                }
            }
        })
        .response
    }
}
