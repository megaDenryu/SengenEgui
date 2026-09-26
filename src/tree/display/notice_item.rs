//! 通知の回と並べる通知。一定時間で消える通知が、どの通知が新しいかを見分けるための値と、回と文の組である。

/// 通知の回とは、何番目に出した通知かを数える値のことである。新しい通知を出すたびに `次` で進める。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct 通知の回(u64);

impl 通知の回 {
    /// 最初の回。
    pub const fn 最初() -> Self {
        Self(0)
    }

    /// この次の回。
    pub const fn 次(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

/// 並べる通知とは、何番目に出した通知かを数える回と、その文の組のことである。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 並べる通知 {
    pub(super) 回: 通知の回,
    pub(super) 文: String,
}

impl 並べる通知 {
    /// 回と文から作る。
    pub fn 作成する(回: 通知の回, 文: impl Into<String>) -> Self {
        Self {
            回, 文: 文.into()
        }
    }
}
