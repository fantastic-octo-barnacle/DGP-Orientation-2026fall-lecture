# Foundation Demo

本仓库用于 2 小时基础课的贯穿演示和短课堂活动。它不是正式招新项目，也不用于评价候选人。

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

1. 展示源代码是纯文本。
2. 比较 Python 解释运行与 Rust 编译运行。
3. 展示缺少依赖或环境时的失败。
4. 展示第三方包、版本要求、直接依赖和间接依赖。
5. 展示项目元数据、锁文件与生成环境的区别。
6. 修改根目录 README，观察 Git 状态和 diff。
7. 比较 pytest、Ruff、Pyright 与 Cargo 对应工具。
8. 说明 CI 如何在新环境中重复这些检查。

## 约束

- 业务代码应足够小，讲师不需要解释其算法。
- 候选人的课堂活动不依赖 Python 或 Rust 工具链已经安装。
- 根目录 README 的 Git 活动应能在只安装 Git 的情况下完成。
- 正式代码、测试和故障场景在逐章讲义设计完成后实现。
