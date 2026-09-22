//! 論理画素。egui のレイアウト座標の単位であり、表示倍率を掛ける前の寸法である。
//! 物理画素（画面の実際のドット）と混同しないために型で区別する。

/// 論理画素とは、egui のレイアウト座標系で測った長さのことである。
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 論理画素(f32);

impl 論理画素 {
    /// 長さ0の論理画素。
    pub const ゼロ: 論理画素 = 論理画素(0.0);

    /// 数値から論理画素を作る。定数の定義でも使えるよう const である。
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// egui の API へ渡すための生の値。この口以外で f32 へ戻さない。
    pub const fn eguiへ渡す値(self) -> f32 {
        self.0
    }

    /// 2つの論理画素のうち大きい方を返す。
    pub fn 大きい方(self, 相手: Self) -> Self {
        Self(self.0.max(相手.0))
    }
}

/// 論理画素を短く書くための生成の口。`画素(8.0)` のように使う。
pub const fn 画素(値: f32) -> 論理画素 {
    論理画素::生成する(値)
}

impl std::ops::Add for 論理画素 {
    type Output = Self;
    fn add(self, 相手: Self) -> Self {
        Self(self.0 + 相手.0)
    }
}

impl std::ops::Sub for 論理画素 {
    type Output = Self;
    fn sub(self, 相手: Self) -> Self {
        Self(self.0 - 相手.0)
    }
}

impl std::ops::Mul<f32> for 論理画素 {
    type Output = Self;
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}

impl From<論理画素> for egui::Margin {
    fn from(値: 論理画素) -> Self {
        egui::Margin::from(値.eguiへ渡す値())
    }
}

impl From<論理画素> for egui::CornerRadius {
    fn from(値: 論理画素) -> Self {
        egui::CornerRadius::from(値.eguiへ渡す値())
    }
}

impl std::fmt::Display for 論理画素 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(出力, "{}px", self.0)
    }
}
