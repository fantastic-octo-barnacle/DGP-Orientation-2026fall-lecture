# 候选人交付 v1 验证记录

## 交付位置

- 候选人仓库：[DGP-Orientation-2026fall-projects](https://github.com/fantastic-octo-barnacle/DGP-Orientation-2026fall-projects)，private，已启用 GitHub Template。
- 候选人基线提交：`4df1b44`。
- 参考程序发布：[reference-v1.0.0](https://github.com/fantastic-octo-barnacle/DGP-Orientation-2026fall-projects/releases/tag/reference-v1.0.0)。
- 内部实现与构建分支：`delivery/candidate-v1`，源码没有合并到内部 main。
- 本地候选人目录与设计仓库同级，名称为 `DGP-Orientation-2026fall-projects`。

## 范围与证据

五项材料已交付：根 README、共通指引、独立项目任务书、Python 与三个 Rust 基线项目、分阶段自查及验收说明。

独立 Demo C 已取消；仓库不含活动日期、检查点时间、提交渠道或求助渠道。个人技术成果要求仍保留。两个项目共享消息协议，唯一参考程序覆盖全部动作，候选人只实现所选项目。

[原生平台验证运行](https://github.com/fantastic-octo-barnacle/DGP-Orientation-2026fall-training/actions/runs/35257409909)全部成功：

| 平台 | 基线检查 | 参考程序测试 | 实际互通 |
|---|---|---|---|
| Windows x86-64（MSVC） | 通过 | 通过 | 通过 |
| Linux x86-64 | 通过 | 通过 | 通过 |
| macOS Apple Silicon | 通过 | 通过 | 通过 |

每个平台执行：

- Python：锁文件恢复、Ruff 格式与静态检查、Pyright、pytest。
- 三个 Rust 项目：分别 fmt、Clippy、test、build，使用各自锁文件。
- 参考程序：格式与 Clippy、全动作与错误校验、跨连接并发、读取超时、空闲连接结束、在途请求宽限和超期取消。
- 跨进程验证：Python 客户端到参考服务端、参考客户端到 Python 服务端、参考客户端到两种 Rust 服务端，以及同步 Rust 客户端互通。

本机另使用 Python 3.12.14、uv 0.12.15 和 Rust 1.98.1 GNU 工具链验证。25 个候选人本地文档链接均可解析。CI 内部快照与候选人仓库 42 个文件逐项哈希一致。

## 发布文件

已核对可执行文件的实际架构，并运行 Windows 发布文件的 --version。Release 附 SHA256SUMS：

| 文件 | SHA-256 |
|---|---|
| rm-recruit-reference-linux-x86_64 | 9873249cf9cc0e671bc6f4fd101a3b1da1214db796ad5218bc592588b0b7a41a |
| rm-recruit-reference-macos-arm64 | 15c50578f9e95ef59af68f34239e6166654d8b879609c1ee35e367c02f58c539 |
| rm-recruit-reference-windows-x86_64.exe | d7b5f30f0f64d84e56fded7ee6f74ea915cf53b9f8e6adbb158b6969be7a3a51 |

## 维护约定

- 完整参考源码及行为测试在 tools/reference-program，仅留在内部仓库。
- tools/delivery-validation/candidate 为本次候选人基线的精确验证快照。更新候选人代码后同步快照，再运行内部平台验证。
- 参考程序的完整 CI 留在内部，不给候选人提供可直接复制的 CI 答案。
- 任何后续协议变化需同步任务书、基线、参考程序、测试与发布版本。
- 当前仓库仍为 private；公开可见性和正式候选人访问范围尚未调整。
