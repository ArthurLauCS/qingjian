//! 导入词库：把用户给的文件（青简 TSV、Rime `.dict.yaml`、现成的 `.qj`）变成用户词库目录里的一个 `.qj`。
//!
//! 导入时只做格式转换，不做拼音校验：不合法的音节查不到而已，不影响别的词。
//! 第二列是形码编码的（五笔词库）不进拼音词库，报 [`DictionaryError::CodeWords`]，由调用方改走 [`import_code_words`]。
//! 目标文件名取源文件的主干（`law.dict.yaml` → `law.qj`），已存在就覆盖（重新导入即更新）。

mod code_words;
mod format;
mod imported;
mod rime;

use std::path::{Path, PathBuf};

use qingjian_format::{Container, Metadata};

use crate::dictionary::Dictionary;
use crate::error::DictionaryError;

pub use code_words::{code_words_scheme, import_code_words};
pub use format::is_code_table;
pub use imported::Imported;
// `dict-convert wubi` 也读 Rime `.dict.yaml`（形码码表的第二列是编码，转换方式一样），共用这一个解析器
pub use rime::{Parsed, looks_like_rime, to_tsv};

/// 把 `source` 导入到 `dest_dir`，返回写出的文件与元数据。
pub fn import(source: &Path, dest_dir: &Path) -> Result<Imported, DictionaryError> {
    let stem = source
        .file_name()
        .and_then(|n| n.to_str())
        .map(strip_extensions)
        .filter(|s| !s.is_empty())
        .ok_or(DictionaryError::Corrupt("source has no usable file name"))?;
    std::fs::create_dir_all(dest_dir)?;
    let target = dest_dir.join(format!("{stem}.qj"));
    let (dictionary, metadata) = if Container::is_qj(source) {
        let dictionary = Dictionary::open_qj(source)?;
        let metadata = dictionary.metadata().cloned().unwrap_or_default();
        (dictionary, metadata)
    } else {
        let text = std::fs::read_to_string(source)?;
        let (tsv, name) = if rime::looks_like_rime(&text) {
            let parsed = rime::to_tsv(&text);
            (parsed.tsv, parsed.name)
        } else {
            (text, None)
        };
        if format::is_code_table(&tsv) {
            return Err(DictionaryError::CodeWords);
        }
        let dictionary = Dictionary::parse(&tsv)?;
        let metadata = Metadata {
            name: name.unwrap_or_else(|| stem.clone()),
            source: source.display().to_string(),
            ..Metadata::default()
        };
        (dictionary, metadata)
    };
    if dictionary.is_empty() {
        return Err(DictionaryError::Corrupt(
            "no usable entries; expected word and explicit pinyin columns",
        ));
    }
    dictionary.write_qj(&target, &metadata)?;
    Ok(Imported {
        path: target,
        name: metadata.name,
        entries: dictionary.len(),
    })
}

/// `law.dict.yaml` → `law`，`dict.tsv` → `dict`。
pub(crate) fn strip_extensions(file_name: &str) -> String {
    let mut stem = file_name;
    for suffix in [".dict.yaml", ".yaml", ".yml", ".tsv", ".txt", ".qj"] {
        if let Some(s) = stem.strip_suffix(suffix) {
            stem = s;
            break;
        }
    }
    stem.to_owned()
}

/// 词库目录里的 `.qj` 文件名。
pub fn target_name(stem: &str) -> PathBuf {
    PathBuf::from(format!("{stem}.qj"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_tsv_and_rime_into_qj() {
        let dir = std::env::temp_dir().join(format!("qingjian-import-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let tsv = dir.join("finance.tsv");
        std::fs::write(&tsv, "账套\tzhang tao\t500\n").unwrap();
        let imported = import(&tsv, &dir.join("dicts")).unwrap();
        assert_eq!(imported.name, "finance");
        assert_eq!(imported.entries, 1);
        let dictionary = Dictionary::from_path(&imported.path).unwrap();
        assert_eq!(dictionary.lookup(&["zhang", "tao"], false)[0].text, "账套");
        assert_eq!(dictionary.metadata().unwrap().name, "finance");

        let rime = dir.join("law.dict.yaml");
        std::fs::write(
            &rime,
            "# Rime dictionary\n---\nname: law\nversion: \"1\"\nsort: by_weight\n...\n合同法\the tong fa\t120\n民法典\tmin fa dian\n",
        )
        .unwrap();
        let imported = import(&rime, &dir.join("dicts")).unwrap();
        assert_eq!(imported.path.file_name().unwrap(), "law.qj");
        assert_eq!(imported.name, "law");
        assert_eq!(imported.entries, 2);
        let dictionary = Dictionary::from_path(&imported.path).unwrap();
        assert_eq!(
            dictionary.lookup(&["min", "fa", "dian"], false)[0].text,
            "民法典"
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn imports_tone_numbered_pinyin() {
        let dir =
            std::env::temp_dir().join(format!("qingjian-import-tones-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let tsv = dir.join("toned.tsv");
        std::fs::write(&tsv, "东西\tdong1 xi5\t20\n绿色\tlv4 se4\t10\n").unwrap();
        assert!(!format::is_code_table(
            &std::fs::read_to_string(&tsv).unwrap()
        ));
        let imported = import(&tsv, &dir.join("dicts")).unwrap();
        let dictionary = Dictionary::from_path(&imported.path).unwrap();
        let hit = dictionary.lookup(&["dong", "xi"], false)[0];
        assert_eq!((hit.text, hit.pinyin), ("东西", "dong xi"));
        assert_eq!(hit.tones.to_vec(2), [1, 5]);
        assert_eq!(dictionary.lookup(&["lv", "se"], false)[0].text, "绿色");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn refuses_wubi_words_as_pinyin() {
        let dir = std::env::temp_dir().join(format!("qingjian-import-wubi-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let tsv = dir.join("wubi-test.tsv");
        std::fs::write(&tsv, "拉混\truji\t1\n青简\tgetu\t1\n").unwrap();
        assert!(matches!(
            import(&tsv, &dir.join("dicts")),
            Err(DictionaryError::CodeWords)
        ));
        assert!(!dir.join("dicts/wubi-test.qj").exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn strips_known_extensions() {
        assert_eq!(strip_extensions("law.dict.yaml"), "law");
        assert_eq!(strip_extensions("dict.tsv"), "dict");
        assert_eq!(strip_extensions("x.qj"), "x");
        assert_eq!(strip_extensions("noext"), "noext");
    }
}
