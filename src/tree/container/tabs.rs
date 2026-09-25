//! タブ列。見出しを選択ボタンとして横に並べ、下に区切り線を引く合成部品。
//! 選択中の番号はこのフレームの値を渡し、別の見出しが押されたら番号から作った応答を発行する。
//! タブの中身はこの部品が持たず、利用側が `条件付き表示` 等で番号に応じて出す。
//! 選ばれた見出しの文字は、テーマが決めた強調色の上の文字色で描く（`選ばれた項目と範囲選択の色` 参照）。

use std::rc::Rc;

use crate::theme::選ばれた項目と範囲選択の色;

/// タブ列型とは、見出しの一覧から1つを選ぶ横並びの記述のことである。
pub struct タブ列型<M> {
    見出し一覧: Vec<String>,
    選択中の番号: usize,
    番号から応答を作る: Box<dyn Fn(usize) -> M>,
}

impl<M> タブ列型<M> {
    pub(crate) fn 新規(
        見出し一覧: Vec<String>,
        選択中の番号: usize,
        番号から応答を作る: Box<dyn Fn(usize) -> M>,
    ) -> Self {
        Self {
            見出し一覧,
            選択中の番号,
            番号から応答を作る,
        }
    }

    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 選択の色 = 選ばれた項目と範囲選択の色::一時記憶から読む(ui);
        let 反応 = ui
            .horizontal(|内側| {
                for (番号, 見出し) in self.見出し一覧.iter().enumerate() {
                    let 選ばれている = 番号 == self.選択中の番号;
                    let 文字 = 選択の色.選択できる項目の文字(見出し, 選ばれている);
                    if 内側.selectable_label(選ばれている, 文字).clicked() && !選ばれている
                    {
                        発行した応答.push((self.番号から応答を作る)(番号));
                    }
                }
            })
            .response;
        ui.separator();
        反応
    }
}

impl<M: 'static> タブ列型<M> {
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> タブ列型<N> {
        let 元の作る = self.番号から応答を作る;
        タブ列型 {
            見出し一覧: self.見出し一覧,
            選択中の番号: self.選択中の番号,
            番号から応答を作る: Box::new(move |番号| {
                応答を変換する(元の作る(番号))
            }),
        }
    }
}
