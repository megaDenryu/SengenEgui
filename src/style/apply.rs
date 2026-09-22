//! スタイルの egui への適用。文字の装飾と、背景・余白・枠線を持つ枠での包み込みの2系統。

use crate::measure::画素;
use crate::style::スタイル;

type 文字を飾る手続き = fn(egui::RichText) -> egui::RichText;

impl スタイル {
    /// 文字系の指定を egui の文字表現へ写す。
    pub(crate) fn 文字へ適用する(&self, 文字: egui::RichText) -> egui::RichText {
        let mut 結果 = 文字;
        if let Some(大きさ) = self.文字サイズ {
            結果 = 結果.size(大きさ.eguiへ渡す値());
        }
        if let Some(色) = self.文字色 {
            結果 = 結果.color(色);
        }
        結果 = self.文字の飾りを適用する(結果);
        結果
    }

    fn 文字の飾りを適用する(&self, 文字: egui::RichText) -> egui::RichText {
        let 真偽の飾り一覧: [(Option<bool>, 文字を飾る手続き); 6] = [
            (self.太字, egui::RichText::strong),
            (self.等幅, egui::RichText::monospace),
            (self.斜体, egui::RichText::italics),
            (self.下線, egui::RichText::underline),
            (self.取り消し線, egui::RichText::strikethrough),
            (self.弱い, egui::RichText::weak),
        ];
        真偽の飾り一覧.into_iter().fold(文字, |文字, (指定, 飾る)| {
            if 指定 == Some(true) {
                飾る(文字)
            } else {
                文字
            }
        })
    }

    /// 枠系の指定があれば枠で包んで中身を描画し、無ければそのまま描画する。
    /// 中身が返した反応（枠が無いとき）か、枠の反応（枠があるとき）を返す。
    pub(crate) fn 枠で包んで描画する(
        &self,
        ui: &mut egui::Ui,
        中身: impl FnOnce(&mut egui::Ui) -> egui::Response,
    ) -> egui::Response {
        if !self.枠が要る() {
            return 中身(ui);
        }
        self.枠を組む().show(ui, 中身).response
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
            枠 = 枠.inner_margin(egui::Margin::same(余白.eguiへ渡す値()));
        }
        if let Some(余白) = self.外余白 {
            枠 = 枠.outer_margin(egui::Margin::same(余白.eguiへ渡す値()));
        }
        if let Some(丸み) = self.角丸 {
            枠 = 枠.corner_radius(egui::CornerRadius::same(丸み.eguiへ渡す値()));
        }
        if let Some(色) = self.枠線色 {
            let 太さ = self.枠線太さ.unwrap_or(画素(1.0));
            枠 = 枠.stroke(egui::Stroke::new(太さ.eguiへ渡す値(), 色));
        }
        枠
    }
}
