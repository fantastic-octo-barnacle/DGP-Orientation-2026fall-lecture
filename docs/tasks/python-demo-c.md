# Python Demo C：完整工具链仓库

本仓库由出题方提供。它用于展示一个可复现的现代 Python 项目如何组织，不作为候选人的编程成果。

## 示例主题

建议使用“图书信息展示器”等与正式日志检查器不同的业务主题。程序读取一份很小的图书 JSON 数据，使用 Pydantic 验证，使用 Typer 接收参数，并使用 Rich 显示结果。

## 仓库内容

```text
python-toolchain-demo/
├── src/
│   └── toolchain_demo/
├── tests/
├── .gitignore
├── .python-version
├── pyproject.toml
├── uv.lock
└── README.md
```

项目配置：

- uv 项目及锁文件。
- pytest 测试。
- Ruff 格式化与基础代码检查。
- Pyright `basic` 类型检查。
- `src` 项目结构。
- Python 版本约束。

仓库不包含 `.venv`、工具缓存、IDE 个人配置或现成的 CI workflow。

## 候选人任务

1. 从 GitHub 取得仓库，并恢复本地项目环境。
2. 运行程序及测试。
3. 分别运行 Ruff 和 Pyright。
4. 修改代码以观察一次格式问题。
5. 修改代码以观察一次类型问题。
6. 修改程序行为以观察一次测试失败。
7. 删除本地虚拟环境，再恢复并运行项目。
8. 解释各项目文件的用途，以及哪些内容应该或不应该提交到 Git。

## 设计要求

- 示例程序本身保持很小，候选人不需要理解复杂业务。
- 各类工具应能报告彼此不同的问题。
- 任务说明提供目标、资料方向和搜索关键词，不给完整命令答案。
- 正式 Python 项目不能通过直接复制本 Demo 的业务代码完成。
