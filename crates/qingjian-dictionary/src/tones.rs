//! 词目逐音节的声调：1–4 是四声，5 是轻声，0 是不知道（不带声调的词库、某个音节没标）。
//!
//! 查词的键一律不带声调，声调只给显示用。五个音节以内的词压进词目自己的 16 位（每个音节 3 位，
//! 2–5 字词占绝大多数，不多占空间）；更长的词在那 16 位里只放一个标记，声调另放一节，按词目下标二分找。

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// 词目里最多压几个音节的声调。
pub(crate) const PACKED_SYLLABLES: usize = 5;

/// 词目的声调字段里这一位为 1：声调在溢出节里。
pub(crate) const SPILLED: u16 = 1 << 15;

/// 一个音节末尾的数字声调：`ni3` → (`ni`, 3)，`ma0` 与 `ma5` 都是轻声，不带数字的是 0。
pub fn split_tone(syllable: &str) -> (&str, u8) {
    match syllable.as_bytes().last() {
        Some(&digit @ b'1'..=b'5') => (&syllable[..syllable.len() - 1], digit - b'0'),
        Some(b'0') => (&syllable[..syllable.len() - 1], 5),
        _ => (syllable, 0),
    }
}

/// 压进词目的 16 位；超过 [`PACKED_SYLLABLES`] 的只返回 [`SPILLED`]，全都不知道的是 0。
pub(crate) fn pack(tones: &[u8]) -> u16 {
    if tones.iter().all(|&t| t == 0) {
        return 0;
    }
    if tones.len() > PACKED_SYLLABLES {
        return SPILLED;
    }
    tones
        .iter()
        .enumerate()
        .fold(0, |packed, (i, &t)| packed | u16::from(t) << (3 * i))
}

/// 溢出节的一条：词目下标与它在声调 arena 里的位置。12 字节、无填充，原样落盘。
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub(crate) struct Spill {
    /// 在 `Dictionary::slots` 里的下标，整节按它升序。
    pub slot: u32,

    /// 在声调 arena 里的字节偏移；arena 每个音节一个字符 `0`–`5`。
    pub start: u32,

    /// 音节数。
    pub len: u16,

    /// 对齐用，全零。
    pub reserved: u16,
}

/// 一个词逐音节的声调。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tones<'a>(Repr<'a>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Repr<'a> {
    #[default]
    Unknown,
    Packed(u16),
    Spilled(&'a [u8]),
}

impl<'a> Tones<'a> {
    pub(crate) fn packed(bits: u16) -> Self {
        Self(Repr::Packed(bits))
    }

    /// `digits` 是溢出 arena 里的一段 `0`–`5`。
    pub(crate) fn spilled(digits: &'a [u8]) -> Self {
        Self(Repr::Spilled(digits))
    }

    /// 第 `index` 个音节的声调，0 是不知道。
    pub fn get(&self, index: usize) -> u8 {
        match self.0 {
            Repr::Unknown => 0,
            Repr::Packed(bits) if index < PACKED_SYLLABLES => (bits >> (3 * index) & 0b111) as u8,
            Repr::Packed(_) => 0,
            Repr::Spilled(digits) => digits.get(index).map_or(0, |d| d.saturating_sub(b'0')),
        }
    }

    /// 前 `count` 个音节的声调。
    pub fn to_vec(&self, count: usize) -> Vec<u8> {
        (0..count).map(|i| self.get(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_trailing_tone_digits() {
        assert_eq!(split_tone("ni3"), ("ni", 3));
        assert_eq!(split_tone("lv4"), ("lv", 4));
        assert_eq!(split_tone("men5"), ("men", 5));
        assert_eq!(split_tone("ma0"), ("ma", 5));
        assert_eq!(split_tone("hao"), ("hao", 0));
    }

    #[test]
    fn packs_up_to_five_syllables() {
        let bits = pack(&[3, 3, 0, 5, 1]);
        assert_eq!(Tones::packed(bits).to_vec(6), [3, 3, 0, 5, 1, 0]);
        assert_eq!(pack(&[0, 0]), 0);
        assert_eq!(pack(&[1, 2, 3, 4, 5, 1]), SPILLED);
        assert_eq!(Tones::spilled(b"123450").to_vec(6), [1, 2, 3, 4, 5, 0]);
    }
}
