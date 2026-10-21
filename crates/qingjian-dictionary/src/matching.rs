use crate::tones::Tones;

/// 一次查询命中的词目，全部借用自词库。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Match<'a> {
    /// 词。
    pub text: &'a str,

    /// 空格分隔的音节。
    pub pinyin: &'a str,

    /// 静态词频。
    pub frequency: u32,

    /// 逐音节声调；词库没标的是不知道。
    pub tones: Tones<'a>,

    /// 来自补充词库（[`crate::Dictionary::set_secondary`]）：精确命中不压过主词库的纠错，整句里多扣一点分。
    pub secondary: bool,

    /// 音节数与查询模式的长度一致（而不是以查询为前缀的更长词）。
    pub exact: bool,
}

impl<'a> Match<'a> {
    pub fn syllables(&self) -> impl Iterator<Item = &'a str> {
        self.pinyin.split(' ')
    }

    pub fn syllable_count(&self) -> usize {
        self.pinyin.split(' ').count()
    }
}
