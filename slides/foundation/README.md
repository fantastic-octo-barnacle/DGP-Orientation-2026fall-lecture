# 两小时基础课幻灯片

这里保存两小时“计算机开发基础”课程的正式 Slidev 工程。

## 使用

```bash
pnpm install
pnpm dev
pnpm build
```

课件入口是 [`slides.md`](./slides.md)。内容依据 [`training/foundation/`](../../training/foundation/) 中的讲义结构、各节内容和讲师备注；Slides 负责课堂呈现，不替代讲义。

## 约定

- 使用 `@ktym4a/slidev-theme-ktym4a`，当前启用主题的颜色轮转模式。
- 使用 `slidev-addon-counter`：默认 `<Counter />` 为 level 2，显示 `1.1`、`1.2`；大节使用显式的 level 1，显示 `1`、`2`。
- 标题页、课程地图、总结和路线交接页不参与计数；讲义 `00–10` 对应 11 个大节。
- 终端演示优先使用 Slidev 与主题原生代码块，直接展示 prompt、命令和输出。
- 当前只制作课件，不实现 `demos/foundation/` 的项目代码；课件中的示例是静态教学素材。
