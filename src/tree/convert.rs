//! 各ノード種別から直和型への変換。`子!` マクロがこの変換を各子要素に対して呼ぶ。

use crate::primitives::{ボタン型, 入力欄型, 切り替え型, 文章型};
use crate::tree::{
    ウィンドウ型, スクロール型, ノード, 仮想列型, 折り畳み型, 格子型, 積み型
};

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

impl<M> From<仮想列型<M>> for ノード<M> {
    fn from(値: 仮想列型<M>) -> Self {
        Self::仮想列(値)
    }
}

impl<M> From<折り畳み型<M>> for ノード<M> {
    fn from(値: 折り畳み型<M>) -> Self {
        Self::折り畳み(値)
    }
}

impl<M> From<格子型<M>> for ノード<M> {
    fn from(値: 格子型<M>) -> Self {
        Self::格子(値)
    }
}

impl<M> From<ウィンドウ型<M>> for ノード<M> {
    fn from(値: ウィンドウ型<M>) -> Self {
        Self::ウィンドウ(値)
    }
}
