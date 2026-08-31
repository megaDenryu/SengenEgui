//! 文字表示。木は毎フレーム組み直すので、動的な値も `format!` 等でその場の文字列を渡す。

use crate::style::スタイル;

pub struct 文章型 {
    内容: String,
    装飾値: スタイル,
    折り返し禁止: bool,
}

impl 文章型 {
    pub(crate) fn 新規(内容: String) -> Self {
        Self {
            内容,
            装飾値: スタイル::無指定,
            折り返し禁止: false,
        }
    }

    /// 装飾を適用する。装飾は名前付き定数として構造の外で定義する（lib.rs 方針3）。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    /// 折り返さず、幅が足りなければ末尾を省略記号で切り詰める。
    /// 仮想化リストのように行高さが一定である前提の場所で使う。
    pub fn 折り返さない(mut self) -> Self {
        self.折り返し禁止 = true;
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) {
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.内容.clone()));
        if self.折り返し禁止 {
            ui.add(egui::Label::new(文字).truncate());
        } else {
            ui.label(文字);
        }
    }
}
