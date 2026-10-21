---
title: Sentences, abbreviations and correction
order: 1
description: "Typing whole sentences, abbreviated pinyin with initials only, correcting mistyped letters and remembering habitual typos, and the local sentence model that ranks sentences on your device."
---

## Sentences

There's no mode to switch: type a whole sentence and then commit it. The first candidate for `jintiantianqihenhao` is 今天天气很好.
When you reach the last character of a sentence, typing just its initial is enough to get the whole sentence early: the first candidate for `youdiandongx` is 有点东西, the same as for the full `youdiandongxi`.

When you pick a stretch of pinyin in several steps, the words picked first stay in the input box and the whole stretch is committed together once it's done; see [Picked the wrong word](../getting-started/first-input.md#Picked-the-wrong-word).

## Abbreviated pinyin

Typing only each character's initial works too: `jttqhh` gives 今天天气很好.
Abbreviations of two or three letters are usually still treated as words (`sj` gives 时间); type a few more syllables when you want a sentence.

## Spelling correction

- **When the pinyin doesn't spell anything valid** (an extra letter, a missing letter, two keys swapped, one wrong letter): `nihooma` → 你好吗, `woxiagnqu` → 我想去, `mingtain` → 明天.
  The pinyin line marks the letters being changed with a strikethrough and keeps them in place; all candidates are based on the corrected pinyin.
  You don't have to finish the word when two neighbouring keys are swapped: `mignt` and `migntia` already give 明天, so just keep typing.
  When both kinds of correction make sense, a swap is preferred (`mignti` → 命题, rather than 面体 by changing one letter).
- **When every syllable is valid but the sentence makes no sense**: `meiganxi` still gives 没关系 (`gan` is read as `guan`). This kind of correction only affects sentences, not single-word candidates.

A correction is only made when the corrected sentence is clearly more natural than the original; valid abbreviations (`nhao`, `womend`) aren't "corrected" by mistake, and input that is entirely an English word (`hello`) isn't corrected either.

### When you don't want a correction

`Enter` still commits the letters exactly as typed. A letter string you've committed as typed once is never corrected again.

### Remembering habitual typos

Corrections you accept are remembered: if you habitually type `shuo` as `shou`, after a few times 我说 will naturally move up. Deleting with `Backspace` and picking something else also undoes that record.

There's no correction with double pinyin or Wubi: a wrong key in double pinyin changes a whole initial or final, and Wubi keys are radical codes, so neither is a case of misspelling one letter.

## Local sentence model

Qingjian ships with a small model that reorders sentence candidates on your device so that the first one reads more naturally. It runs entirely offline: no network, nothing sent anywhere.

- Pause briefly after typing (less than a tenth of a second) and the sentence candidate is replaced with the one the model finds more natural; until you pause, the sentence built from the dictionary is shown.
- The model only adjusts sentence candidates; when the dictionary can't build a full sentence, it may fill one in. Word candidates and words you've picked before aren't affected, and your own habits still come first.
- Once you've paged further or moved the highlight with the arrow keys, the model's result no longer replaces the page you're looking at.
- For the first few seconds after the input method starts, the model is still loading and sentences come from the dictionary.
- On older Macs (for example, macOS 14 without the required graphics features), the model runs on the processor instead and ranking may be a little slower.

If you don't want it, turn off 本地整句模型 (Local sentence model) under 设置 → 候选 (Settings → Candidates); ranking then uses the dictionary only. This doesn't affect AI services.
To use your own model, see [Data files and logs](../advanced/data-files.md#Using-your-own-local-sentence-model).
