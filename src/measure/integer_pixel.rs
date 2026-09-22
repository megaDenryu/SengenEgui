//! 整数の論理画素。egui の余白（`Margin`、i8）と角丸（`CornerRadius`、u8）は整数で受けるため、
//! 小数の論理画素を黙って丸めずに、範囲を型で保証した整数の型で受け取る。
//! `生成する` は const であり、範囲外は const の評価でコンパイルエラーになる。
//! 実行時に作るときは `生成を試みる` が Result を返す。

/// 整数画素の範囲外とは、整数の論理画素として受け取れない数値の区別のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 整数画素の範囲外 {
    /// 0 より小さい値を渡した。
    負である(i32),
    /// 型の上限より大きい値を渡した。
    上限を超える {
        /// 渡した値。
        値: i32,
        /// その型の上限。
        上限: i32,
    },
}

/// 余白の画素とは、0 以上 127 以下の整数で表した余白の長さのことである（egui の `Margin` の範囲）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 余白の画素(i8);

impl 余白の画素 {
    /// 0 以上の値から作る。負の値は const の評価で失敗する。
    pub const fn 生成する(値: i8) -> Self {
        assert!(値 >= 0, "余白の画素は0以上でなければならない");
        Self(値)
    }

    /// 実行時の値から作る。0 未満と 127 超は理由を返す。
    pub fn 生成を試みる(値: i32) -> Result<Self, 整数画素の範囲外> {
        if 値 < 0 {
            return Err(整数画素の範囲外::負である(値));
        }
        i8::try_from(値)
            .map(Self)
            .map_err(|_| 整数画素の範囲外::上限を超える {
                値,
                上限: i32::from(i8::MAX),
            })
    }

    /// egui の API へ渡すための生の値。この口以外で整数へ戻さない。
    pub const fn eguiへ渡す値(self) -> i8 {
        self.0
    }
}

/// 角丸の画素とは、0 以上 255 以下の整数で表した角の丸みのことである（egui の `CornerRadius` の範囲）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 角丸の画素(u8);

impl 角丸の画素 {
    /// 0 以上 255 以下の値から作る。u8 の範囲がそのまま型の範囲である。
    pub const fn 生成する(値: u8) -> Self {
        Self(値)
    }

    /// 実行時の値から作る。0 未満と 255 超は理由を返す。
    pub fn 生成を試みる(値: i32) -> Result<Self, 整数画素の範囲外> {
        if 値 < 0 {
            return Err(整数画素の範囲外::負である(値));
        }
        u8::try_from(値)
            .map(Self)
            .map_err(|_| 整数画素の範囲外::上限を超える {
                値,
                上限: i32::from(u8::MAX),
            })
    }

    /// egui の API へ渡すための生の値。この口以外で整数へ戻さない。
    pub const fn eguiへ渡す値(self) -> u8 {
        self.0
    }
}

/// 余白の画素を短く書くための生成の口。`余白画素(8)` のように使う。
pub const fn 余白画素(値: i8) -> 余白の画素 {
    余白の画素::生成する(値)
}

/// 角丸の画素を短く書くための生成の口。`角丸画素(6)` のように使う。
pub const fn 角丸画素(値: u8) -> 角丸の画素 {
    角丸の画素::生成する(値)
}

impl std::fmt::Display for 整数画素の範囲外 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::負である(値) => {
                write!(出力, "整数の画素は0以上でなければならないが {値} を渡した")
            }
            Self::上限を超える { 値, 上限 } => {
                write!(
                    出力,
                    "整数の画素は {上限} 以下でなければならないが {値} を渡した"
                )
            }
        }
    }
}

impl std::error::Error for 整数画素の範囲外 {}
