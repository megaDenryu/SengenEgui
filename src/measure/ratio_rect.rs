//! 割合で表した矩形。画像の幅と高さを1として、左上を原点に位置と大きさを表す矩形である。
//! 画面上の大きさに依らないため、表示寸法を変えても同じ範囲を指す。

use crate::measure::割合;

/// 割合で表した矩形とは、画像の幅と高さを1として、左端・上端・右端・下端の位置で表した矩形のことである。
/// 左端は右端以下、上端は下端以下であることを生成時に確かめる。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct 割合で表した矩形 {
    左端: 割合,
    上端: 割合,
    右端: 割合,
    下端: 割合,
}

/// 割合で表した矩形の不正とは、矩形として受け取れない端の組の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum 割合で表した矩形の不正 {
    /// 左端が右端より右にある。
    左端が右端より右にある,
    /// 上端が下端より下にある。
    上端が下端より下にある,
}

impl 割合で表した矩形 {
    /// 画像の全体を指す矩形。
    pub const 全体: 割合で表した矩形 = 割合で表した矩形 {
        左端: 割合::ゼロ,
        上端: 割合::ゼロ,
        右端: 割合::全部,
        下端: 割合::全部,
    };

    /// 4つの端から矩形を作る。左端が右端より右か、上端が下端より下なら理由を返す。
    pub fn 生成する(
        左端: 割合,
        上端: 割合,
        右端: 割合,
        下端: 割合,
    ) -> Result<Self, 割合で表した矩形の不正> {
        if 左端 > 右端 {
            return Err(割合で表した矩形の不正::左端が右端より右にある);
        }
        if 上端 > 下端 {
            return Err(割合で表した矩形の不正::上端が下端より下にある);
        }
        Ok(Self {
            左端,
            上端,
            右端,
            下端,
        })
    }

    /// 左端の位置。
    pub const fn 左端(self) -> 割合 {
        self.左端
    }

    /// 上端の位置。
    pub const fn 上端(self) -> 割合 {
        self.上端
    }

    /// 右端の位置。
    pub const fn 右端(self) -> 割合 {
        self.右端
    }

    /// 下端の位置。
    pub const fn 下端(self) -> 割合 {
        self.下端
    }

    /// egui の画像の uv（左上を0・右下を1とした、テクスチャのうち描く範囲）へ渡す矩形。
    pub(crate) fn eguiのuvへ渡す値(self) -> egui::Rect {
        self.画面の矩形へ写す(egui::Rect::from_min_max(
            egui::Pos2::ZERO,
            egui::pos2(1.0, 1.0),
        ))
    }

    /// テクスチャのうちこの矩形が指す部分を、その部分の寸法を持つテクスチャとして返す。
    /// egui は縦横比を保つ計算に元のテクスチャ全体の寸法を使うため、描く部分の寸法へ差し替える。
    pub(crate) fn 描く部分の大きさを持つテクスチャ(
        self,
        テクスチャ: &egui::TextureHandle,
    ) -> egui::load::SizedTexture {
        let 部分の比 = self.eguiのuvへ渡す値().size();
        egui::load::SizedTexture::new(テクスチャ.id(), テクスチャ.size_vec2() * 部分の比)
    }

    /// 画面上の画像の矩形の中で、この矩形が占める画面上の矩形を求める。
    pub(crate) fn 画面の矩形へ写す(self, 画像の矩形: egui::Rect) -> egui::Rect {
        let 横 = |端: 割合| 端.二つの値の間を取る(画像の矩形.left(), 画像の矩形.right());
        let 縦 = |端: 割合| 端.二つの値の間を取る(画像の矩形.top(), 画像の矩形.bottom());
        egui::Rect::from_min_max(
            egui::pos2(横(self.左端), 縦(self.上端)),
            egui::pos2(横(self.右端), 縦(self.下端)),
        )
    }
}

impl std::fmt::Display for 割合で表した矩形の不正 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::左端が右端より右にある => write!(出力, "矩形の左端が右端より右にある"),
            Self::上端が下端より下にある => write!(出力, "矩形の上端が下端より下にある"),
        }
    }
}

impl std::error::Error for 割合で表した矩形の不正 {}
