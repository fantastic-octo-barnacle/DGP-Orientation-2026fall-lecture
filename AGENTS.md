# Repository Guidance

本仓库用于设计和维护 RM 软件组招新培训、个人项目与面试材料，正文默认使用中文。

## 开始工作前

- 修改术语或方案边界前，先阅读 [`CONTEXT.md`](./CONTEXT.md)。
- 已确认的方案位于 [`docs/design/`](./docs/design/)；早期题目和项目草案位于 [`docs/drafts/`](./docs/drafts/)，不得视为定稿。
- 按 [`docs/design/roadmap.md`](./docs/design/roadmap.md) 的顺序推进：培训 → 正式项目 spec → 最终面试。

## 目录职责

- `training/`：intro 交接、预备学习和两小时基础课讲义。
- `slides/`：Slidev 幻灯片源码及其静态资源。
- `demos/foundation/`：本仓库内、供两小时基础课使用的演示项目。
- `demos/projects/`：正式项目配套的示例项目与参考实现。
- `docs/adr/`：关键且长期有效的设计决策。

课堂 demo 与正式项目 Demo C 是不同材料。候选人可见材料不得混入内部面试题、评价记录或标准答案。

## 工具与维护

- 使用 `rg` 搜索；使用 `uv` 运行 Python 脚本。
- JavaScript/Slidev 依赖和脚本使用 `pnpm`；TeX 内容优先使用 `tectonic`。
- 除非用户明确要求，不使用额外的开发工作流技能或插件。
- 移动或重命名文件后，更新相关索引并检查所有 Markdown 相对链接。
