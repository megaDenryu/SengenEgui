//! ボタン。押下時の処理は `押されたら` で渡し、描画と入力検出を利用側の式から分離する。

use crate::{node::ノード, source::文字列源, style::スタイル};

pub struct ボタンノード {
    源: 文字列源,
    装飾値: スタイル,
    押下時: Option<Box<dyn FnMut()>>,
    有効判定: Option<Box<dyn Fn() -> bool>>,
    最小幅: Option<f32>,
}

impl ボタンノード {
    pub(crate) fn 新規(源: 文字列源) -> Self {
        Self {
            源,
            装飾値: スタイル::無指定,
            押下時: None,
            有効判定: None,
            最小幅: None,
        }
    }

    /// 押された瞬間に呼ばれる処理を設定する。
    pub fn 押されたら(mut self, 処理: impl FnMut() + 'static) -> Self {
        self.押下時 = Some(Box::new(処理));
        self
    }

    /// 有効条件を設定する。false の間は淡色になり押せない。毎フレーム判定を取りに行く。
    pub fn 有効条件(mut self, 判定: impl Fn() -> bool + 'static) -> Self {
        self.有効判定 = Some(Box::new(判定));
        self
    }

    pub fn 装飾(mut self, 指定: スタイル) -> Self {
        self.装飾値 = 指定;
        self
    }

    pub fn 最小幅(mut self, 幅: f32) -> Self {
        self.最小幅 = Some(幅);
        self
    }
}

impl ノード for ボタンノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        let 有効 = self.有効判定.as_ref().is_none_or(|判定| 判定());
        let 文字 = self
            .装飾値
            .文字へ適用する(egui::RichText::new(self.源.現在値()));
        let mut 部品 = egui::Button::new(文字);
        if let Some(幅) = self.最小幅 {
            部品 = 部品.min_size(egui::vec2(幅, 0.0));
        }
        if ui.add_enabled(有効, 部品).clicked()
            && let Some(処理) = self.押下時.as_mut()
        {
            処理();
        }
    }
}
