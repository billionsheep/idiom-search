# idiom-search

轻量化成语检索工具。项目使用 Rust 编写，数据使用 SQLite 保存，并且 `data/idioms.sqlite` 会随代码一起提交到 GitHub。用户 clone 后不需要部署 MySQL、Redis 或后端服务，就可以直接查询成语。

## 功能

- 精确查询成语。
- 按关键词模糊查询成语、拼音、释义、出处、典故、例句和标签。
- 列出较少见或冷僻成语。
- 输出读音、释义、出处、典故、例句、冷僻程度和标签。
- 支持通过环境变量切换 SQLite 数据文件。

## 环境要求

- Rust 1.94 或更新版本。

## 快速开始

```bash
cargo run -- stats
```

精确查询：

```bash
cargo run -- exact 画蛇添足
```

关键词查询：

```bash
cargo run -- search 多此一举
cargo run -- search 战国 --limit 5
```

列出冷僻成语：

```bash
cargo run -- list --rare
```

使用自定义数据文件：

```bash
IDIOM_DB_PATH=/path/to/idioms.sqlite cargo run -- exact 画蛇添足
```

## 数据字段

SQLite 表名为 `idioms`，字段如下：

| 字段 | 含义 |
| --- | --- |
| `word` | 成语 |
| `pinyin` | 拼音/读音 |
| `meaning` | 释义/翻译 |
| `origin` | 出处 |
| `story` | 典故 |
| `example` | 例句 |
| `rare_level` | 常用、较少见、冷僻 |
| `tags` | 分类标签 |

## 数据文件

- `data/idioms.sqlite`：运行时直接读取的 SQLite 数据文件。
- `data/idioms_seed.sql`：可审阅、可扩充的种子 SQL。

重新生成数据文件：

```bash
rm -f data/idioms.sqlite
sqlite3 data/idioms.sqlite < data/idioms_seed.sql
```

当前第一版数据为人工整理的常见成语和部分较少见/冷僻成语，用于项目 MVP 演示和后续扩展。后续补充外部数据时，需要在本文件中补充来源说明，并避免引入版权不明的大段文本。

## 持续推进方向

- 每次提交前运行 `./scripts/verify.sh`，确保项目保持可运行。
- 补充更多冷僻成语和古语。
- 增加按标签筛选。
- 增加导出 JSON/CSV。
- 后续可扩展 Web 页面或 HTTP API。

## 发布与禅道登记

- GitHub 发布步骤见 `docs/publish-checklist.md`。
- 禅道需求模板见 `docs/zentao-entry.md`。
- 项目进展记录见 `docs/progress.md`。
