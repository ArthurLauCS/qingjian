---
title: Typing habits
order: 5
description: "What Qingjian remembers from your typing, how old habits slowly fade, deleting candidates you don't want, turning learning off and starting over."
---

Qingjian remembers your typing habits so that words you use often rank higher. Everything is recorded on your device and never uploaded.

## What it remembers

- Words you've picked, and which word you picked for a given stretch of pinyin.
- Which words you tend to type together, which sentence conversion adjusts to.
- Words you've created: when two words picked one after the other aren't in the dictionary together but you've picked them a few times, they're saved as one word; when you pick a stretch of pinyin in several steps and type it the same way a second time, it's saved as one word.
- AI candidates you've picked (recorded by the reading you typed).
- English words you've committed as typed.
- Spelling corrections you've accepted; see [Sentences, abbreviations and correction](../input/sentence.md#Remembering-habitual-typos).

If you pick the wrong word, delete it and retype, that pick isn't remembered; see [Picked the wrong word](../getting-started/first-input.md#Picked-the-wrong-word).

## Slowly fading

Pick counts cool off naturally over time: if you don't type a word for 90 days, its count is roughly halved, so old words make way for new ones and the candidates always follow your current habits.
Cooling only affects ranking; the records themselves stay.

## Deleting candidates you don't want

While typing pinyin, press `⇧ + number` (Windows / Linux: `Shift + number`) to delete candidate N on the current page:

- Words you've created and AI candidates you've picked are deleted entirely.
- Dictionary words aren't deleted, but all records about them are cleared and their original ranking is restored.

The result is shown briefly below the pinyin line (已删除用户词「…」 "user word deleted", or 没什么可删 "nothing to delete" when there's no record), and disappears with the next key.
The modifier key can be changed under 设置 → 快捷键 (Settings → Shortcuts); see [Changing keys](keys-settings.md#Glosses-and-deleting-candidates).

## Turning learning off and starting over

- With 学习输入习惯 (Learn typing habits) turned off under 设置 → 候选 (Settings → Candidates), nothing new is recorded, though what's already remembered still affects ranking.
- Nothing typed in password fields or in input fields an app marks as private is ever recorded.
- To start over, delete the typing-habit files in the data folder; see [Data files and logs](../advanced/data-files.md#Data-files). On Android you can clear everything with one tap in the Qingjian app; see [Android: vocabulary book and settings](../platforms/android-settings.md#Exporting-and-clearing).
