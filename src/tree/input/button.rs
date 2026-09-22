//! ボタン。押されたら応答の値を発行する。処理そのものは持たない（lib.rs 方針2）。

use crate::measure::論理画素;
use crate::style::スタイル;

/// ボタン型とは、押されると応答を発する部品の記述のことである。
pub struct ボタン型<M> {
    表示文字列: String,
    応答: M,
    装飾値: スタイル,
    有効指定: bool,
    最小幅指定: Option<論理画素>,
    小さい指定: bool,
    枠なし指定: bool,
}

impl<M> ボタン型<M> {
    pub(crate) fn 新規(表示文字列: String, 応答: M) -> Self {
        Self {
            表示文字列,
            応答,
            装飾値: スタイル::無指定,
            有効指定: true,
            最小幅指定: None,
            小さい指定: false,
            枠なし指定: false,
        }
    }

    /// 有効・無効を指定する。false の間は淡色になり押せない。値はこのフレームの判定を渡す。
    pub fn 有効(mut self, 有効: bool) -> Self {
        self.有効指定 = 有効;
        self
    }

    /// 装飾を適用する。装飾は名前付き定数として構造の外で定義する（lib.rs 方針3）。
    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    /// ボタンの幅の下限を指定する。
    pub fn 最小幅(mut self, 幅: 論理画素) -> Self {
        self.最小幅指定 = Some(幅);
        self
    }

    /// 文字の周りの余白を詰めた小さなボタンにする。
    pub fn 小さい(mut self) -> Self {
        self.小さい指定 = true;
        self
    }

    /// 面の塗りと枠線を消し、文字だけのボタンにする。
    pub fn 枠なし(mut self) -> Self {
        self.枠なし指定 = true;
        self
    }
}

impl<M: Clone> ボタン型<M> {
    pub(crate) fn 描画する(
        &self,
        ui: &mut egui::Ui,
        発行した応答: &mut Vec<M>,
    ) -> egui::Response {
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.表示文字列.clone()));
        let mut 部品 = egui::Button::new(文字).frame(!self.枠なし指定);
        if let Some(色) = self.装飾値.背景色 {
            部品 = 部品.fill(色);
        }
        if let Some(幅) = self.最小幅指定 {
            部品 = 部品.min_size(egui::vec2(幅.eguiへ渡す値(), 0.0));
        }
        if self.小さい指定 {
            部品 = 部品.small();
        }
        let 反応 = ui.add_enabled(self.有効指定, 部品);
        if 反応.clicked() {
            発行した応答.push(self.応答.clone());
        }
        反応
    }
}

impl<M> ボタン型<M> {
    /// 応答型を別の型へ写す。ノードの `写す` から呼ばれる。
    pub(crate) fn 写す<N>(self, 応答を変換する: &dyn Fn(M) -> N) -> ボタン型<N> {
        ボタン型 {
            表示文字列: self.表示文字列,
            応答: 応答を変換する(self.応答),
            装飾値: self.装飾値,
            有効指定: self.有効指定,
            最小幅指定: self.最小幅指定,
            小さい指定: self.小さい指定,
            枠なし指定: self.枠なし指定,
        }
    }
}
