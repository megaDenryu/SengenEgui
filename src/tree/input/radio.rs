//! ラジオボタン。複数の選択肢から1つを選ぶ丸い印。選ばれているかはこのフレームの判定を渡し、
//! 押されたら応答を発行する（選ばれていないものを押したときだけ発行する）。

/// ラジオボタン型とは、排他的な選択肢の1つを表す部品の記述のことである。
pub struct ラジオボタン型<M> {
    表示文字列: String,
    選ばれている: bool,
    応答: M,
}

impl<M> ラジオボタン型<M> {
    pub(crate) fn 新規(表示文字列: String, 選ばれている: bool, 応答: M) -> Self {
        Self {
            表示文字列,
            選ばれている,
            応答,
        }
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(
        self, 応答を変換する: &dyn Fn(M) -> N
    ) -> ラジオボタン型<N> {
        ラジオボタン型 {
            表示文字列: self.表示文字列,
            選ばれている: self.選ばれている,
            応答: 応答を変換する(self.応答),
        }
    }
}

impl<M: Clone> ラジオボタン型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 反応 = ui.radio(self.選ばれている, self.表示文字列.clone());
        if 反応.clicked() && !self.選ばれている {
            発行した応答.push(self.応答.clone());
        }
        反応
    }
}
