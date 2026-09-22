//! 一行と複数行のテキスト入力に共通する部分。値・応答の作り方・幅・案内文・確定時のみ発行を持つ。
//!
//! 既定は毎キー発行で、描画後に状態へ適用されれば次のフレームで値がつながる。
//! IME変換や高価な適用処理と相性が悪い場面では `確定時のみ発行` を使う。
//! 確定時のみ発行では編集中の下書きを egui の一時記憶に置き、
//! フォーカスが外れた（Enterを含む）ときだけ応答を発行する。

use std::rc::Rc;

use crate::measure::論理画素;

pub(super) struct テキスト入力の共通<M> {
    値: String,
    新しい値から応答を作る: Box<dyn Fn(String) -> M>,
    幅指定: Option<論理画素>,
    案内文指定: Option<String>,
    確定時識別子: Option<String>,
}

impl<M> テキスト入力の共通<M> {
    pub(super) fn 新規(
        値: String, 新しい値から応答を作る: Box<dyn Fn(String) -> M>
    ) -> Self {
        Self {
            値,
            新しい値から応答を作る,
            幅指定: None,
            案内文指定: None,
            確定時識別子: None,
        }
    }

    pub(super) fn 幅を指定する(&mut self, 幅: 論理画素) {
        self.幅指定 = Some(幅);
    }

    pub(super) fn 案内文を指定する(&mut self, 案内: String) {
        self.案内文指定 = Some(案内);
    }

    pub(super) fn 確定時識別子を指定する(&mut self, 識別子: String) {
        self.確定時識別子 = Some(識別子);
    }

    /// 幅と案内文を egui の部品へ写す。一行・複数行それぞれの固有の指定は呼び出し側が先に写す。
    fn 共通の指定を適用する<'a>(&self, 部品: egui::TextEdit<'a>) -> egui::TextEdit<'a> {
        let mut 部品 = 部品;
        if let Some(幅) = self.幅指定 {
            部品 = 部品.desired_width(幅.eguiへ渡す値());
        }
        if let Some(案内) = &self.案内文指定 {
            部品 = 部品.hint_text(案内.clone());
        }
        部品
    }

    /// 部品を描画し、値が確定したら応答を発行する。`部品を組む` は編集中の文字列から
    /// 一行または複数行の egui の部品を作る。
    pub(super) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
        部品を組む: impl for<'a> Fn(&'a mut String) -> egui::TextEdit<'a>,
    ) -> egui::Response {
        let Some(識別子) = &self.確定時識別子 else {
            let mut 値 = self.値.clone();
            let 反応 = ui.add(self.共通の指定を適用する(部品を組む(&mut 値)));
            if 反応.changed() {
                発行した応答.push((self.新しい値から応答を作る)(値));
            }
            return 反応;
        };
        let 鍵 = ui.make_persistent_id(識別子);
        let mut 下書き = ui
            .data_mut(|記憶| 記憶.get_temp::<String>(鍵))
            .unwrap_or_else(|| self.値.clone());
        let 反応 = ui.add(self.共通の指定を適用する(部品を組む(&mut 下書き)));
        if 反応.changed() {
            ui.data_mut(|記憶| 記憶.insert_temp(鍵, 下書き.clone()));
        }
        if 反応.lost_focus() {
            ui.data_mut(|記憶| 記憶.remove::<String>(鍵));
            if 下書き != self.値 {
                発行した応答.push((self.新しい値から応答を作る)(下書き));
            }
        }
        反応
    }
}

impl<M: 'static> テキスト入力の共通<M> {
    pub(super) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> テキスト入力の共通<N> {
        let 元の変更 = self.新しい値から応答を作る;
        テキスト入力の共通 {
            値: self.値,
            新しい値から応答を作る: Box::new(move |値| {
                応答を変換する(元の変更(値))
            }),
            幅指定: self.幅指定,
            案内文指定: self.案内文指定,
            確定時識別子: self.確定時識別子,
        }
    }
}
