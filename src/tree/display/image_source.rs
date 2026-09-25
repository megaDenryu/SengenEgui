//! 画像の出所と、出所になる値からの変換。画像の部品は URI か登録済みのテクスチャから画像を取る。
//! 変換はこの型の側に置き、依存の向きを 部品の木 → テクスチャ の一方向に保つ。

use crate::texture::{差し替えられるテクスチャ, 登録したテクスチャの参照};

/// 画像の出所とは、表示する画像をどこから取るかの区別のことである。
#[derive(Clone)]
pub enum 画像の出所 {
    /// 読み込み器が解決する URI。
    URI(String),
    /// egui に登録済みのテクスチャ。
    テクスチャ(登録したテクスチャの参照),
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
        Self::テクスチャ(登録したテクスチャの参照::from(値))
    }
}

/// `画像(参照)` の形で画像の部品へ渡すための変換。
impl From<登録したテクスチャの参照> for 画像の出所 {
    fn from(値: 登録したテクスチャの参照) -> Self {
        Self::テクスチャ(値)
    }
}

/// `画像(&参照)` の形で、参照を手放さずに画像の部品へ渡すための変換。
impl From<&登録したテクスチャの参照> for 画像の出所 {
    fn from(値: &登録したテクスチャの参照) -> Self {
        Self::テクスチャ(値.clone())
    }
}

/// `画像(&テクスチャ)` の形で画像の部品へ渡すための変換。
impl From<&差し替えられるテクスチャ> for 画像の出所 {
    fn from(値: &差し替えられるテクスチャ) -> Self {
        Self::テクスチャ(値.参照を複製する())
    }
}
