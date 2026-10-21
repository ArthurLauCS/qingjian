---
title: Custom phrases
order: 3
description: "Typing a fixed phrase, address or long reply with a short input code and choosing its position among the candidates; on macOS, the system's text replacements work too."
---

In 设置 → 自定义短语 (Settings → Custom phrases), give a piece of text you use often an input code; when you type that code, the text appears at the position you chose among the candidates.

## Adding and editing

Each phrase is a row on the page: an on/off switch (labelled with the input code), its candidate position, the first line of its content, and 编辑 (Edit); the last row is 新增… (Add…).

Adding and editing happen in a card that pops up:

- **输入码 (Input code)**: 1–32 lowercase English letters, such as `dz`.
- **位置 (Position)**: 1–9, the candidate position this phrase takes.
- **内容 (Content)**: can span several lines and contain spaces, addresses, long replies or punctuation, and is committed exactly as written; the candidate window shows only a short preview of long text.

Click 保存 (Save) to apply; `Esc` or 取消 (Cancel) discards the changes. When editing, the card has 删除 (Delete) on its left, which needs two clicks to confirm.

For example, set position 1 of `ee` to `：` and position 2 to `；`: type `ee` and press Space to get ：, or press 2 to get ；.

## Position rules

- Ordinary pinyin candidates fill the other positions; neither your typing habits nor the local sentence model moves a phrase. AI candidates only use free slots that don't affect fixed positions.
- If a fixed position falls beyond the first page (for example position 9 with 5 candidates per page), AI candidates aren't shown that time; when positions span pages, you pick by the number on the current page.
- Positions for the same input code can't repeat; on a conflict, the card explains why and keeps your draft. Disabled phrases still hold their positions.
- When there aren't enough ordinary candidates, the slot is left empty, and an empty slot can't be committed.

Custom phrases don't appear in English mode, and with Traditional Chinese output on they're still committed as written.
If a phrase is changed in another settings window while you're editing it, saving warns you and keeps your draft instead of overwriting.

## System text replacements (macOS)

With 也用系统的文本替换 (Also use system text replacements) on (the default), entries in System Settings → Keyboard → Text Replacements also work while you type pinyin:
once you've typed the whole shortcut, the phrase appears in the first free slot (phrases from this page take their slots first), and `Space` or a number key commits it.
Only entries whose shortcut is all lowercase English letters work; entries added or changed in System Settings take effect the next time you switch to Qingjian. They aren't listed on this page; change them in System Settings.

Android's saved phrases are in the clipboard panel; see [Android: tools and AI](../platforms/android-tools.md#Clipboard-and-saved-phrases).
