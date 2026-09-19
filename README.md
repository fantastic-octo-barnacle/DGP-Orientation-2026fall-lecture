# 计算机开发基础 Slidev 工程

## 使用

使用 Node.js 24，pnpm 版本由 `package.json` 的 `packageManager` 固定。

```bash
pnpm install
pnpm dev
pnpm build
```

## 格式化

```bash
pnpm fmt        # 写入格式化结果
pnpm fmt:check  # 仅检查
```

## CI 与自动部署

[GitHub Actions 工作流](.github/workflows/ci.yml) 在推送到 `main`、向 `main` 提交 PR 或手动触发时，安装锁定依赖、检查格式并构建幻灯片。只有 `main` 的非 PR 运行会在检查通过后部署到 GitHub Pages。
