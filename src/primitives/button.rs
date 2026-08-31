//! ボタン。押されたら応答の値を発行する。処理そのものは持たない（lib.rs 方針2）。

use crate::style::スタイル;

pub struct ボタン型<M> {
    表示: String,
    応答: M,
    装飾値: スタイル,
    有効指定: bool,
    最小幅指定: Option<f32>,
}

impl<M> ボタン型<M> {
    pub(crate) fn 新規(表示: String, 応答: M) -> Self {
        Self {
            表示,
            応答,
            装飾値: スタイル::無指定,
            有効指定: true,
            最小幅指定: None,
        }
    }

    /// 有効・無効を指定する。false の間は淡色になり押せない。値はこのフレームの判定を渡す。
    pub fn 有効(mut self, 有効: bool) -> Self {
        self.有効指定 = 有効;
        self
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    pub fn 最小幅(mut self, 幅: f32) -> Self {
        self.最小幅指定 = Some(幅);
        self
    }
}

impl<M: Clone> ボタン型<M> {
    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.表示.clone()));
        let mut 部品 = egui::Button::new(文字);
        if let Some(幅) = self.最小幅指定 {
            部品 = 部品.min_size(egui::vec2(幅, 0.0));
        }
        if ui.add_enabled(self.有効指定, 部品).clicked() {
            集配.push(self.応答.clone());
        }
    }
}
