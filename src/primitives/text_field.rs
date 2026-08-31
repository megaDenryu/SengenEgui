//! 一行テキスト入力。現在値はこのフレームの値を渡し、変更は新しい値から応答を作って発行する。
//!
//! 既定は毎キー発行で、描画後に状態へ適用されれば次のフレームで値がつながる。
//! IME変換や高価な適用処理と相性が悪い場面では `確定時のみ発行` を使う。
//! 確定時のみ発行では編集中の下書きを egui の一時記憶に置き、
//! フォーカスが外れた（Enterを含む）ときだけ応答を発行する。

pub struct 入力欄型<M> {
    値: String,
    変更: Box<dyn Fn(String) -> M>,
    幅指定: Option<f32>,
    案内文指定: Option<String>,
    確定時識別子: Option<String>,
}

impl<M> 入力欄型<M> {
    pub(crate) fn 新規(値: String, 変更: Box<dyn Fn(String) -> M>) -> Self {
        Self {
            値,
            変更,
            幅指定: None,
            案内文指定: None,
            確定時識別子: None,
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

    /// 毎キーでなく、フォーカスが外れた（Enterを含む）ときだけ発行する。
    /// 識別子は下書きの置き場の鍵で、同一画面内で重複させない。
    pub fn 確定時のみ発行(mut self, 識別子: impl Into<String>) -> Self {
        self.確定時識別子 = Some(識別子.into());
        self
    }

    fn 部品を組む<'a>(&self, 値: &'a mut String) -> egui::TextEdit<'a> {
        let mut 部品 = egui::TextEdit::singleline(値);
        if let Some(幅) = self.幅指定 {
            部品 = 部品.desired_width(幅);
        }
        if let Some(案内) = &self.案内文指定 {
            部品 = 部品.hint_text(案内.clone());
        }
        部品
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        match &self.確定時識別子 {
            None => self.毎キーで描画する(ui, 集配),
            Some(識別子) => self.確定時のみで描画する(ui, 集配, 識別子),
        }
    }

    fn 毎キーで描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        let mut 値 = self.値.clone();
        if ui.add(self.部品を組む(&mut 値)).changed() {
            集配.push((self.変更)(値));
        }
    }

    fn 確定時のみで描画する(
        &self, ui: &mut egui::Ui, 集配: &mut Vec<M>, 識別子: &str
    ) {
        let 鍵 = ui.make_persistent_id(識別子);
        let mut 下書き = ui
            .data_mut(|記憶| 記憶.get_temp::<String>(鍵))
            .unwrap_or_else(|| self.値.clone());
        let 応答 = ui.add(self.部品を組む(&mut 下書き));
        if 応答.changed() {
            ui.data_mut(|記憶| 記憶.insert_temp(鍵, 下書き.clone()));
        }
        if 応答.lost_focus() {
            ui.data_mut(|記憶| 記憶.remove::<String>(鍵));
            if 下書き != self.値 {
                集配.push((self.変更)(下書き));
            }
        }
    }
}

impl<M: 'static> 入力欄型<M> {
    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N: 'static>(self, 変換: std::rc::Rc<dyn Fn(M) -> N>) -> 入力欄型<N> {
        let 元の変更 = self.変更;
        入力欄型 {
            値: self.値,
            変更: Box::new(move |値| 変換(元の変更(値))),
            幅指定: self.幅指定,
            案内文指定: self.案内文指定,
            確定時識別子: self.確定時識別子,
        }
    }
}
