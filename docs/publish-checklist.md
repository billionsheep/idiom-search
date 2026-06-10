# 发布与登记清单

## 1. 创建 GitHub 仓库

`idiom-search` 是独立轻量项目，应创建独立仓库，不放进 `osh-backend` 或 `osh-frontend`。

优先创建组织仓库：

- 仓库名：`idiom-search`
- 组织：`juege-osh`
- 可见性：公开
- 不要初始化 README、LICENSE 或 `.gitignore`，本地项目已经包含这些文件。

如果没有组织权限，先创建个人仓库：

- 仓库名：`idiom-search`
- 可见性：公开
- 后续拿到权限后再迁移到 `juege-osh`。

## 2. 推送本地代码

在本地执行：

```bash
cd /Users/moon/Workspace/projects/juege/idiom-search
git remote add origin <GitHub 仓库 SSH 地址>
git push -u origin main
```

示例：

```bash
git remote add origin git@github.com:billionsheep/idiom-search.git
git push -u origin main
```

## 3. 推送前验证

```bash
cd /Users/moon/Workspace/projects/juege/idiom-search
./scripts/verify.sh
```

验证重点：

- 数据总数不少于 100 条。
- `画蛇添足` 可以精确查询。
- `多此一举` 可以模糊查到相关成语。
- 冷僻成语列表可以输出。

## 4. 禅道登记

当前 `Leon` 账号可以登录，并已创建个人待办 `#1 成语检索工具 MVP`。

如果团队要求正式产品研发需求，需要管理员给 `Leon` 增加 `osh-platform` 下创建研发需求的权限。

登记内容直接复制：

- 文件：`docs/zentao-entry.md`
- 标题：`成语检索工具 MVP`
- 类型：需求或任务，按团队禅道配置选择。

## 5. 群同步

仓库推送后发：

```text
成语检索工具 MVP 已完成第一版：Rust + SQLite，数据文件随代码提交 GitHub；当前 161 条成语，支持精确查询、关键词查询、冷僻成语列表和统计。仓库：<仓库地址>。下一步补充更多冷僻成语数据和标签筛选。
```
