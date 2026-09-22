//! 数の型を消した入力。数値入力とスライダーは数の型（i32・f32 等）で汎用だが、
//! ノードの列挙は型引数を M しか持てないため、ノードへ変換する時点で描画の手続きを閉包へ
//! 閉じ込めて数の型を消す。閉包は描画のたびに部品を組み、発行する応答を返す。

use std::rc::Rc;

/// 描画する手続きとは、部品を組んで描画し、結果を返す閉包のことである。
pub(crate) type 描画する手続き<M> = Box<dyn Fn(&mut egui::Ui) -> 描画の結果<M>>;

/// 描画の結果とは、描画した範囲の反応と、この描画で発行する応答のことである。
pub(crate) struct 描画の結果<M> {
    pub(crate) 反応: egui::Response,
    pub(crate) 発行する応答: Option<M>,
}

/// 型を消した入力型とは、数の型を閉包へ閉じ込めた入力の部品のことである。
pub struct 型を消した入力型<M> {
    描画する: 描画する手続き<M>,
}

impl<M> 型を消した入力型<M> {
    pub(crate) fn 新規(描画する: 描画する手続き<M>) -> Self {
        Self { 描画する }
    }

    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 結果 = (self.描画する)(ui);
        if let Some(応答) = 結果.発行する応答 {
            発行した応答.push(応答);
        }
        結果.反応
    }
}

impl<M: 'static> 型を消した入力型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 型を消した入力型<N> {
        let 元の描画する = self.描画する;
        型を消した入力型 {
            描画する: Box::new(move |ui| {
                let 結果 = 元の描画する(ui);
                描画の結果 {
                    反応: 結果.反応,
                    発行する応答: 結果.発行する応答.map(&*応答を変換する),
                }
            }),
        }
    }
}
