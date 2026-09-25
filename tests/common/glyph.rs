//! 字形の縦の位置の測定。egui が並べた字形の描画の矩形（字形の位置に字形の画像のずれを足した矩形）の縦の中心を、
//! 英字と日本語に分けて平均する。ボタンの中で文字が縦の中央にあるかは、この中心の差で比べる。

use std::sync::Arc;

/// 英字と日本語の字形の縦の中心の平均。どちらかの字形が無い族は None である。
pub struct 縦の中心 {
    pub 英字: Option<f32>,
    pub 日本語: Option<f32>,
}

/// 描画の矩形の縦の中心を、英字と日本語に分けて集める。起点は文字の図形の左上（画面の座標）である。
#[derive(Default)]
pub struct 縦の中心の集まり {
    英字: Vec<f32>,
    日本語: Vec<f32>,
}

impl 縦の中心の集まり {
    /// 並べた文字の全部の字形を、起点からの画面の座標で足す。空白と画像を持たない字形は飛ばす。
    pub fn 足す(&mut self, 起点: egui::Pos2, 並べた文字: &egui::Galley) {
        for 行 in &並べた文字.rows {
            for 字形 in 行
                .row
                .glyphs
                .iter()
                .filter(|字形| !字形.uv_rect.is_nothing())
            {
                let 上端 = 起点.y + 行.pos.y + 字形.pos.y + 字形.uv_rect.offset.y;
                let 中心 = 上端 + 字形.uv_rect.size.y / 2.0;
                if 字形.chr.is_ascii() {
                    self.英字.push(中心);
                } else {
                    self.日本語.push(中心);
                }
            }
        }
    }

    pub fn 平均する(&self) -> 縦の中心 {
        let 平均 = |一覧: &[f32]| {
            (!一覧.is_empty())
                .then(|| 一覧.iter().sum::<f32>() / 一覧.iter().map(|_| 1.0).sum::<f32>())
        };
        縦の中心 {
            英字: 平均(&self.英字),
            日本語: 平均(&self.日本語),
        }
    }
}

impl 縦の中心 {
    /// 日本語の中心から英字の中心を引いた差。正なら日本語が下にある。両方が無ければ試験を止める。
    pub fn 日本語の英字からの差(&self) -> f32 {
        match (self.英字, self.日本語) {
            (Some(英字), Some(日本語)) => 日本語 - 英字,
            _ => panic!("英字と日本語の両方の字形が要る"),
        }
    }
}

/// 字の大きさと族を指定して、文字列を1行に並べる。
pub fn 一行に並べる(
    eguiの本体: &egui::Context,
    文字列: &str,
    字: egui::FontId,
) -> Arc<egui::Galley> {
    eguiの本体
        .fonts(|フォント| フォント.layout_no_wrap(文字列.to_string(), 字, egui::Color32::WHITE))
}

/// 出力の図形のうち、文字の図形を全部集める。
pub fn 文字の図形一覧(出力: &egui::FullOutput) -> Vec<egui::epaint::TextShape> {
    let mut 一覧 = Vec::new();
    for 切り抜いた図形 in &出力.shapes {
        文字の図形を集める(&切り抜いた図形.shape, &mut 一覧);
    }
    一覧
}

fn 文字の図形を集める(図形: &egui::Shape, 一覧: &mut Vec<egui::epaint::TextShape>) {
    match 図形 {
        egui::Shape::Text(文字) => 一覧.push(文字.clone()),
        egui::Shape::Vec(中の一覧) => {
            中の一覧.iter().for_each(|中| 文字の図形を集める(中, 一覧))
        }
        _ => {}
    }
}
