//! 見本帳の状態と応答。画面は状態を読んで組み、操作は応答として発行され、ここで状態へ適用する。

use eframe::egui;
use sengen_egui::範囲枠の操作;

use crate::sample_range_frame::見本の範囲枠;

/// 応答とは、見本帳の画面の操作の語彙のことである。
#[derive(Clone, Debug)]
pub enum 応答 {
    タブを選んだ(usize),
    押した(&'static str),
    有効を切り替えた(bool),
    一行を変えた(String),
    合言葉を変えた(String),
    複数行を変えた(String),
    数を変えた(i32),
    小数を変えた(f32),
    選んだ(&'static str),
    色を変えた(egui::Color32),
    窓を開閉した(bool),
    覆いを開閉した(bool),
    受け取った(String),
    記録を消した,
    範囲枠を操作した(範囲枠の操作),
}

/// 状態とは、見本帳の画面が読む値の集まりのことである。
pub struct 状態 {
    pub 選択中のタブ: usize,
    pub 有効: bool,
    pub 一行: String,
    pub 合言葉: String,
    pub 複数行: String,
    pub 数: i32,
    pub 小数: f32,
    pub 選んだもの: &'static str,
    pub 色: egui::Color32,
    pub 窓が開いている: bool,
    pub 覆いが開いている: bool,
    pub 受け取った一覧: Vec<String>,
    pub 最後に押したもの: &'static str,
    pub 記録: Vec<String>,
    pub 画像: egui::TextureHandle,
    pub 範囲枠: 見本の範囲枠,
}

impl 状態 {
    pub fn 新規(画像: egui::TextureHandle) -> Self {
        Self {
            選択中のタブ: 0,
            有効: true,
            一行: "一行の値".to_string(),
            合言葉: String::new(),
            複数行: "1行目\n2行目".to_string(),
            数: 3,
            小数: 0.5,
            選んだもの: "甲",
            色: egui::Color32::from_rgb(80, 150, 240),
            窓が開いている: false,
            覆いが開いている: false,
            受け取った一覧: Vec::new(),
            最後に押したもの: "（まだ何も押していない）",
            記録: Vec::new(),
            画像,
            範囲枠: 見本の範囲枠::新規(),
        }
    }

    pub fn 適用する(&mut self, 応答: 応答) {
        self.記録.push(format!("{応答:?}"));
        match 応答 {
            応答::タブを選んだ(番号) => self.選択中のタブ = 番号,
            応答::押した(名前) => self.最後に押したもの = 名前,
            応答::有効を切り替えた(値) => self.有効 = 値,
            応答::一行を変えた(値) => self.一行 = 値,
            応答::合言葉を変えた(値) => self.合言葉 = 値,
            応答::複数行を変えた(値) => self.複数行 = 値,
            応答::数を変えた(値) => self.数 = 値,
            応答::小数を変えた(値) => self.小数 = 値,
            応答::選んだ(値) => self.選んだもの = 値,
            応答::色を変えた(値) => self.色 = 値,
            応答::窓を開閉した(値) => self.窓が開いている = 値,
            応答::覆いを開閉した(値) => self.覆いが開いている = 値,
            応答::受け取った(値) => self.受け取った一覧.push(値),
            応答::記録を消した => self.記録.clear(),
            応答::範囲枠を操作した(操作) => self.範囲枠.操作を適用する(操作),
        }
    }
}
