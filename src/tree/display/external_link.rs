//! 外部リンク。押されるとブラウザで行き先を開く。
//!
//! 開く動作は egui 自身（`egui::Hyperlink`）が行うため、応答を発行せず利用側の状態も触らない。
//! これは中核の方針2（操作は応答として集める）の例外であり、アプリ内で処理すべき押下は
//! `リンク`（応答を発する入力の部品）を使う。

/// 外部リンク型とは、ブラウザで行き先を開くリンクの記述のことである。
pub struct 外部リンク型 {
    表示文字列: String,
    行き先: String,
}

impl 外部リンク型 {
    pub(crate) fn 新規(表示文字列: String, 行き先: String) -> Self {
        Self {
            表示文字列, 行き先
        }
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::Hyperlink::from_label_and_url(
            self.表示文字列.clone(),
            self.行き先.clone(),
        ))
    }
}
