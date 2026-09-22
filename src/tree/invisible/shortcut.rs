//! キー操作。画面に何も描かず、そのフレームにキーの組が押されていたら応答を発行する。
//!
//! 発行の判定は egui の `consume_shortcut` で行い、押下の事象を消費する（同じ組を見張る
//! 別の部品には届かない）。文字入力欄にフォーカスがあるときも修飾キー付きの組は届く。
//! 修飾キー無しの文字キーは、木の中でこの部品より先に描画された入力欄が文字として消費した
//! 後には残らないため、そのような組は木の先頭に置くか、修飾キー付きの組にする。

/// キーの組とは、修飾キーとキーを合わせた1つの押し方のことである。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct キーの組(egui::KeyboardShortcut);

impl キーの組 {
    /// 修飾キー（Ctrl・Shift・Alt 等）とキーから組を作る。修飾キー無しなら `egui::Modifiers::NONE`。
    pub const fn 生成する(修飾キー: egui::Modifiers, キー: egui::Key) -> Self {
        Self(egui::KeyboardShortcut::new(修飾キー, キー))
    }

    /// 修飾キーを押さない単独のキーの組。
    pub const fn 単独(キー: egui::Key) -> Self {
        Self::生成する(egui::Modifiers::NONE, キー)
    }
}

/// キー操作型とは、キーの組と押されたときの応答の組の記述のことである。
pub struct キー操作型<M> {
    キーの組: キーの組,
    応答: M,
}

impl<M> キー操作型<M> {
    pub(crate) fn 新規(組: キーの組, 応答: M) -> Self {
        Self {
            キーの組: 組, 応答
        }
    }

    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> キー操作型<N> {
        キー操作型 {
            キーの組: self.キーの組,
            応答: 応答を変換する(self.応答),
        }
    }
}

impl<M: Clone> キー操作型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 押された = ui.input_mut(|入力| 入力.consume_shortcut(&self.キーの組.0));
        if 押された {
            発行した応答.push(self.応答.clone());
        }
        ui.response()
    }
}
