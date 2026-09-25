//! 画素数の寸法。画像の幅と高さを画素の個数で数えた値であり、論理画素（画面上の長さ）と混同しないために型で分ける。

/// 画素数の寸法とは、画像の幅と高さを画素の個数で表した組のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct 画素数の寸法 {
    幅: u32,
    高さ: u32,
}

impl 画素数の寸法 {
    /// 幅と高さの画素の個数から寸法を作る。
    pub const fn 生成する(幅: u32, 高さ: u32) -> Self {
        Self { 幅, 高さ }
    }

    /// 幅の画素の個数。
    pub const fn 幅(self) -> u32 {
        self.幅
    }

    /// 高さの画素の個数。
    pub const fn 高さ(self) -> u32 {
        self.高さ
    }

    /// 幅か高さが0であるか。
    pub const fn 空であるか(self) -> bool {
        self.幅 == 0 || self.高さ == 0
    }

    /// 1画素につき4バイトで並べたときの全体のバイト数。usize に収まらなければ None を返す。
    pub(crate) fn 四バイトの画素で並べたバイト数(self) -> Option<usize> {
        let 画素の個数 = u64::from(self.幅).checked_mul(u64::from(self.高さ))?;
        usize::try_from(画素の個数.checked_mul(4)?).ok()
    }

    /// egui の画像の寸法（幅・高さの順の usize の組）。usize が32ビットより狭い環境で収まらなければ None を返す。
    pub(crate) fn eguiへ渡す値(self) -> Option<[usize; 2]> {
        Some([
            usize::try_from(self.幅).ok()?,
            usize::try_from(self.高さ).ok()?,
        ])
    }
}

impl std::fmt::Display for 画素数の寸法 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(出力, "{}x{}画素", self.幅, self.高さ)
    }
}
