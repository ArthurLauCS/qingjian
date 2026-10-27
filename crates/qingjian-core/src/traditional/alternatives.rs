//! 一个简体字对应的各个繁体字（干 → 幹 乾 干），按变体换成台湾 / 香港字形。
//!
//! 数据是 `data/opencc/` 下的 OpenCC 字典（来源与筛法见 `data/README.md`）。

use std::collections::HashMap;
use std::sync::LazyLock;

use super::TraditionalVariant;

/// 简体字 → 它对应的繁体字，常用的在前；只收对应不止一个的字。
static CHARACTERS: LazyLock<HashMap<char, Vec<char>>> =
    LazyLock::new(|| parse(include_str!("../../data/opencc/STCharacters.txt")));

static TAIWAN: LazyLock<HashMap<char, Vec<char>>> =
    LazyLock::new(|| parse(include_str!("../../data/opencc/TWVariants.txt")));

static HONGKONG: LazyLock<HashMap<char, Vec<char>>> =
    LazyLock::new(|| parse(include_str!("../../data/opencc/HKVariants.txt")));

/// `ch` 对应的各个繁体字，已换成 `variant` 的字形；只对应一个繁体字（或不用换）的字返回空。
pub(super) fn alternatives(variant: TraditionalVariant, ch: char) -> Vec<String> {
    let glyphs = match variant {
        TraditionalVariant::Taiwan => Some(&*TAIWAN),
        TraditionalVariant::Hongkong => Some(&*HONGKONG),
        TraditionalVariant::Off | TraditionalVariant::Standard => None,
    };
    CHARACTERS.get(&ch).map_or_else(Vec::new, |chars| {
        chars
            .iter()
            .map(|c| {
                glyphs
                    .and_then(|g| g.get(c))
                    .map_or(*c, |v| v[0])
                    .to_string()
            })
            .collect()
    })
}

/// OpenCC 文本字典：`键\t值 值…`，`#` 开头的是注释；只认单字的键与值。
fn parse(source: &str) -> HashMap<char, Vec<char>> {
    source
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let (key, values) = line.split_once('\t')?;
            let mut key = key.chars();
            let ch = key.next().filter(|_| key.next().is_none())?;
            let values: Vec<char> = values
                .split(' ')
                .filter_map(|v| {
                    let mut v = v.chars();
                    v.next().filter(|_| v.next().is_none())
                })
                .collect();
            (!values.is_empty()).then_some((ch, values))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 列出全部繁体字，常用的在前；台湾那档把 裏 换成 裡。
    #[test]
    fn lists_every_traditional_character() {
        assert_eq!(
            alternatives(TraditionalVariant::Standard, '干'),
            ["幹", "乾", "干"]
        );
        assert_eq!(
            alternatives(TraditionalVariant::Standard, '里'),
            ["裏", "里"]
        );
        assert_eq!(alternatives(TraditionalVariant::Taiwan, '里'), ["裡", "里"]);
        assert!(alternatives(TraditionalVariant::Taiwan, '你').is_empty());
    }
}
