//! 拡大率。画面全体の拡大の倍率であり、1 が等倍である。0 以下と非有限の値は作れない。
//! 割合（0〜1 の比率）とは別の量であり、混同しないために型で区別する。

/// 拡大率とは、等倍を 1 とする画面全体の拡大の倍率のことである。
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 拡大率(f32);

/// 拡大率の範囲外とは、拡大率として受け取れない数値の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum 拡大率の範囲外 {
    /// 0 以下の値を渡した。
    正でない(f32),
    /// NaN や無限大のように有限でない値を渡した。
    有限でない,
}

impl 拡大率 {
    /// 等倍（1.0）。
    pub const 等倍: 拡大率 = 拡大率(1.0);

    /// 正の有限な値から拡大率を作る。範囲外なら理由を返す。
    pub const fn 生成する(値: f32) -> Result<Self, 拡大率の範囲外> {
        if !値.is_finite() {
            return Err(拡大率の範囲外::有限でない);
        }
        if 値 <= 0.0 {
            return Err(拡大率の範囲外::正でない(値));
        }
        Ok(Self(値))
    }

    /// 範囲外の値を等倍に置き換えて拡大率を作る。
    pub const fn 範囲外なら等倍にして生成する(値: f32) -> Self {
        match Self::生成する(値) {
            Ok(値) => 値,
            Err(_) => Self::等倍,
        }
    }

    /// egui の API へ渡すための生の値。この口以外で f32 へ戻さない。
    pub const fn eguiへ渡す値(self) -> f32 {
        self.0
    }
}

impl Default for 拡大率 {
    fn default() -> Self {
        Self::等倍
    }
}

impl std::fmt::Display for 拡大率の範囲外 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::正でない(値) => {
                write!(出力, "拡大率は0より大きくなければならないが {値} を渡した")
            }
            Self::有限でない => write!(出力, "拡大率に有限でない値を渡した"),
        }
    }
}

impl std::error::Error for 拡大率の範囲外 {}
