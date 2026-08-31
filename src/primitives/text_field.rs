//! 一行テキスト入力。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。

pub struct 入力欄型<M> {
    値: String,
    変更: Box<dyn Fn(String) -> M>,
    幅指定: Option<f32>,
    案内文指定: Option<String>,
}

impl<M> 入力欄型<M> {
    pub(crate) fn 新規(値: String, 変更: Box<dyn Fn(String) -> M>) -> Self {
        Self {
            値,
            変更,
            幅指定: None,
            案内文指定: None,
        }
    }

    pub fn 幅(mut self, 幅: f32) -> Self {
        self.幅指定 = Some(幅);
        self
    }

    /// 未入力のときに淡色で表示する案内文を設定する。
    pub fn 案内文(mut self, 案内: impl Into<String>) -> Self {
        self.案内文指定 = Some(案内.into());
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 値 = self.値.clone();
        let mut 部品 = egui::TextEdit::singleline(&mut 値);
        if let Some(幅) = self.幅指定 {
            部品 = 部品.desired_width(幅);
        }
        if let Some(案内) = &self.案内文指定 {
            部品 = 部品.hint_text(案内.clone());
        }
        if ui.add(部品).changed() {
            集配.push((self.変更)(値));
        }
    }
}
