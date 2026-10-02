//! キーの組の表記を確かめる。どのキーと修飾キーの組も、表記にしてから読むと同じ組へ戻ることと、決めた形の表記になることと、
//! 読めない表記を誤りとして返すことを確かめる。

use sengen_egui::{キー, キーの組, キーの組の表記の誤り, 修飾キー};

/// 往復を確かめる修飾キー。Ctrl は読むと `修飾キー::COMMAND` になるため、COMMAND で書く。
const 確かめる修飾キー: [修飾キー; 6] = [
    修飾キー::NONE,
    修飾キー::SHIFT,
    修飾キー::ALT,
    修飾キー::COMMAND,
    修飾キー {
        shift: true,
        ..修飾キー::COMMAND
    },
    修飾キー {
        alt: true,
        shift: true,
        ..修飾キー::COMMAND
    },
];

#[test]
fn どのキーと修飾キーの組も表記にして読むと同じ組へ戻る() {
    for キー in キー::ALL {
        for 修飾 in 確かめる修飾キー {
            let 組 = キーの組::生成する(修飾, *キー);
            let 表記 = 組.表記();
            assert_eq!(キーの組::表記から読む(&表記), Ok(組), "{表記}");
        }
    }
}

#[test]
fn 表記は修飾キーを決めた順に前へ付け矢印キーを矢印の文字で書く() {
    let 組 = |修飾, キー| キーの組::生成する(修飾, キー).表記();
    let コマンドとシフト = 修飾キー {
        shift: true,
        ..修飾キー::COMMAND
    };
    assert_eq!(組(コマンドとシフト, キー::Z), "Ctrl+Shift+Z");
    assert_eq!(組(修飾キー::SHIFT, キー::ArrowLeft), "Shift+←");
    assert_eq!(組(修飾キー::NONE, キー::OpenBracket), "[");
    assert_eq!(組(修飾キー::NONE, キー::Space), "Space");
    assert_eq!(組(修飾キー::CTRL, キー::S), "Ctrl+S");
    assert_eq!(組(修飾キー::COMMAND, キー::Plus), "Ctrl++");
}

#[test]
fn 修飾キーの順が違う表記とeguiの名前の表記も読める() {
    let コマンドとシフト = 修飾キー {
        shift: true,
        ..修飾キー::COMMAND
    };
    assert_eq!(
        キーの組::表記から読む("Shift+Ctrl+Z"),
        Ok(キーの組::生成する(コマンドとシフト, キー::Z))
    );
    assert_eq!(
        キーの組::表記から読む("ArrowLeft"),
        Ok(キーの組::単独(キー::ArrowLeft))
    );
    assert_eq!(
        キーの組::表記から読む("Ctrl+S"),
        Ok(キーの組::生成する(修飾キー::COMMAND, キー::S))
    );
}

#[test]
fn 読めない表記は誤りを返す() {
    for 表記 in ["", "Ctrl+", "Shift+", "Ctrl+無いキー", "Hyper+A"] {
        assert_eq!(
            キーの組::表記から読む(表記),
            Err(キーの組の表記の誤り::知らないキー(
                表記.to_string()
            )),
            "{表記}"
        );
    }
}
