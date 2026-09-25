//! 見本の再生。再生画面のページが読む状態と、その応答の適用と、描画の前に毎回行う再生の進め方を持つ。
//! 利用する側（動画の再生アプリ等）が差し替えられるテクスチャを保持して毎コマ差し替える形の見本である。

use std::path::PathBuf;
use std::time::{Duration, Instant};

use eframe::egui;
use sengen_egui::{
    区間の帯の操作, 差し替えられるテクスチャ, 拡大縮小の仕方, 画素の並びの不正, 通知の回,
};

use crate::sample_clip::見本の区間;
use crate::sample_texture::見本の画素のバイト列;

/// 見本の再生の長さ。再生位置の帯の範囲になる。
pub const 再生の長さ: Duration = Duration::from_secs(8);
/// 1コマを出している時間（1秒に16コマ）。
pub const 一コマの時間: Duration = Duration::from_millis(62);

/// 再生画面の応答とは、再生画面のページの操作の語彙のことである。
#[derive(Clone, Debug)]
pub enum 再生画面の応答 {
    再生を切り替えた,
    位置を動かしている(f32),
    位置を決めた(f32),
    札を選んだ(usize),
    通知を出した(&'static str),
    ファイルを受け取った(PathBuf),
    区間の帯を操作した(区間の帯の操作),
}

/// 見本の再生とは、映像のテクスチャと再生の進み具合と、再生画面の他の表示の状態の組のことである。
pub struct 見本の再生 {
    pub 映像: 差し替えられるテクスチャ,
    pub 再生中: bool,
    pub 位置: Duration,
    pub 選んだ札: usize,
    pub 通知: Option<(通知の回, &'static str)>,
    pub 受け取ったファイル: Vec<String>,
    pub 区間: 見本の区間,
    表示中のコマ: u32,
    前回の時刻: Option<Instant>,
}

impl 見本の再生 {
    pub fn 登録して作る(
        eguiの本体: &egui::Context,
    ) -> Result<Self, 画素の並びの不正> {
        let 最初のコマ = 見本の画素のバイト列::ずらして作る(0);
        let 映像 = 差し替えられるテクスチャ::登録して作る(
            eguiの本体,
            "見本の映像",
            最初のコマ.画素の並び()?,
            拡大縮小の仕方::画素をそのまま使う,
        );
        Ok(Self {
            映像,
            再生中: false,
            位置: Duration::ZERO,
            選んだ札: 0,
            通知: None,
            受け取ったファイル: Vec::new(),
            区間: 見本の区間::新規(),
            表示中のコマ: 0,
            前回の時刻: None,
        })
    }

    /// 描画の前に毎回呼び、再生中なら経過した時間だけ位置を進め、コマが変わったときだけ映像を差し替える。
    pub fn 進める(&mut self, 今: Instant) -> Result<(), 画素の並びの不正> {
        let 経過 = self.前回の時刻.map_or(Duration::ZERO, |前回| 今 - 前回);
        self.前回の時刻 = Some(今);
        if self.再生中 {
            let ナノ秒 = (self.位置 + 経過).as_nanos() % 再生の長さ.as_nanos();
            self.位置 = Duration::from_nanos(u64::try_from(ナノ秒).unwrap_or(0));
        }
        let コマ = self.位置.as_nanos() / 一コマの時間.as_nanos();
        let コマ = u32::try_from(コマ).unwrap_or(0);
        if コマ != self.表示中のコマ {
            self.映像
                .差し替える(見本の画素のバイト列::ずらして作る(コマ).画素の並び()?);
            self.表示中のコマ = コマ;
        }
        Ok(())
    }

    pub fn 適用する(&mut self, 応答: 再生画面の応答) {
        match 応答 {
            再生画面の応答::再生を切り替えた => self.再生中 = !self.再生中,
            再生画面の応答::位置を動かしている(秒) | 再生画面の応答::位置を決めた(秒) => {
                self.位置 = Duration::from_secs_f32(秒.clamp(0.0, 再生の長さ.as_secs_f32()))
            }
            再生画面の応答::札を選んだ(番号) => self.選んだ札 = 番号,
            再生画面の応答::通知を出した(文) => {
                let 回 = self.通知.map_or(通知の回::最初(), |(回, _)| 回.次());
                self.通知 = Some((回, 文));
            }
            再生画面の応答::ファイルを受け取った(パス) => {
                self.受け取ったファイル.push(パス.display().to_string())
            }
            再生画面の応答::区間の帯を操作した(操作) => {
                if let Some(秒) = self.区間.操作を適用する(操作) {
                    self.位置 = Duration::from_secs_f64(秒.clamp(0.0, 再生の長さ.as_secs_f64()));
                }
            }
        }
    }
}
