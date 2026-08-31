//! 表示文字列の取得手段。SengenImgui の「静的 string / 動的 Func<string>」の対に相当する。

/// 文字列源とは、描画のたびに表示文字列を得る手段の区別のことである。
/// 変わらない値は `固定`、実行中に変わる値は `動的` で持つ。
pub enum 文字列源 {
    固定(String),
    動的(Box<dyn Fn() -> String>),
}

impl 文字列源 {
    /// 現在の表示文字列を返す。`動的` はクロージャを呼んで最新値を取りに行く。
    pub fn 現在値(&self) -> String {
        match self {
            Self::固定(値) => 値.clone(),
            Self::動的(取得) => 取得(),
        }
    }
}

impl From<&str> for 文字列源 {
    fn from(値: &str) -> Self {
        Self::固定(値.to_string())
    }
}

impl From<String> for 文字列源 {
    fn from(値: String) -> Self {
        Self::固定(値)
    }
}
