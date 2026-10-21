---
title: 本地整句模型
order: 5
description: 本地模型在本机给候选重新排序：怎么生效、实验功能里的知意与知微、什么时候不生效、怎么关。
---

青简的本地模型系列叫「含章」。含章·通变（Hanzhang Tongbian）负责拼音解码与纠错；含章·知微（Hanzhang Zhiwei）负责根据上下文重排候选。安装包只带通变。全程离线，不联网、不发送任何内容。

## 实验功能

macOS 上「设置 → 实验功能」可以下载另外两个模型：

- **知意**（推荐）：读光标前的字与敲的拼音，从候选窗前几条里挑首选，由[知意输入法](https://github.com/dd1000001000/zhiyi_ime)的作者训练并授权，约 154 MB。开着时可以选档位：省电、均衡、准确，越往后比的候选越多、看的前文越长，也越费电。
- **含章·知微**：读光标前的字给整句候选重新排序，约 52 MB。

点「下载」后显示进度，下完自动启用；同一时间只用一个模型，开了一个就关掉另一个，都关着时用通变。模型从 Gitee 下载，不通时改从 GitHub，每一块都核对校验值。
配置文件里对应 `[model]` 分节的 `name`（`tongbian` / `zhiwei` / `zhiyi`）与 `zhiyi_tier`（`saver` / `balanced` / `accurate`）；选了没下载的模型时用通变。Windows 与 Linux 还没接这两个模型。

## 怎么生效

敲完一段拼音停一下（不到十分之一秒），整句候选就换成模型认为更通顺的那句；没停就按词库统计出的先显示。
通变根据这段拼音选句，不读取光标前文。知微与知意会参考光标前面的几十个字；应用不让读上下文时（部分终端、Electron 应用），改看你刚才在这个输入框里打的字，换了应用或输入框就从头算，一分钟没打字也不再算。这个时长可以在配置文件 `[model]` 分节的 `context_memory_seconds` 里改，填 0 表示一直算。Linux 目前也只看在这个输入框里打的字。

模型会调整整句候选；词库拼不出完整句子时，还可能补出整句候选。词候选、你选过的词都不受它影响；你的习惯仍然优先。

macOS 14 等缺少所需 Metal 功能的系统会自动用 CPU 运行本地模型，加载和重排可能更慢。

## 什么时候不生效

- 已经翻到后面的页或用方向键移过高亮时，模型的结果不再换掉正在看的这页。
- 密码框里不组句；应用声明为私密的输入框里不读上下文。
- 输入法刚启动的几秒钟模型还在加载，这段时间按词库统计出句。

## 关掉

「设置 → 候选」关闭「本地整句模型」，或配置文件 `[model]` 里 `enabled = false`（Linux 只有这一种，保存即生效）。关闭后只用词库统计，与云联想互不影响。

## 自己的模型

把 `hanzhang-tongbian-small.qjm` 放进用户数据目录的 `models/hanzhang-tongbian/`，重启输入法后优先使用。含章·知微使用 `hanzhang-zhiwei-small.qjm`，放在 `models/hanzhang-zhiwei/`，在「实验功能」里打开（或 `[model] name = "zhiwei"`）后使用。旧用户目录 `model-p2c/` 与 `model/` 仍可读取；同一模型的新目录优先。直接导出的 `model.safetensors`、`config.json`、`vocab.json` 三个文件也能识别；同目录里两种格式都有时优先用 `.qjm`：

- **macOS**：`~/Library/Application Support/Qingjian/models/hanzhang-tongbian/`
- **Windows**：`%APPDATA%\Qingjian\models\hanzhang-tongbian\`
- **Linux**：`~/.local/share/qingjian/models/hanzhang-tongbian/`
