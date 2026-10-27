---
title: Traditional Chinese output
order: 8
description: "Turning candidates and committed text into Traditional Chinese: the differences between the Taiwan, Hong Kong and standard forms, and why you still type Mandarin pinyin and mainland vocabulary."
---

The way you type doesn't change: you still type Mandarin pinyin. Only the characters shown in the candidate window and the characters committed change.

## Turning it on

设置 → 候选 → 输出字形 (Settings → Candidates → Output characters):

| Option | Effect |
|---|---|
| 简体 (Simplified) | No conversion (default) |
| 繁体（台湾） (Traditional, Taiwan) | Taiwan vocabulary and character forms: 软件 → 軟體, 这里 → 這裡 |
| 繁体（香港） (Traditional, Hong Kong) | Hong Kong character forms |
| 繁体（通用字形） (Traditional, standard forms) | Standard Traditional forms: 这里 → 這裏 |

On Windows you can also turn on a shortcut and press `Ctrl + Shift + F` to switch between Simplified and Traditional; see [Changing keys](../personalize/keys-settings.md#Shortcut-for-switching-Simplified-and-Traditional-(Windows)).

The Taiwan option offers Taiwan vocabulary first, then the form that only changes characters: `ruanjian` gives 軟體 first and 軟件 second. The Hong Kong and standard options have no vocabulary table and only change character forms. Taiwan and Hong Kong choose different forms (裡 / 裏, 著 / 着), hence the two separate options. Changes take effect immediately.

A single character lists every Traditional character it stands for: `gan` offers 幹, 乾 and 干. The form you pick is remembered and comes first next time for the same word; picking the default one again (the vocabulary one in the Taiwan option) restores the usual order. Each option remembers separately. Sentence candidates come in one form only, with Taiwan vocabulary in the Taiwan option.
Candidates and sentence completions from AI are converted too. On Android, choose it in the keyboard settings; see [Android: vocabulary book and settings](../platforms/android-settings.md).

## Typing habits are always recorded in Simplified

The dictionary, the words you've picked, the words you've created and your typing statistics are all recorded in Simplified Chinese. So:

- Words learned while Traditional output is on still rank first after you switch back to Simplified, and vice versa.
- Deleting a candidate deletes that Simplified record.
- The Traditional forms you pick are kept in a separate table and only change the order of the forms.
- Glosses beside candidates are shown as usual.

Custom phrases are committed exactly as you wrote them, and Traditional output doesn't change them. If you want a phrase in Traditional, write it in Traditional.

## You still type Mandarin pinyin and mainland vocabulary

The conversion happens when candidates are shown and committed; the dictionary itself doesn't change. So:

- **Readings follow Mandarin**: 垃圾 is typed `laji` (not the Taiwanese reading `lese`), and 企划 is typed `qihua`.
- **Vocabulary follows mainland usage**: the dictionary uses mainland words, and the Taiwan option can only swap in the Taiwan vocabulary its conversion table knows; the Hong Kong and standard options keep the vocabulary as it is.
- **Characters with several Traditional forms are occasionally wrong**: when one Simplified character maps to several Traditional ones (后 → 後 / 后, 发 → 發 / 髮), the conversion decides by word, and uncommon combinations may get the wrong one. When that happens, type just that character and pick from its Traditional characters; after that it comes first when you type that character alone.

If you need full Taiwanese readings and local vocabulary, you need a separate Traditional Chinese dictionary; this switch can't do that.
