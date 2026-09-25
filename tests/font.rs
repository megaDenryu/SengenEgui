//! 日本語フォントの候補が、読めたフォントを egui の文字の族の最後へ足し、どれも読めなければ試した置き場所を返すことを確かめる。

use std::path::PathBuf;

use sengen_egui::日本語フォントの候補;

/// egui の既定のフォントを1つ、試験用の一時ファイルへ書き出して置き場所を返す。日本語の字形は持たないが、読めることの確認には足りる。
fn 読めるフォントのファイルを作る(名前: &str) -> PathBuf {
    let 定義 = egui::FontDefinitions::default();
    let Some(フォント) = 定義.font_data.values().next() else {
        panic!("egui の既定のフォントが1つも無い");
    };
    let 置き場所 = std::env::temp_dir().join(名前);
    if let Err(原因) = std::fs::write(&置き場所, &フォント.font) {
        panic!("試験のフォントを書き出せない: {原因}");
    }
    置き場所
}

#[test]
fn 読めない置き場所だけなら試した置き場所を持つ失敗を返す() {
    let eguiの本体 = egui::Context::default();
    let 候補 = 日本語フォントの候補::置き場所の一覧から作る([
        "存在しない/一つ目.ttc",
        "存在しない/二つ目.ttf",
    ]);
    let 結果 = 候補.最初に読めたものを設定する(&eguiの本体);
    let Err(失敗) = 結果 else {
        panic!("読めない置き場所だけなのに設定できた");
    };
    let 文 = 失敗.to_string();
    assert!(
        文.contains("一つ目.ttc") && 文.contains("二つ目.ttf"),
        "{文}"
    );
}

#[test]
fn 最初に読めたフォントを通常と等幅の族の最後へ足す() {
    let eguiの本体 = egui::Context::default();
    let 置き場所 = 読めるフォントのファイルを作る("sengen_egui_試験のフォント.ttf");
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
