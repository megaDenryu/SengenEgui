//! 選択ボタン。選ばれている間は面が強調色で塗られるボタンで、タブや切り替えの見た目に使う。
//! 選ばれているかはこのフレームの判定を渡し、押されたら応答を発行する。

/// 選択ボタン型とは、選ばれている状態を面の色で示すボタンの記述のことである。
pub struct 選択ボタン型<M> {
    表示文字列: String,
    選ばれている: bool,
    応答: M,
}

impl<M> 選択ボタン型<M> {
    pub(crate) fn 新規(表示文字列: String, 選ばれている: bool, 応答: M) -> Self {
        Self {
            表示文字列,
            選ばれている,
            応答,
        }
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> 選択ボタン型<N> {
        選択ボタン型 {
            表示文字列: self.表示文字列,
            選ばれている: self.選ばれている,
            応答: 応答を変換する(self.応答),
        }
    }
}

impl<M: Clone> 選択ボタン型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 反応 = ui.selectable_label(self.選ばれている, self.表示文字列.clone());
        if 反応.clicked() {
            発行した応答.push(self.応答.clone());
        }
        反応
    }
}
