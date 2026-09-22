//! 落とし先。子を枠で囲み、ドラッグ元から運ばれた値が枠の上で放されたら応答を発行する。
//! 受け取れる値の型はノードへ変換する時点で閉包へ閉じ込める。運ばれているものが受け取れる型なら
//! 枠を強調し、受け取れない型なら枠を淡くする（egui の `dnd_drop_zone` と同じ見た目）。

use std::any::Any;
use std::rc::Rc;

use crate::tree::ノード;

type 落とされた値から応答を作る手続き<M> = Box<dyn Fn(&egui::Response) -> Option<M>>;

/// 落とし先型とは、子と、落とされた値から応答を作る手続きの組の記述のことである。
pub struct 落とし先型<M> {
    子: ノード<M>,
    受け取れる型が運ばれているか: Rc<dyn Fn(&egui::Context) -> bool>,
    落とされた値から応答を作る: 落とされた値から応答を作る手続き<M>,
}

impl<M> 落とし先型<M> {
    pub(crate) fn 新規<運ぶ値: Clone + Any + Send + Sync>(
        子: ノード<M>,
        落とされた値から応答を作る: impl Fn(運ぶ値) -> M + 'static,
    ) -> Self {
        Self {
            子,
            受け取れる型が運ばれているか: Rc::new(|文脈| {
                egui::DragAndDrop::has_payload_of_type::<運ぶ値>(文脈)
            }),
            落とされた値から応答を作る: Box::new(move |反応| {
                反応
                    .dnd_release_payload::<運ぶ値>()
                    .map(|値| 落とされた値から応答を作る((*値).clone()))
            }),
        }
    }
}

impl<M: Clone> 落とし先型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 何か運ばれている = egui::DragAndDrop::has_any_payload(ui.ctx());
        let 受け取れる = (self.受け取れる型が運ばれているか)(ui.ctx());
        let mut 枠 = egui::Frame::new()
            .inner_margin(ui.spacing().menu_margin)
            .begin(ui);
        self.子.描画する(&mut 枠.content_ui, 発行した応答);
        let 反応 = 枠.allocate_space(ui);
        let 見た目 = if 何か運ばれている && 受け取れる && 反応.contains_pointer() {
            ui.visuals().widgets.active
        } else {
            ui.visuals().widgets.inactive
        };
        枠.frame.fill = 見た目.bg_fill;
        枠.frame.stroke = 見た目.bg_stroke;
        if 何か運ばれている && !受け取れる {
            枠.frame.fill = ui.visuals().disable(枠.frame.fill);
            枠.frame.stroke.color = ui.visuals().disable(枠.frame.stroke.color);
        }
        枠.paint(ui);
        if let Some(応答) = (self.落とされた値から応答を作る)(&反応) {
            発行した応答.push(応答);
        }
        反応
    }
}

impl<M: 'static> 落とし先型<M> {
    pub(crate) fn 写す<N: 'static>(
        self,
        応答を変換する: Rc<dyn Fn(M) -> N>,
    ) -> 落とし先型<N> {
        let 元の作る = self.落とされた値から応答を作る;
        let 変換 = Rc::clone(&応答を変換する);
        落とし先型 {
            子: self.子.rcで写す(応答を変換する),
            受け取れる型が運ばれているか: self.受け取れる型が運ばれているか,
            落とされた値から応答を作る: Box::new(move |反応| {
                元の作る(反応).map(&*変換)
            }),
        }
    }
}
