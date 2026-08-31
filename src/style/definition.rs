//! スタイルの型。全項目が省略可能で、未指定の項目は egui の既定値を使う。

/// スタイルとは、ノードへ適用する装飾の集合のことである。
/// 利用側は `pub const` の名前付き定数として専用ファイルへ集約し、
/// 構造側からは `.装飾(定数名)` で参照する。
#[derive(Clone, Copy, Default)]
pub struct スタイル {
    pub 文字サイズ: Option<f32>,
    pub 文字色: Option<egui::Color32>,
    pub 太字: Option<bool>,
    pub 等幅: Option<bool>,
    /// コンテナでは枠の塗り、ボタンでは面の塗りになる。
    pub 背景色: Option<egui::Color32>,
    pub 内余白: Option<i8>,
    pub 外余白: Option<i8>,
    pub 角丸: Option<u8>,
    /// 枠線を引くときの色。太さ未指定なら1.0で引く。
    pub 枠線色: Option<egui::Color32>,
    pub 枠線太さ: Option<f32>,
}

impl スタイル {
    /// 何も指定しないスタイル。定数定義で `..スタイル::無指定` の形の基底に使う。
    pub const 無指定: スタイル = スタイル {
        文字サイズ: None,
        文字色: None,
        太字: None,
        等幅: None,
        背景色: None,
        内余白: None,
        外余白: None,
        角丸: None,
        枠線色: None,
        枠線太さ: None,
    };

    /// 上書き側の指定を優先して2つのスタイルを合成する。「基本 + 状態差分」の派生に使う。
    /// 自身は変更せず、新しい値を返す。
    pub fn 重ねる(self, 上書き: スタイル) -> スタイル {
        スタイル {
            文字サイズ: 上書き.文字サイズ.or(self.文字サイズ),
            文字色: 上書き.文字色.or(self.文字色),
            太字: 上書き.太字.or(self.太字),
            等幅: 上書き.等幅.or(self.等幅),
            背景色: 上書き.背景色.or(self.背景色),
            内余白: 上書き.内余白.or(self.内余白),
            外余白: 上書き.外余白.or(self.外余白),
            角丸: 上書き.角丸.or(self.角丸),
            枠線色: 上書き.枠線色.or(self.枠線色),
            枠線太さ: 上書き.枠線太さ.or(self.枠線太さ),
        }
    }
}
