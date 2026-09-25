//! 強調色の地の上の色。egui は選ばれた項目（選択ボタンと選択欄の一覧の選ばれた項目）の文字を選択の縁の色で描き、
//! 入力欄の範囲選択の地を強調色で塗る。選択の縁の色は入力欄のフォーカスの枠線にも使われ、テーマは強調色と文字色を
//! 半々に混ぜて導くため明るく、明るい強調色の地の上の文字が読めない。選択の縁の色は変えずに、選ばれた項目の文字にだけ
//! 別の色を当て、入力欄の範囲選択の地だけを読める色へ差し替える。
//! egui の見た目にはこの2つの色の項目が無いため、テーマを適用するときに egui の一時記憶へ置き、部品が描くときに読む。

use egui::Color32;

/// 強調色まわりの色とは、テーマが決めた、選ばれた項目の文字の色と入力欄の範囲選択の地の色の組のことである。
/// どちらも None なら egui の既定の描き方のまま描く。
#[derive(Clone, Copy, Default)]
pub(crate) struct 強調色まわりの色 {
    pub(super) 選ばれた項目の文字色: Option<Color32>,
    pub(super) 入力欄の範囲選択の地: Option<Color32>,
}

impl 強調色まわりの色 {
    fn 鍵() -> egui::Id {
        egui::Id::new("sengen_egui_強調色まわりの色")
    }

    /// egui の一時記憶へ置く。テーマの適用から呼ぶ。
    pub(super) fn 置く(self, 文脈: &egui::Context) {
        文脈.data_mut(|記憶域| 記憶域.insert_temp(Self::鍵(), self));
    }

    /// egui の一時記憶から読む。テーマを適用していなければ、どちらも None の組を返す。
    pub(crate) fn 読む(ui: &egui::Ui) -> Self {
        ui.data(|記憶域| 記憶域.get_temp::<Self>(Self::鍵()))
            .unwrap_or_default()
    }

    /// 選ばれた項目の文字。選ばれていて文字色が決まっていればその色を付け、それ以外は egui の既定の色で描く文字にする。
    pub(crate) fn 選択できる項目の文字(
        self,
        文字列: &str,
        選ばれている: bool,
    ) -> egui::WidgetText {
        match self.選ばれた項目の文字色 {
            Some(色) if 選ばれている => egui::RichText::new(文字列).color(色).into(),
            _ => 文字列.into(),
        }
    }

    /// 入力欄を、範囲選択の地の色を差し替えた内側の領域で描く。差し替えは内側の領域だけに効き、後に並ぶ部品へ漏れない。
    pub(crate) fn 入力欄を描く(
        self,
        ui: &mut egui::Ui,
        部品: egui::TextEdit<'_>,
    ) -> egui::Response {
        let Some(地) = self.入力欄の範囲選択の地 else {
            return ui.add(部品);
        };
        ui.scope(|内側| {
            内側.visuals_mut().selection.bg_fill = 地;
            内側.add(部品)
        })
        .inner
    }
}
