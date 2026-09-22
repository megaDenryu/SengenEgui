//! 色選択。押すと色の編集画面が開き、色が変わるたびに新しい色から作った応答を発行する。

use std::rc::Rc;

/// 色選択型とは、色を編集する部品の記述のことである。
pub struct 色選択型<M> {
    現在の色: egui::Color32,
    新しい色から応答を作る: Box<dyn Fn(egui::Color32) -> M>,
}

impl<M> 色選択型<M> {
    pub(crate) fn 新規(
        現在の色: egui::Color32,
        新しい色から応答を作る: Box<dyn Fn(egui::Color32) -> M>,
    ) -> Self {
        Self {
            現在の色,
            新しい色から応答を作る,
        }
    }

    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let mut 色 = self.現在の色;
        let 反応 = ui.color_edit_button_srgba(&mut 色);
        if 反応.changed() {
            発行した応答.push((self.新しい色から応答を作る)(色));
        }
        反応
    }
}

impl<M: 'static> 色選択型<M> {
    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N: 'static>(
        self, 応答を変換する: Rc<dyn Fn(M) -> N>
    ) -> 色選択型<N> {
        let 元の変更 = self.新しい色から応答を作る;
        色選択型 {
            現在の色: self.現在の色,
            新しい色から応答を作る: Box::new(move |色| {
                応答を変換する(元の変更(色))
            }),
        }
    }
}
