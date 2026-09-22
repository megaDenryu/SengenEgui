//! 割合。0 を空、1 を満杯とする比率であり、百分率（0〜100）と混同しないために型で区別する。
//! 範囲外の値を黙って丸める口は持たない。丸めるときは `範囲へ丸めて生成する` と名前で明示する。

/// 割合とは、0 以上 1 以下の実数で表した比率のことである。
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 割合(f32);

/// 割合の範囲外とは、割合として受け取れない数値の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum 割合の範囲外 {
    /// 0 より小さい値を渡した。
    負である(f32),
    /// 1 より大きい値を渡した。
    一を超える(f32),
    /// NaN のように大小を比べられない値を渡した。
    数でない,
}

impl 割合 {
    /// 何も満たしていない割合（0）。
    pub const ゼロ: 割合 = 割合(0.0);
    /// 全部を満たした割合（1）。
    pub const 全部: 割合 = 割合(1.0);

    /// 0 以上 1 以下の値から割合を作る。範囲外なら理由を返す。
    pub const fn 生成する(値: f32) -> Result<Self, 割合の範囲外> {
        if 値.is_nan() {
            return Err(割合の範囲外::数でない);
        }
        if 値 < 0.0 {
            return Err(割合の範囲外::負である(値));
        }
        if 値 > 1.0 {
            return Err(割合の範囲外::一を超える(値));
        }
        Ok(Self(値))
    }

    /// 範囲外の値を 0 と 1 へ丸めて割合を作る。NaN は 0 にする。
    pub const fn 範囲へ丸めて生成する(値: f32) -> Self {
        if 値.is_nan() || 値 < 0.0 {
            return Self::ゼロ;
        }
        if 値 > 1.0 {
            return Self::全部;
        }
        Self(値)
    }

    /// egui の API へ渡すための生の値。この口以外で f32 へ戻さない。
    pub const fn eguiへ渡す値(self) -> f32 {
        self.0
    }
}

impl std::fmt::Display for 割合の範囲外 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::負である(値) => {
                write!(出力, "割合は0以上でなければならないが {値} を渡した")
            }
            Self::一を超える(値) => {
                write!(出力, "割合は1以下でなければならないが {値} を渡した")
            }
            Self::数でない => write!(出力, "割合に数でない値を渡した"),
        }
    }
}

impl std::error::Error for 割合の範囲外 {}
