//! 日本語フォントの候補が、使えたフォントを egui の文字の族の最後へ足し、どれも使えなければ
//! 試した置き場所と理由を返すことを確かめる。

use std::path::PathBuf;

use sengen_egui::{フォントを使えなかった理由, 日本語フォントの候補};

/// 試験ごとに一意の一時ファイルの置き場所。試験は並んで走るため、試験の名前とプロセスの番号で分ける。
fn 一時ファイルの置き場所(試験の名前: &str) -> PathBuf {
    std::env::temp_dir().join(format!("sengen_egui_{試験の名前}_{}", std::process::id()))
}

fn 書き出す(置き場所: &PathBuf, 中身: &[u8]) {
    if let Err(原因) = std::fs::write(置き場所, 中身) {
        panic!("試験のファイルを書き出せない: {原因}");
    }
}

#[test]
fn 読めない置き場所とフォントでないファイルだけなら置き場所ごとの理由を持つ失敗を返す() {
    let eguiの本体 = egui::Context::default();
    let フォントでない = 一時ファイルの置き場所("フォントでない.ttf");
    書き出す(&フォントでない, b"not a font file");
    let 候補 = 日本語フォントの候補::置き場所の一覧から作る([
        PathBuf::from("存在しない/一つ目.ttc"),
        フォントでない.clone(),
    ]);
    let 結果 = 候補.最初に読めたものを設定する(&eguiの本体);
    let _ = std::fs::remove_file(&フォントでない);
    let Err(失敗) = 結果 else {
        panic!("使えない置き場所だけなのに設定できた");
    };
    let [読めない, 形式が違う] = 失敗.試した結果一覧() else {
        panic!("試した結果は2件のはずである: {失敗}");
    };
    assert_eq!(読めない.置き場所(), PathBuf::from("存在しない/一つ目.ttc"));
    assert!(matches!(
        読めない.理由(),
        フォントを使えなかった理由::読めなかった(_)
    ));
    assert_eq!(形式が違う.置き場所(), フォントでない);
    assert!(matches!(
        形式が違う.理由(),
        フォントを使えなかった理由::フォントの形式でない
    ));
    let _ = eguiの本体.run(egui::RawInput::default(), |_| {});
}

#[test]
fn 最初に使えたフォントを通常と等幅の族の最後へ足す() {
    let eguiの本体 = egui::Context::default();
    let 定義 = egui::FontDefinitions::default();
    let Some(既定のフォント) = 定義.font_data.values().next() else {
        panic!("egui の既定のフォントが1つも無い");
    };
    let 置き場所 = 一時ファイルの置き場所("使えるフォント.ttf");
    書き出す(&置き場所, &既定のフォント.font);
    let 候補 = 日本語フォントの候補::置き場所の一覧から作る([
        PathBuf::from("存在しない/一つ目.ttc"),
        置き場所.clone(),
    ]);
    let 結果 = 候補.最初に読めたものを設定する(&eguiの本体);
    let _ = std::fs::remove_file(&置き場所);
    assert!(結果.is_ok());
    let _ = eguiの本体.run(egui::RawInput::default(), |_| {});
    let 最後の名前一覧 = eguiの本体.fonts(|フォント| {
        [egui::FontFamily::Proportional, egui::FontFamily::Monospace].map(|族| {
            フォント.lock().fonts.definitions().families[&族]
                .last()
                .cloned()
        })
    });
    let 足した名前 = Some("日本語の字形".to_string());
    assert_eq!(最後の名前一覧, [足した名前.clone(), 足した名前]);
}
