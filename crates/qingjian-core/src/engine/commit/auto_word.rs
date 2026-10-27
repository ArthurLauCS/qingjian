//! 自动造词：一段拼音上屏完时把这段的词合成用户词，连着选出的两个词记够次数也合成用户词。
//! 门槛常数见 [`AUTO_WORD_THRESHOLD`] 一组。

use super::{Choice, Transition};
use crate::candidate::{Candidate, CandidateKind};
use crate::engine::{
    AUTO_WORD_MAX_CHARS, AUTO_WORD_THRESHOLD, AUTO_WORD_THRESHOLD_SAME_BUFFER,
    EXPLICIT_TRANSITION_WEIGHT, Engine, Learner, reading_key,
};
use qingjian_dictionary::Dictionary;

impl Engine {
    /// 一段拼音上屏完了，这段里上屏的几个词合起来就是用户对这段拼音的答案：分几次选完的
    /// （`jidiaole` 先选 挤、剩下的走整句 掉了），或一次整句上屏、整句全由单字拼成的（`touchuan` 的 透 + 传，词库里没有这个词）。
    /// 合成词词库里没有、不超过 [`AUTO_WORD_MAX_CHARS`] 字，选一次就造成用户词，之后简拼、半截拼音也打得出来。
    /// 分几次选完的另记一次「整段拼音 → 合成词」的选择，下次整段打出来它直接排第一；整句的不记：
    /// 一次整句接受不算用户在这段拼音下选了中文，记了会压英文词的位置。返回记下的选择与造出的词，撤销时退回。
    pub(super) fn finish_buffer(
        &mut self,
        split: bool,
    ) -> (Option<(String, Choice)>, Option<String>) {
        let words = self.chain.buffer_words();
        // 整句里有词库里的词（版本 + 好、我的 + 书）是拼出来的短句而不是新词：空格手滑上屏一次就成词，会压住真正要的词
        if words.len() < 2 || (!split && words.iter().any(|(t, _)| t.chars().count() > 1)) {
            return (None, None);
        }
        let text: String = words.iter().map(|(t, _)| t.as_str()).collect();
        let syllables: Vec<String> = words.iter().flat_map(|(_, s)| s.iter().cloned()).collect();
        let chars = text.chars().count();
        let key = self.chain.buffer_key().to_owned();
        if key.is_empty() || chars > AUTO_WORD_MAX_CHARS || chars != syllables.len() {
            return (None, None);
        }
        let candidate = Candidate {
            text,
            kind: CandidateKind::Chinese,
            syllables,
            reading: None,
            translation: None,
            aux_code: None,
        };
        let reading = reading_key(candidate.syllables.iter().map(String::as_str));
        let phrase = split.then(|| {
            self.learner.record_choice(&key, &reading, &candidate.text);
            (
                key,
                Choice {
                    reading,
                    text: candidate.text.clone(),
                },
            )
        });
        if self.knows_word(&candidate) {
            return (phrase, None);
        }
        tracing::debug!(text = %candidate.text, "整段拼音上屏完，自动造词");
        self.learner
            .learn_word(&candidate.text, &candidate.syllables);
        // 私密输入、关了学习时 learn_word 被吞掉而删词照做：没造出来的，撤销时不能去删
        let coined = self.knows_word(&candidate).then_some(candidate.text);
        (phrase, coined)
    }

    /// 一个中文词上屏了：记 `times` 份转移、推进链；`auto_word` 为真（用户自己选的词）时，
    /// 紧接着上一个词、合起来词库里没有、且这条接续记够次数还自动造词。
    pub(in crate::engine) fn record_word(
        &mut self,
        text: &str,
        syllables: &[String],
        times: u32,
        auto_word: bool,
        buffer_left: bool,
    ) {
        self.learner
            .record_transition(self.chain.context(), text, times);
        self.recording
            .push(Transition::new(self.chain.context(), text, times));
        if auto_word {
            let threshold = if self.chain.same_buffer() {
                AUTO_WORD_THRESHOLD_SAME_BUFFER
            } else {
                AUTO_WORD_THRESHOLD
            };
            self.try_auto_word(text, syllables, threshold);
        }
        self.chain.advance(text, syllables, buffer_left);
    }

    /// 上一个词 + 这个词合成用户词的条件见 [`AUTO_WORD_THRESHOLD`]。
    fn try_auto_word(&mut self, text: &str, syllables: &[String], threshold: u32) {
        let Some(previous) = self.chain.previous().map(str::to_owned) else {
            return;
        };
        let joined = format!("{previous}{text}");
        let chars = joined.chars().count();
        let mut joined_syllables = self.chain.previous_syllables().to_vec();
        joined_syllables.extend(syllables.iter().cloned());
        if chars > AUTO_WORD_MAX_CHARS || chars != joined_syllables.len() {
            return;
        }
        // 这条转移刚记过，计数已含本次；阈值按「选了几次」算，计数是按份记的
        let seen = self
            .learner
            .user_ngram()
            .map_or(0, |b| b.pair(Some(&previous), text));
        if seen < threshold * EXPLICIT_TRANSITION_WEIGHT {
            return;
        }
        let candidate = Candidate {
            text: joined,
            kind: CandidateKind::Chinese,
            syllables: joined_syllables,
            reading: None,
            translation: None,
            aux_code: None,
        };
        if self.knows_word(&candidate) {
            return;
        }
        tracing::debug!(text = %candidate.text, "自动造词");
        self.learner
            .learn_word(&candidate.text, &candidate.syllables);
    }

    /// 主词库或用户词里是否已有这个词（同音节）。
    pub(super) fn knows_word(&self, candidate: &Candidate) -> bool {
        let syllables: Vec<&str> = candidate.syllables.iter().map(String::as_str).collect();
        if syllables.is_empty() {
            return true;
        }
        let known = |dictionary: &Dictionary| {
            dictionary
                .lookup(&syllables, false)
                .iter()
                .any(|hit| hit.exact && hit.text == candidate.text)
        };
        self.all_dictionaries().into_iter().any(known)
    }
}
