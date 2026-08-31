//! テーマ。画面全体の見た目（明暗・強調色・表示倍率）を1箇所で決める。
//! 適用しなければ egui の既定（OSの明暗設定への追従）のまま動く。

/// 明暗とは、画面全体の基調の区別のことである。
#[derive(Clone, Copy)]
pub enum 明暗 {
    濃色,
    淡色,
}

/// テーマとは、画面全体へ一括で適用する見た目の指定のことである。
/// ノード単位の装飾はスタイルが担い、テーマは全体の基調だけを担う。
#[derive(Clone, Copy)]
pub struct テーマ {
    pub 基調: 明暗,
    /// 選択・リンク等の強調に使う色。
    pub 強調色: Option<egui::Color32>,
    /// 画面全体の拡大率。1.0が等倍。
    pub 表示倍率: Option<f32>,
}

impl テーマ {
    pub fn 適用する(&self, 文脈: &egui::Context) {
        let mut 見た目 = match self.基調 {
            明暗::濃色 => egui::Visuals::dark(),
            明暗::淡色 => egui::Visuals::light(),
        };
        if let Some(色) = self.強調色 {
            見た目.selection.bg_fill = 色;
            見た目.hyperlink_color = 色;
        }
        文脈.set_visuals(見た目);
        if let Some(倍率) = self.表示倍率 {
            文脈.set_zoom_factor(倍率);
        }
    }
}
