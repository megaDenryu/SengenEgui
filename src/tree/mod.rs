//! ノード木の中核。ノードは1フレーム分のUI記述であり、毎フレーム組み直して捨てる。
//! 操作は応答型 M の値として集配へ積み、描画後に利用側がまとめて状態へ適用する。

mod scroll;
mod stack;

pub use scroll::スクロール型;
pub use stack::積み型;

pub(crate) use stack::積む向き;

use crate::primitives::{ボタン型, 入力欄型, 切り替え型, 文章型};

/// ノードとは、1フレーム分のUI構造の記述のことである。
/// 型引数 M は操作が発する応答（利用側で定義するメッセージ型）を表す。
pub enum ノード<M> {
    文章(文章型),
    ボタン(ボタン型<M>),
    切り替え(切り替え型<M>),
    入力欄(入力欄型<M>),
    余白(f32),
    区切り線,
    積み(積み型<M>),
    縦スクロール(スクロール型<M>),
    無し,
}

impl<M: Clone> ノード<M> {
    /// 木を描画し、操作が発した応答を集配へ積む。
    pub fn 描画する(&self, ui: &mut egui::Ui, 集配: &mut Vec<M>) {
        match self {
            Self::文章(中身) => 中身.描画する(ui),
            Self::ボタン(中身) => 中身.描画する(ui, 集配),
            Self::切り替え(中身) => 中身.描画する(ui, 集配),
            Self::入力欄(中身) => 中身.描画する(ui, 集配),
            Self::余白(量) => ui.add_space(*量),
            Self::区切り線 => {
                ui.separator();
            }
            Self::積み(中身) => 中身.描画する(ui, 集配),
            Self::縦スクロール(中身) => 中身.描画する(ui, 集配),
            Self::無し => {}
        }
    }

    /// 描画して、発した応答の一覧を返す。アプリの毎フレーム処理の入口。
    pub fn 描画して集める(&self, ui: &mut egui::Ui) -> Vec<M> {
        let mut 集配 = Vec::new();
        self.描画する(ui, &mut 集配);
        集配
    }
}

impl<M> From<文章型> for ノード<M> {
    fn from(値: 文章型) -> Self {
        Self::文章(値)
    }
}

impl<M> From<ボタン型<M>> for ノード<M> {
    fn from(値: ボタン型<M>) -> Self {
        Self::ボタン(値)
    }
}

impl<M> From<切り替え型<M>> for ノード<M> {
    fn from(値: 切り替え型<M>) -> Self {
        Self::切り替え(値)
    }
}

impl<M> From<入力欄型<M>> for ノード<M> {
    fn from(値: 入力欄型<M>) -> Self {
        Self::入力欄(値)
    }
}

impl<M> From<積み型<M>> for ノード<M> {
    fn from(値: 積み型<M>) -> Self {
        Self::積み(値)
    }
}

impl<M> From<スクロール型<M>> for ノード<M> {
    fn from(値: スクロール型<M>) -> Self {
        Self::縦スクロール(値)
    }
}
