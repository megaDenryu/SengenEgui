//! 余白と区切り線。縦積みの中なら縦、横並びの中なら横に効く。

use crate::node::ノード;

pub struct 余白ノード {
    量: f32,
}

impl 余白ノード {
    pub(crate) fn 新規(量: f32) -> Self {
        Self { 量 }
    }
}

impl ノード for 余白ノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        ui.add_space(self.量);
    }
}

pub struct 区切り線ノード;

impl ノード for 区切り線ノード {
    fn 描画する(&mut self, ui: &mut egui::Ui) {
        ui.separator();
    }
}
