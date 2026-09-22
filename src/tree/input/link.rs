//! リンク。下線付きの文字で、押されたら応答を発行する。ブラウザで開く外部リンクとは別の部品である。

/// リンク型とは、押されると応答を発する文字のリンクの記述のことである。
pub struct リンク型<M> {
    表示文字列: String,
    応答: M,
}

impl<M> リンク型<M> {
    pub(crate) fn 新規(表示文字列: String, 応答: M) -> Self {
        Self {
            表示文字列, 応答
        }
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> リンク型<N> {
        リンク型 {
            表示文字列: self.表示文字列,
            応答: 応答を変換する(self.応答),
        }
    }
}

impl<M: Clone> リンク型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 反応 = ui.link(self.表示文字列.clone());
        if 反応.clicked() {
            発行した応答.push(self.応答.clone());
        }
        反応
    }
}
