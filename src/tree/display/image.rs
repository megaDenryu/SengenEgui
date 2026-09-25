//! 画像の表示。
//!
//! 前提: 出所が URI（`file://`・`https://`・`bytes://`）のとき、egui 本体は画像の復号器を
//! 持たないため、利用側が起動時に読み込み器を導入していなければ表示できない
//! （egui_extras の `install_image_loaders` 等）。読み込み器なしで確実に表示できる出所は
//! `画像の出所::テクスチャ`（利用側が画素から作って egui に登録済みのテクスチャ）だけである。

use crate::measure::{縦横の論理画素, 論理画素};

/// 画像の出所とは、表示する画像をどこから取るかの区別のことである。
#[derive(Clone)]
pub enum 画像の出所 {
    /// 読み込み器が解決する URI。
    URI(String),
    /// egui に登録済みのテクスチャ。
    テクスチャ(egui::TextureHandle),
}

impl From<&str> for 画像の出所 {
    fn from(値: &str) -> Self {
        Self::URI(値.to_string())
    }
}

impl From<String> for 画像の出所 {
    fn from(値: String) -> Self {
        Self::URI(値)
    }
}

impl From<egui::TextureHandle> for 画像の出所 {
    fn from(値: egui::TextureHandle) -> Self {
        Self::テクスチャ(値)
    }
}

/// 画像型とは、画像を表示する部品の記述のことである。
pub struct 画像型 {
    出所: 画像の出所,
    最大幅指定: Option<論理画素>,
    表示寸法指定: Option<縦横の論理画素>,
}

impl 画像型 {
    pub(crate) fn 新規(出所: 画像の出所) -> Self {
        Self {
            出所,
            最大幅指定: None,
            表示寸法指定: None,
        }
    }

    /// 表示する幅の上限を指定する。縦横比は保つ。
    pub fn 最大幅(mut self, 幅: 論理画素) -> Self {
        self.最大幅指定 = Some(幅);
        self
    }

    /// 表示する幅と高さを指定する。縦横比を保って、指定の幅と高さに収まる最大の大きさで描く。
    /// 指定の縦横比が画像と一致するときは、指定の幅と高さにちょうど描く。
    /// 最大幅も指定したときは、幅は最大幅を超えない。
    pub fn 表示寸法(mut self, 寸法: 縦横の論理画素) -> Self {
        self.表示寸法指定 = Some(寸法);
        self
    }

    pub(crate) fn 描画する(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(self.egui部品を組む())
    }

    pub(crate) fn egui部品を組む(&self) -> egui::Image<'static> {
        let mut 部品 = match &self.出所 {
            画像の出所::URI(綴り) => egui::Image::from_uri(綴り.clone()),
            画像の出所::テクスチャ(テクスチャ) => {
                egui::Image::from_texture(テクスチャ)
            }
        };
        if let Some(幅) = self.最大幅指定 {
            部品 = 部品.max_width(幅.eguiへ渡す値());
        }
        if let Some(寸法) = self.表示寸法指定 {
            部品 = 部品.fit_to_exact_size(寸法.eguiへ渡す値());
        }
        部品
    }
}
