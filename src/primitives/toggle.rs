//! チェックボックス。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。

use crate::style::スタイル;

pub struct 切り替え型<M> {
    表示: String,
    値: bool,
    変更: Box<dyn Fn(bool) -> M>,
    装飾値: スタイル,
}

impl<M> 切り替え型<M> {
    pub(crate) fn 新規(表示: String, 値: bool, 変更: Box<dyn Fn(bool) -> M>) -> Self {
        Self {
            表示,
            値,
            変更,
            装飾値: スタイル::無指定,
        }
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 値 = self.値;
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.表示.clone()));
        if ui.checkbox(&mut 値, 文字).changed() {
            集配.push((self.変更)(値));
        }
    }
}
