//! チェックボックスの記述と描画。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。

use crate::style::スタイル;

pub struct チェックボックス型<M> {
    表示文字列: String,
    値: bool,
    新しい値から応答を作る: Box<dyn Fn(bool) -> M>,
    装飾値: スタイル,
}

impl<M> チェックボックス型<M> {
    pub(crate) fn 新規(
        表示文字列: String,
        値: bool,
        新しい値から応答を作る: Box<dyn Fn(bool) -> M>,
    ) -> Self {
        Self {
            表示文字列,
            値,
            新しい値から応答を作る,
            装飾値: スタイル::無指定,
        }
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 発行した応答: &mut Vec<M>) {
        let mut 値 = self.値;
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.表示文字列.clone()));
        if ui.checkbox(&mut 値, 文字).changed() {
            発行した応答.push((self.新しい値から応答を作る)(値));
        }
    }
}

impl<M: 'static> チェックボックス型<M> {
    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: std::rc::Rc<dyn Fn(M) -> N>,
    ) -> チェックボックス型<N> {
        let 元の変更 = self.新しい値から応答を作る;
        チェックボックス型 {
            表示文字列: self.表示文字列,
            値: self.値,
            新しい値から応答を作る: Box::new(move |値| {
                応答を変換する(元の変更(値))
            }),
            装飾値: self.装飾値,
        }
    }
}
