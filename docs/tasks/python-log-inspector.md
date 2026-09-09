# Python 编程项目：结构化日志检查器

候选人自行创建 uv 项目和个人 GitHub 仓库，实现一个命令行结构化日志检查器。业务逻辑保持简单，重点观察 Python 基础、依赖使用、输入验证、错误处理、测试、项目配置和版本管理。

## 输入格式

输入采用 JSON Lines：每一行是一条独立 JSON 记录。

```json
{"time":"2026-09-01T12:00:00+08:00","level":"INFO","source":"client","message":"connected"}
```

字段要求：

- `time`：带时区的 ISO 8601 时间。
- `level`：`DEBUG`、`INFO`、`WARNING` 或 `ERROR`。
- `source`：去除首尾空白后为非空字符串。
- `message`：去除首尾空白后为非空字符串。

公开异常类型包括非法 JSON、缺少字段、字段类型错误、非法时间、未知等级、空字符串和多余字段。

验证规则：

- `level` 严格使用公开的大写值，不自动修正大小写。
- 多余字段视为非法，Pydantic 使用禁止额外字段的配置。
- 空白行视为一条非法记录。
- 每条记录独立验证，一条错误不影响后续记录。

## 固定依赖

运行依赖：

- Pydantic
- Typer
- Rich

开发依赖：

- pytest
- Ruff
- Pyright，使用 `basic` 模式

## 命令行行为

基础命令：

```text
log-inspector LOG_FILE
```

标准层增加两个筛选选项：

```text
log-inspector LOG_FILE --level ERROR
log-inspector LOG_FILE --source client
log-inspector LOG_FILE --level ERROR --source server
```

规则：

- `level` 只接受公开的四个等级。
- `source` 使用精确字符串匹配。
- 同时指定时采用“且”关系。
- 所有输入行先验证，再对合法记录进行筛选。
- 非法记录不能因筛选条件而被静默忽略。
- 保留匹配记录在输入文件中的原始顺序。

## 输出

输出至少包括：

```text
Total lines: 100
Valid records: 92
Invalid records: 8
Matched records: 17
```

标准层使用 Rich 显示匹配记录和等级统计。非法记录至少报告行号和简短原因，不要求原样显示 Pydantic 的完整内部错误。

总行数、合法数和非法数统计整个文件；`Matched records` 只统计满足筛选条件的合法记录。文件成功读取时，即使包含非法记录也正常退出；文件不存在、无法读取或命令参数非法时返回非零退出状态。不增加 `--strict` 选项。

## 项目层级

### 基础层

- 逐行读取文件。
- 使用 Pydantic 验证记录。
- 统计合法与非法记录。
- 隔离错误行并继续处理。
- 输出基本汇总。

### 标准层

- 使用 Typer 实现文件参数、`level` 和 `source` 筛选。
- 使用 Rich 输出结果。
- 拆分读取、验证、筛选、统计和显示逻辑。
- 使用 pytest 覆盖正常与异常输入。
- 配置并运行 Ruff 和 Pyright。

### 进阶层

- 支持 JSON 报告输出。
- 流式处理较大文件。
- 生成独立错误报告。
- 使用 CI 自动执行测试和静态检查。

## 公开数据集

- `small.jsonl`：24 行，其中 18 条合法、6 条非法，便于手工核对。
- `mixed.jsonl`：240 行，覆盖所有公开异常类型。
- `large.jsonl`：由出题方提供确定性生成脚本，生成约 10 万行数据，供进阶层验证流式处理。

不设置隐藏字段或隐藏异常类型。示例数据不包含隐私、敏感内容或需要专业背景理解的字段。

## 仓库要求

仓库包含源码、测试、`pyproject.toml`、锁文件、必要的 Python 版本信息、README 和恰当的 `.gitignore`。不提交 `.venv`、工具缓存或程序生成的报告。

Ruff、Pyright 或测试尚有问题时，不自动淘汰候选人；其理解、定位和修复计划会在面试中核验。
