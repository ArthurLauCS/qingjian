//! 繁体写法表：繁体输出时用户给某个简体词选了哪种写法（软件 选了 軟件 而不是缺省的 軟體），下次排第一。

use std::io::Write;
use std::path::{Path, PathBuf};

use qingjian_core::storage::write_atomic;

use super::{FrequencyLearner, USER_TRADITIONAL_FILE, data_lines};
use crate::error::LearningError;

impl FrequencyLearner {
    /// 繁体写法表与词频文件同目录。
    pub(super) fn traditional_path(frequency_path: &Path) -> PathBuf {
        frequency_path.with_file_name(USER_TRADITIONAL_FILE)
    }

    /// 从 `档\t简体词\t写法` 读繁体写法表，返回跳过的坏行数。
    pub(super) fn load_traditional(&mut self, source: &str) -> usize {
        let mut skipped = 0;
        for line in data_lines(source) {
            let mut fields = line.split('\t');
            match (fields.next(), fields.next(), fields.next(), fields.next()) {
                (Some(variant), Some(simplified), Some(form), None) if !form.is_empty() => {
                    self.traditional
                        .entry(variant.to_owned())
                        .or_default()
                        .insert(simplified.to_owned(), form.to_owned());
                }
                _ => skipped += 1,
            }
        }
        skipped
    }

    pub(super) fn save_traditional_to(&mut self, path: &Path) -> Result<(), LearningError> {
        let mut rows: Vec<(&String, &String, &String)> = self
            .traditional
            .iter()
            .flat_map(|(variant, forms)| forms.iter().map(move |(s, form)| (variant, s, form)))
            .collect();
        rows.sort();
        write_atomic(path, |file| {
            writeln!(file, "# 青简繁体写法：档\t简体词\t选的写法")?;
            for (variant, simplified, form) in rows {
                writeln!(file, "{variant}\t{simplified}\t{form}")?;
            }
            Ok(())
        })?;
        self.traditional_dirty = false;
        Ok(())
    }

    /// 繁体写法表里有几条。
    pub fn traditional_count(&self) -> usize {
        self.traditional.values().map(|forms| forms.len()).sum()
    }
}
