# 通用参考程序（内部维护）

完整参考实现，仅在内部设计仓库维护。候选人仓库只获取三个平台的可执行文件、版本和校验信息，不发布本目录源码或完整行为测试。

运行 cargo test、cargo clippy --all-targets -- -D warnings 和 cargo build --release。实现协议 v1 的全部动作，候选人只实现所选项目子集。

发布验证包括：所有动作、错误校验、跨连接并发、读取超时、空闲连接结束、在途请求宽限及超期取消；另通过 verify_delivery.py 验证候选人基线和参考程序的实际互通。

本机使用 Windows GNU 工具链；发布 Windows 使用 MSVC 工具链，Linux 与 macOS 采用对应原生 runner。候选人代码的跨平台验证快照保存在内部 tools/delivery-validation/candidate 下，不包含 Git 历史、构建产物和虚拟环境。
