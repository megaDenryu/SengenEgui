//! 帯の値と帯の値の差。区間の帯が扱う値は利用する側の単位（動画の秒など）で表し、部品はその単位を知らない。
//! 利用する側のドメインが倍精度の小数で計算するため、倍精度の小数のまま持つ。

/// 帯の値とは、区間の帯の上の1点を、利用する側の単位で表した値のことである。
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 帯の値(f64);

/// 帯の値の差とは、帯の上の2点の間の向きを持つ距離を、利用する側の単位で表した値のことである。
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 帯の値の差(f64);

impl 帯の値 {
    /// 倍精度の小数から作る。
    pub const fn 倍精度の小数から生成する(値: f64) -> Self {
        Self(値)
    }

    /// 利用する側のドメインが計算に使う倍精度の小数。
    pub const fn 倍精度の小数にする(self) -> f64 {
        self.0
    }
}

impl 帯の値 {
    pub(crate) fn 小さい方(self, 相手: Self) -> Self {
        Self(self.0.min(相手.0))
    }

    pub(crate) fn 大きい方(self, 相手: Self) -> Self {
        Self(self.0.max(相手.0))
    }

    /// 下限と上限の間へ収める。NaN は下限にする。前提: 下限は上限以下である。
    pub(crate) fn 間へ収める(self, 下限: Self, 上限: Self) -> Self {
        Self(self.0.max(下限.0).min(上限.0))
    }
}

impl 帯の値の差 {
    /// 利用する側のドメインが計算に使う倍精度の小数。
    pub const fn 倍精度の小数にする(self) -> f64 {
        self.0
    }
}

impl std::ops::Sub for 帯の値 {
    type Output = 帯の値の差;
    fn sub(self, 相手: Self) -> 帯の値の差 {
        帯の値の差(self.0 - 相手.0)
    }
}

impl std::ops::Add<帯の値の差> for 帯の値 {
    type Output = Self;
    fn add(self, 差: 帯の値の差) -> Self {
        Self(self.0 + 差.0)
    }
}

/// 帯の値の差に比を掛けると、同じ向きで長さが比の倍の差になる。
impl std::ops::Mul<f64> for 帯の値の差 {
    type Output = Self;
    fn mul(self, 比: f64) -> Self {
        Self(self.0 * 比)
    }
}
