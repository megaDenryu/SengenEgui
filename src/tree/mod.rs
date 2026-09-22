//! ノード木の中核。ノードは1フレーム分のUI記述であり、毎フレーム組み直して捨てる。
//! 操作は応答型 M の値として発行した応答へ積み、描画後に利用側がまとめて状態へ適用する。
//!
//! 部品は族（同じ種類の部品のまとまり）ごとのモジュールに置く。族のモジュールは
//! 族の列挙と、その `描画する`・`写す`・`From` を持つ。部品を1つ足すときに触るのは
//! 部品自身のファイル・族のファイル・糖衣ファクトリの族のファイルの3つである。

/// 部品の型から族の列挙を経て [`ノード`] へ変換する `From` を生やす。
/// 族のファイルで部品ごとに1行書く。
macro_rules! ノードへ変換する {
    ($族の枝:ident, $族:ident :: $枝:ident, $部品:ty) => {
        impl<M> From<$部品> for $crate::tree::ノード<M> {
            fn from(値: $部品) -> Self {
                Self::$族の枝($族::$枝(値))
            }
        }
    };
}

pub mod attach;
pub mod container;
pub mod display;
pub mod input;
pub mod invisible;
mod map;
mod node;

pub use node::ノード;

/// 子ノード列を順に描画する。各容器の `描画する` が使う共通部。
pub(crate) fn 子を順に描画する<M: Clone>(
    子一覧: &[ノード<M>],
    ui: &mut egui::Ui,
    発行した応答: &mut Vec<M>,
) {
    for 子 in 子一覧 {
        子.描画する(ui, 発行した応答);
    }
}
