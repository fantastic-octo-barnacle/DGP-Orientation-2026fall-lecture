# Foundation Demo

`foundation-demo` 最终作为独立公开 Git 仓库发布，供候选人在 2 小时基础课的 Git 和开发工具环节 clone。它不是课程开场案例，不承担全课主叙事，也不是正式招新项目或评价材料。

当前目录保存该公开仓库的设计说明；实际仓库需要单独初始化、发布和固定当届使用的 tag，不得包含内部面试或评委材料。

## 计划结构

```text
foundation-demo/
├── README.md
├── python-demo/
│   ├── main.py
│   ├── pyproject.toml
│   └── uv.lock
└── rust-demo/
    ├── Cargo.toml
    ├── Cargo.lock
    └── src/main.rs
```

两个程序实现相同的极小行为。课堂不解释语言语法，只比较源码、解释器或编译器、依赖声明、锁文件、生成环境、构建产物、测试和静态检查工具。

## 课堂用途

1. 候选人 clone 独立公开仓库。
2. 修改根目录 README，观察 Git 状态和 diff。
3. 讲师在需要时用其中的极小项目演示项目元数据、锁文件和生成环境。
4. 比较 pytest、Ruff、Pyright 与 Cargo 对应工具。
5. 说明 CI 如何在新环境中重复这些检查。

## 约束

- 业务代码应足够小，讲师不需要解释其算法。
- 候选人的课堂活动不依赖 Python 或 Rust 工具链已经安装。
- 根目录 README 的 Git 活动应能在只安装 Git 的情况下完成。
- 正式代码、测试和故障场景在逐章讲义设计完成后实现。
