# RM 软件组招新考核方案

本仓库记录 RM 软件组面向大一新生的招新培训、个人项目和面试方案。考核的目标是识别值得长期培养的候选人，而不是要求候选人在录取时已经具备正式业务开发能力。

## 基本流程

1. 面向所有方向的概述课，介绍软件、机械和硬件等方向。
2. 软件组进行一节 2 小时的计算机基础与学习方法课。
3. 候选人根据既有编程基础选择 Python 或 Rust 起点路线。
4. 候选人在约一个月内自行学习并完成个人项目。
5. 评委检查代码仓库，并通过面试核验理解、学习过程和解决问题的能力。
6. 从约 40–50 名报名者中录取约 7–8 名预备队员，进入后续长期培养与实习阶段。

## 设计原则

- Python 路线服务于尚不能独立编写简单程序的候选人；Rust 路线服务于已掌握任意语言基本编程概念的候选人。
- 项目业务逻辑保持简单，通过环境、工具、文档、调试、错误处理和逐层改进提供学习机会。
- 项目完成度只是证据之一，不是机械的录取门槛。
- 候选人在项目期间可以搜索、使用 AI、讨论和求助，但必须理解并能够解释自己的关键成果。
- 面试主要核验理解和思路，不设置统一算法笔试，也不要求复杂的现场编程。
- 具体评价维度暂不设置数值权重。

## 文档

- [术语表](./CONTEXT.md)
- [共同基础任务](./docs/common-foundations.md)
- [培训设计](./docs/training.md)
- [Python 起点路线](./docs/python-track.md)
- [Rust 起点路线](./docs/rust-track.md)
- [面试设计](./docs/interview.md)
- [面试题库（内部）](./docs/interview-question-bank.md)
- [评审框架](./docs/evaluation-rubric.md)
- [核心选拔原则 ADR](./docs/adr/0001-select-for-learning-potential.md)

具体任务草案位于 [`docs/tasks/`](./docs/tasks/)：

- [Git 与 GitHub](./docs/tasks/git-github.md)
- [WSL2](./docs/tasks/wsl2.md)
- [Python Demo C](./docs/tasks/python-demo-c.md)
- [Python 结构化日志检查器](./docs/tasks/python-log-inspector.md)
- [Rust 通信协议](./docs/tasks/rust-protocol.md)

## 当前状态

总体结构和边界已经确认。具体项目任务书、测试数据、参考程序协议细节、课程讲义、面试题库和命令行实操题将在后续设计中补充。
