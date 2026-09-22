//! テーマ。画面全体の見た目（明暗・強調色・地の色・文字色・部品の面の色・表示倍率）を1箇所で決める。
//! 適用しなければ egui の既定（OSの明暗設定への追従）のまま動く。
//! 色の項目は未指定なら基調の既定色を使う。egui の濃色の既定は本文が灰色140で暗いため、
//! 読みやすさを求める画面は `文字色` を指定する。

/// 明暗とは、画面全体の基調の区別のことである。
#[derive(Clone, Copy)]
pub enum 明暗 {
    /// 暗い地に明るい文字を置く基調。
    濃色,
    /// 明るい地に暗い文字を置く基調。
    淡色,
}

/// テーマとは、画面全体へ一括で適用する見た目の指定のことである。
/// ノード単位の装飾はスタイルが担い、テーマは全体の基調だけを担う。
#[derive(Clone, Copy)]
pub struct テーマ {
    /// 画面全体の明暗の基調。
    pub 基調: 明暗,
    /// 選択・リンク等の強調に使う色。
    pub 強調色: Option<egui::Color32>,
    /// 画面と浮きウィンドウの背景の色。
    pub 地の色: Option<egui::Color32>,
    /// 装飾で色を指定していない文字と、ボタン等の部品の文字の色。
    pub 文字色: Option<egui::Color32>,
    /// ボタン・チェックボックス等の、触っていないときの面の色。
    pub 部品の面の色: Option<egui::Color32>,
    /// 画面全体の拡大率。1.0が等倍。
    pub 表示倍率: Option<f32>,
}

impl テーマ {
    /// テーマを egui の文脈へ適用する。起動時に1回呼ぶ。
    pub fn 適用する(&self, 文脈: &egui::Context) {
        let mut 見た目 = match self.基調 {
            明暗::濃色 => egui::Visuals::dark(),
            明暗::淡色 => egui::Visuals::light(),
        };
        self.色を見た目へ写す(&mut 見た目);
        文脈.set_visuals(見た目);
        if let Some(倍率) = self.表示倍率 {
            文脈.set_zoom_factor(倍率);
        }
    }

    fn 色を見た目へ写す(&self, 見た目: &mut egui::Visuals) {
        if let Some(色) = self.強調色 {
            見た目.selection.bg_fill = 色;
            見た目.hyperlink_color = 色;
        }
        if let Some(色) = self.地の色 {
            見た目.panel_fill = 色;
            見た目.window_fill = 色;
            見た目.widgets.noninteractive.bg_fill = 色;
            見た目.widgets.noninteractive.weak_bg_fill = 色;
        }
        if let Some(色) = self.文字色 {
            見た目.widgets.noninteractive.fg_stroke.color = 色;
            見た目.widgets.inactive.fg_stroke.color = 色;
        }
        if let Some(色) = self.部品の面の色 {
            見た目.widgets.inactive.bg_fill = 色;
            見た目.widgets.inactive.weak_bg_fill = 色;
        }
    }
}
