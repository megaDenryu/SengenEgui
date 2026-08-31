//! 一行テキスト入力。値の読みと書きを別クロージャで渡す双方向バインディング。

use crate::node::ノード;

pub struct 入力欄ノード {
    読み: Box<dyn Fn() -> String>,
    書き: Box<dyn FnMut(String)>,
    幅: Option<f32>,
    案内文: Option<String>,
}

impl 入力欄ノード {
    pub(crate) fn 新規(読み: Box<dyn Fn() -> String>, 書き: Box<dyn FnMut(String)>) -> Self {
        Self {
            読み,
            書き,
            幅: None,
            案内文: None,
        }
    }

    pub fn 幅(mut self, 幅: f32) -> Self {
        self.幅 = Some(幅);
        self
    }

    /// 未入力のときに淡色で表示する案内文を設定する。
    pub fn 案内文(mut self, 案内: impl Into<String>) -> Self {
        self.案内文 = Some(案内.into());
        self
    }
}

impl ノード for 入力欄ノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let mut 値 = (self.読み)();
        let mut 部品 = egui::TextEdit::singleline(&mut 値);
        if let Some(幅) = self.幅 {
            部品 = 部品.desired_width(幅);
        }
        if let Some(案内) = &self.案内文 {
            部品 = 部品.hint_text(案内.clone());
        }
        if ui.add(部品).changed() {
            (self.書き)(値);
        }
    }
}
