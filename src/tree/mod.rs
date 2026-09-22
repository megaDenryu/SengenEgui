//! ノード木の中核。ノードは1フレーム分のUI記述であり、毎フレーム組み直して捨てる。
//! 操作は応答型 M の値として発行した応答へ積み、描画後に利用側がまとめて状態へ適用する。

mod collapsing;
mod convert;
mod grid;
mod map;
mod scroll;
mod stack;
mod virtual_list;
mod window;

pub use collapsing::折り畳み見出し型;
pub use grid::格子型;
pub use scroll::スクロール型;
pub use stack::積み型;
pub use virtual_list::仮想列型;
pub use window::ウィンドウ型;

pub(crate) use stack::積む向き;

use crate::primitives::{
    チェックボックス型, ボタン型, 一行テキスト入力型, 文字表示型
};

/// ノードとは、1フレーム分のUI構造の記述のことである。
/// 型引数 M は操作が発する応答（利用側で定義するメッセージ型）を表す。
pub enum ノード<M> {
    文字表示(文字表示型),
    ボタン(ボタン型<M>),
    チェックボックス(チェックボックス型<M>),
    一行テキスト入力(一行テキスト入力型<M>),
    余白(f32),
    区切り線,
    積み(積み型<M>),
    縦スクロール(スクロール型<M>),
    仮想列(仮想列型<M>),
    折り畳み見出し(折り畳み見出し型<M>),
    格子(格子型<M>),
    ウィンドウ(ウィンドウ型<M>),
    無し,
}

impl<M: Clone> ノード<M> {
    /// 木を描画し、操作が発した応答を引数の `発行した応答` へ積む。
    pub fn 描画する(&self, ui: &mut egui::Ui, 発行した応答: &mut Vec<M>) {
        match self {
            Self::文字表示(中身) => 中身.描画する(ui),
            Self::ボタン(中身) => 中身.描画する(ui, 発行した応答),
            Self::チェックボックス(中身) => 中身.描画する(ui, 発行した応答),
            Self::一行テキスト入力(中身) => 中身.描画する(ui, 発行した応答),
            Self::余白(量) => ui.add_space(*量),
            Self::区切り線 => {
                ui.separator();
            }
            Self::積み(中身) => 中身.描画する(ui, 発行した応答),
            Self::縦スクロール(中身) => 中身.描画する(ui, 発行した応答),
            Self::仮想列(中身) => 中身.描画する(ui, 発行した応答),
            Self::折り畳み見出し(中身) => 中身.描画する(ui, 発行した応答),
            Self::格子(中身) => 中身.描画する(ui, 発行した応答),
            Self::ウィンドウ(中身) => 中身.描画する(ui, 発行した応答),
            Self::無し => {}
        }
    }

    /// 描画して、発した応答の一覧を返す。アプリの毎フレーム処理の入口。
    pub fn 描画して集める(&self, ui: &mut egui::Ui) -> Vec<M> {
        let mut 発行した応答 = Vec::new();
        self.描画する(ui, &mut 発行した応答);
        発行した応答
    }
}

pub(crate) fn 子を順に描画する<M: Clone>(
    子一覧: &[ノード<M>],
    ui: &mut egui::Ui,
    発行した応答: &mut Vec<M>,
) {
    for 子 in 子一覧 {
        子.描画する(ui, 発行した応答);
    }
}
