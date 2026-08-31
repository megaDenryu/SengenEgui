//! スタイルの egui への適用。文字の装飾と、背景・余白・枠線を持つ枠での包み込みの2系統。

use crate::style::スタイル;

impl スタイル {
    /// 文字系の指定を egui の文字表現へ写す。
    pub(crate) fn 文字へ適用する(&self, 文字: egui::RichText) -> egui::RichText {
        let mut 結果 = 文字;
        if let Some(大きさ) = self.文字サイズ {
            結果 = 結果.size(大きさ);
        }
        if let Some(色) = self.文字色 {
            結果 = 結果.color(色);
        }
        if self.太字 == Some(true) {
            結果 = 結果.strong();
        }
        if self.等幅 == Some(true) {
            結果 = 結果.monospace();
        }
        結果
    }

    /// 枠系の指定があれば枠で包んで中身を描画し、無ければそのまま描画する。
    pub(crate) fn 枠で包んで描画する(
        &self,
        ui: &mut egui::Ui,
        中身: impl FnOnce(&mut egui::Ui),
    ) {
        if !self.枠が要る() {
            中身(ui);
            return;
        }
        self.枠を組む().show(ui, |内側| 中身(内側));
    }

    fn 枠が要る(&self) -> bool {
        self.背景色.is_some()
            || self.内余白.is_some()
            || self.外余白.is_some()
            || self.角丸.is_some()
            || self.枠線色.is_some()
    }

    fn 枠を組む(&self) -> egui::Frame {
        let mut 枠 = egui::Frame::new();
        if let Some(色) = self.背景色 {
            枠 = 枠.fill(色);
        }
        if let Some(余白) = self.内余白 {
            枠 = 枠.inner_margin(egui::Margin::same(余白));
        }
        if let Some(余白) = self.外余白 {
            枠 = 枠.outer_margin(egui::Margin::same(余白));
        }
        if let Some(丸み) = self.角丸 {
            枠 = 枠.corner_radius(egui::CornerRadius::same(丸み));
        }
        if let Some(色) = self.枠線色 {
            let 太さ = self.枠線太さ.unwrap_or(1.0);
            枠 = 枠.stroke(egui::Stroke::new(太さ, 色));
        }
        枠
    }
}
