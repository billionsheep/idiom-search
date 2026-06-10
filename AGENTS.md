# AGENTS.md

本项目是轻量化成语检索工具，默认使用简体中文沟通和编写文档。

## 项目规则

- 保持项目自包含，不依赖外部 MySQL、Redis 或后端服务。
- 默认数据文件为 `data/idioms.sqlite`，应随代码一起提交。
- 修改数据时优先更新 `data/idioms_seed.sql`，再重新生成 SQLite 文件。
- 不提交敏感信息、账号密码、API Key 或版权不明的大段文本。
- 每次提交应保持 `cargo run -- exact 画蛇添足` 可运行。
