//! 一行テキスト入力。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。
//! 発行の様式（毎キー・確定時のみ）は `text_common` が担う。

use std::rc::Rc;

use crate::measure::論理画素;
use crate::tree::input::text_common::テキスト入力の共通;

/// 一行テキスト入力型とは、1行の文字列を編集する部品の記述のことである。
pub struct 一行テキスト入力型<M> {
    共通: テキスト入力の共通<M>,
    伏せ字指定: bool,
}

impl<M> 一行テキスト入力型<M> {
    pub(crate) fn 新規(
        値: String, 新しい値から応答を作る: Box<dyn Fn(String) -> M>
    ) -> Self {
        Self {
            共通: テキスト入力の共通::新規(値, 新しい値から応答を作る),
            伏せ字指定: false,
        }
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

    /// 毎キーでなく、フォーカスが外れた（Enterを含む）ときだけ発行する。
    /// 識別子は下書きの置き場の鍵で、同一画面内で重複させない。
    pub fn 確定時のみ発行(mut self, 識別子: impl Into<String>) -> Self {
        self.共通.確定時識別子を指定する(識別子.into());
        self
    }

    /// 入力した文字を伏せ字で表示する。合言葉の入力に使う。
    pub fn 伏せ字(mut self) -> Self {
        self.伏せ字指定 = true;
        self
    }

    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 伏せ字 = self.伏せ字指定;
        self.共通.描画する(ui, 発行した応答, |値| {
            egui::TextEdit::singleline(値).password(伏せ字)
        })
    }
}

impl<M: 'static> 一行テキスト入力型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 一行テキスト入力型<N> {
        一行テキスト入力型 {
            共通: self.共通.写す(応答を変換する),
            伏せ字指定: self.伏せ字指定,
        }
    }
}
