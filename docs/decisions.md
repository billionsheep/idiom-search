# 决策记录

## 2026-06-10：项目采用独立仓库

结论：`idiom-search` 应作为独立轻量化仓库推进，不放进 `osh-backend` 或 `osh-frontend`。

原因：

- 项目目标是自包含的成语检索工具，用户 clone 后即可运行。
- 数据文件 `data/idioms.sqlite` 需要随代码提交，和主站数据库、接口、发布流程无关。
- 放入主站前后端会引入不必要的依赖、评审和发布耦合，降低小项目交付速度。
- 独立仓库更符合“轻量化项目数据文件直接跟代码一起上传 GitHub”的要求。

执行方式：

- 本地暂放在 `/Users/moon/Workspace/projects/juege/idiom-search`，与 `osh-backend`、`osh-frontend` 平级。
- 远程优先创建 `juege-osh/idiom-search`。
- 如果暂时没有组织权限，先创建个人仓库 `billionsheep/idiom-search` 或你自己的 GitHub 账号下同名仓库，后续再迁移到组织。
