//! 見えている行の範囲。仮想縦スクロールがそのフレームに組んだ行の最初と最後の番号であり、
//! 利用する側が見えている行に合わせて仕事の順を決める（見えている行の画像を先に読む等）ために応答として受け取る。

use std::ops::{Range, RangeInclusive};

/// 見えている行の範囲とは、仮想縦スクロールがあるフレームに組んだ行の、最初と最後の行番号の組のことである。
/// 最初は最後以下であり、行が1つも見えていないフレームでは作られない（仮想縦スクロールは発しない）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct 見えている行の範囲 {
    最初: usize,
    最後: usize,
}

impl 見えている行の範囲 {
    /// egui が組む行として渡す半開区間から作る。空の区間なら行が見えていないため作らない。
    pub(crate) fn 組む行の区間から作る(区間: Range<usize>) -> Option<Self> {
        let 最後 = 区間.end.checked_sub(1)?;
        (区間.start <= 最後).then_some(Self {
            最初: 区間.start,
            最後,
        })
    }

    /// 最初に見えている行の番号。
    pub const fn 最初(self) -> usize {
        self.最初
    }

    /// 最後に見えている行の番号。
    pub const fn 最後(self) -> usize {
        self.最後
    }

    /// 最初から最後までの行番号（両端を含む）。
    pub const fn 行番号の並び(self) -> RangeInclusive<usize> {
        self.最初..=self.最後
    }

    /// 行番号が範囲に含まれるか。
    pub const fn 含むか(self, 行番号: usize) -> bool {
        self.最初 <= 行番号 && 行番号 <= self.最後
    }
}
