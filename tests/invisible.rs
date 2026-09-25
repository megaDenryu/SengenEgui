//! 落とされたファイルの受け取りがパスごとに応答を発して入力から取り除くことと、
//! 描き直しの予約が次の描き直しまでの時間を egui へ伝えることを確かめる。

mod common;

use std::path::PathBuf;
use std::time::Duration;

use common::output::{入力を指定して描画する, 描き直しまでの時間};
use sengen_egui::{
    ノード, 子, 描き直しを予約する, 縦積み, 落とされたファイルを受け取る
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    先の受け取りが受け取った(PathBuf),
    後の受け取りが受け取った(PathBuf),
}

fn 落とした入力(パス一覧: &[&str]) -> egui::RawInput {
    let dropped_files = パス一覧
        .iter()
        .map(|パス| egui::DroppedFile {
            path: Some(PathBuf::from(パス)),
            ..Default::default()
        })
        .chain([egui::DroppedFile {
            name: "パスの無いファイル".to_string(),
            ..Default::default()
        }])
        .collect();
    egui::RawInput {
        dropped_files,
        ..Default::default()
    }
}

fn 受け取り2つの木() -> ノード<応答> {
    縦積み(子![
        落とされたファイルを受け取る(応答::先の受け取りが受け取った),
        落とされたファイルを受け取る(応答::後の受け取りが受け取った),
    ])
    .into()
}

#[test]
fn 落とされたファイルはパスごとに先に描画した受け取りだけが受け取る() {
    let eguiの本体 = egui::Context::default();
    let 入力 = 落とした入力(&["C:/動画/一.mp4", "C:/動画/二.mp4"]);
    let (集まり, _) = 入力を指定して描画する(&eguiの本体, 入力, &受け取り2つの木);
    assert_eq!(
        集まり,
        vec![
            応答::先の受け取りが受け取った(PathBuf::from("C:/動画/一.mp4")),
            応答::先の受け取りが受け取った(PathBuf::from("C:/動画/二.mp4")),
        ]
    );
}

#[test]
fn 描き直しの予約は待つ時間を次の描き直しまでの時間として伝える() {
    let eguiの本体 = egui::Context::default();
    let 木 = || -> ノード<()> { 描き直しを予約する(Duration::from_millis(33)).into() };
    let (_, 出力) = 入力を指定して描画する(&eguiの本体, egui::RawInput::default(), &木);
    assert!(描き直しまでの時間(&出力) <= Duration::from_millis(33));
}
