//! 複数行テキスト入力。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。
//! 発行の様式（毎キー・確定時のみ）は `text_common` が担う。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::input::text_common::テキスト入力の共通;

/// 複数行テキスト入力型とは、複数行の文字列を編集する部品の記述のことである。
pub struct 複数行テキスト入力型<M> {
    共通: テキスト入力の共通<M>,
    行数指定: Option<usize>,
    等幅指定: bool,
}

impl<M> 複数行テキスト入力型<M> {
    pub(crate) fn 新規(
        値: String, 新しい値から応答を作る: Box<dyn Fn(String) -> M>
    ) -> Self {
        Self {
            共通: テキスト入力の共通::新規(値, 新しい値から応答を作る),
            行数指定: None,
            等幅指定: false,
        }
    }

    /// 最初に見せる行数を指定する。内容がこれより長ければ欄が伸びる。
    pub fn 行数(mut self, 行数: usize) -> Self {
        self.行数指定 = Some(行数);
        self
    }

    /// 等幅の書体で表示する。コードや表の編集に使う。
    pub fn 等幅(mut self) -> Self {
        self.等幅指定 = true;
        self
    }

    /// 入力欄の幅を指定する。
    pub fn 幅(mut self, 幅: 論理画素) -> Self {
        self.共通.幅を指定する(幅);
        self
    }

    /// 未入力のときに淡色で表示する案内文を設定する。
    pub fn 案内文(mut self, 案内: impl Into<String>) -> Self {
        self.共通.案内文を指定する(案内.into());
        self
    }

    /// 毎キーでなく、フォーカスが外れたときだけ発行する。
    /// 識別子は下書きの置き場の鍵で、同一画面内で重複させない。
    pub fn 確定時のみ発行(mut self, 識別子: impl Into<String>) -> Self {
        self.共通.確定時識別子を指定する(識別子.into());
        self
    }

    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 行数 = self.行数指定;
        let 等幅 = self.等幅指定;
        self.共通.描画する(ui, 発行した応答, |値| {
            let mut 部品 = egui::TextEdit::multiline(値);
            if let Some(行数) = 行数 {
                部品 = 部品.desired_rows(行数);
            }
            if 等幅 {
                部品 = 部品.font(egui::TextStyle::Monospace);
            }
            部品
        })
    }
}

impl<M: 'static> 複数行テキスト入力型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 複数行テキスト入力型<N> {
        複数行テキスト入力型 {
            共通: self.共通.写す(応答を変換する),
            行数指定: self.行数指定,
            等幅指定: self.等幅指定,
        }
    }
}
