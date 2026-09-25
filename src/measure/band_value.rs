//! 帯の値・帯の値の差・帯の全体の範囲。区間の帯が扱う値は利用する側の単位（動画の秒など）で表し、部品はその単位を知らない。
//! 利用する側のドメインが倍精度の小数で計算するため、倍精度の小数のまま持つ。画面へ写すときだけ `割合` へ変える。

use crate::measure::割合;

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

/// 帯の全体の範囲とは、区間の帯の左端と右端が表す値の組のことである。始めは終わりより小さく、どちらも有限である。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct 帯の全体の範囲 {
    始め: 帯の値,
    終わり: 帯の値,
}

/// 帯の全体の範囲の不正とは、帯の全体の範囲として受け取れない値の組の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum 帯の全体の範囲の不正 {
    /// 始めか終わりが無限大か NaN である。
    有限でない,
    /// 始めが終わり以上である。長さ0の範囲は帯の上で値を区別できないため受け取らない。
    始めが終わり以上,
}

impl 帯の全体の範囲 {
    /// 始めと終わりから作る。どちらかが有限でないか、始めが終わり以上なら理由を返す。
    pub fn 生成する(
        始め: 帯の値,
        終わり: 帯の値,
    ) -> Result<Self, 帯の全体の範囲の不正> {
        if !始め.0.is_finite() || !終わり.0.is_finite() {
            return Err(帯の全体の範囲の不正::有限でない);
        }
        if 始め >= 終わり {
            return Err(帯の全体の範囲の不正::始めが終わり以上);
        }
        Ok(Self { 始め, 終わり })
    }

    /// 始めの値。
    pub const fn 始め(self) -> 帯の値 {
        self.始め
    }

    /// 終わりの値。
    pub const fn 終わり(self) -> 帯の値 {
        self.終わり
    }

    /// 値が全体のどこにあるかの割合。範囲の外の値は端へ、NaN は始めへ寄せる。
    pub(crate) fn 割合へ写す(self, 値: 帯の値) -> 割合 {
        割合::倍精度の小数から範囲へ丸めて生成する(
            (値 - self.始め).0 / self.長さ().0,
        )
    }

    /// 全体の長さに比を掛けた差。帯の上の横の移動量を値の差へ変えるときに使う。
    pub(crate) fn 長さに比を掛ける(self, 比: f64) -> 帯の値の差 {
        帯の値の差(self.長さ().0 * 比)
    }

    /// 始めから終わりまでの長さ。
    pub(crate) fn 長さ(self) -> 帯の値の差 {
        self.終わり - self.始め
    }

    /// 値を始めと終わりの間へ収める。NaN は始めにする。
    pub(crate) fn 内側へ収める(self, 値: 帯の値) -> 帯の値 {
        帯の値(値.0.max(self.始め.0).min(self.終わり.0))
    }

    /// 開始から長さの区間が全体の内側に収まるよう、開始を収める。長さが全体より長ければ始めを返す。
    pub(crate) fn 長さを保って開始を収める(
        self,
        開始: 帯の値,
        長さ: 帯の値の差,
    ) -> 帯の値 {
        帯の値(開始.0.min(self.終わり.0 - 長さ.0).max(self.始め.0))
    }
}

impl std::fmt::Display for 帯の全体の範囲の不正 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::有限でない => write!(出力, "帯の全体の範囲の始めか終わりが有限でない"),
            Self::始めが終わり以上 => {
                write!(出力, "帯の全体の範囲の始めが終わり以上である")
            }
        }
    }
}

impl std::error::Error for 帯の全体の範囲の不正 {}
