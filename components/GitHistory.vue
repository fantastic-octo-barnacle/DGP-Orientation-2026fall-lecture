<script setup lang="ts">
import { computed } from "vue";

type Kind = "history" | "branch" | "revert" | "cherry-pick" | "remote";
type Node = {
  id: string;
  label: string;
  x: number;
  y: number;
  message: string;
  tone?: string;
  from?: number;
  enterY?: number;
};
type Edge = { a: string; b: string; from?: number; copy?: boolean };
type Scene = {
  nodes: Node[];
  edges: Edge[];
  lanes?: { label: string; y: number }[];
  captions: string[];
};

const props = defineProps<{ kind: Kind; step: number }>();

// Each slide owns its click count. Keeping frames declarative also makes going
// backwards and jumping directly to a click restore the exact same diagram.
const scenes: Record<Kind, Scene> = {
  history: {
    nodes: [
      {
        id: "a",
        label: "A",
        x: 110,
        y: 105,
        message: "创建项目（最早的提交）",
      },
      { id: "b", label: "B", x: 310, y: 105, message: "补充使用说明" },
      { id: "c", label: "C", x: 510, y: 105, message: "修正单位标注" },
      {
        id: "d",
        label: "D",
        x: 710,
        y: 105,
        message: "增加示例（最新的提交）",
      },
    ],
    edges: [
      { a: "a", b: "b" },
      { a: "b", b: "c" },
      { a: "c", b: "d" },
    ],
    captions: [
      "每个圆点都是一次 commit：可以读到改动说明，并查看当时的文件。",
      "回到 B 查看当时的版本；后面的 C、D 仍然保留在历史中。",
      "再回到 D，继续当前的工作。查看旧版本不等于删除后来的提交。",
    ],
  },
  branch: {
    lanes: [
      { label: "主线", y: 30 },
      { label: "实验分支", y: 160 },
    ],
    nodes: [
      { id: "a", label: "A", x: 100, y: 75, message: "已有功能" },
      { id: "b", label: "B", x: 305, y: 75, message: "修正说明", from: 1 },
      {
        id: "c",
        label: "C",
        x: 305,
        y: 205,
        message: "尝试新界面",
        tone: "text-peach",
        from: 1,
      },
      {
        id: "d",
        label: "D",
        x: 510,
        y: 205,
        message: "完善交互",
        tone: "text-peach",
        from: 1,
      },
      {
        id: "m",
        label: "M",
        x: 710,
        y: 75,
        message: "合并两边的变化",
        tone: "text-green",
        from: 2,
      },
    ],
    edges: [
      { a: "a", b: "b", from: 1 },
      { a: "a", b: "c", from: 1 },
      { a: "c", b: "d", from: 1 },
      { a: "b", b: "m", from: 2 },
      { a: "d", b: "m", from: 2 },
    ],
    captions: [
      "从已有版本出发，为一个新想法开出独立分支。",
      "分支里试新界面，主线继续修说明；两条开发路径可以各自推进。",
      "准备好后合并：把两边的变化整合到一起，保留各自的提交记录。",
    ],
  },
  revert: {
    nodes: [
      { id: "a", label: "A", x: 110, y: 105, message: "原有行为" },
      {
        id: "b",
        label: "B",
        x: 310,
        y: 105,
        message: "启用提示音",
        tone: "text-red",
      },
      { id: "c", label: "C", x: 510, y: 105, message: "补充帮助文档" },
      {
        id: "r",
        label: "R",
        x: 710,
        y: 105,
        message: "撤销 B 的改动",
        tone: "text-green",
        from: 1,
      },
    ],
    edges: [
      { a: "a", b: "b" },
      { a: "b", b: "c" },
      { a: "c", b: "r", from: 1 },
    ],
    captions: [
      "提示音效果不好，但之后补充的帮助文档还需要保留。",
      "新增 R 撤销 B 的效果：提示音关闭，C 的文档仍在，B 也仍可查到。",
    ],
  },
  "cherry-pick": {
    lanes: [
      { label: "开发分支", y: 30 },
      { label: "发布分支", y: 160 },
    ],
    nodes: [
      { id: "a", label: "A", x: 100, y: 75, message: "共同起点" },
      {
        id: "b",
        label: "B",
        x: 305,
        y: 75,
        message: "开发新功能",
        tone: "text-peach",
      },
      {
        id: "c",
        label: "C",
        x: 510,
        y: 75,
        message: "修正拼写",
        tone: "text-green",
      },
      { id: "p", label: "P", x: 305, y: 205, message: "发布准备" },
      {
        id: "copy",
        label: "C′",
        x: 710,
        y: 205,
        message: "应用 C 的改动",
        tone: "text-green",
        from: 1,
      },
    ],
    edges: [
      { a: "a", b: "b" },
      { a: "b", b: "c" },
      { a: "a", b: "p" },
      { a: "p", b: "copy", from: 1 },
      { a: "c", b: "copy", from: 1, copy: true },
    ],
    captions: [
      "发布分支只需要修正拼写，暂时不想接入尚未完成的新功能。",
      "挑选 C 的改动，在发布分支产生 C′；B 的新功能不会一起带过来。",
    ],
  },
  remote: {
    lanes: [
      { label: "Remote 远程仓库（另一个 Git 仓库）", y: 30 },
      { label: "我的本地仓库", y: 160 },
    ],
    nodes: [
      { id: "ra", label: "A", x: 110, y: 75, message: "共同版本" },
      {
        id: "rb",
        label: "B",
        x: 310,
        y: 75,
        message: "补充配置",
        from: 1,
        enterY: 130,
      },
      {
        id: "rc",
        label: "C",
        x: 510,
        y: 75,
        message: "修复错误",
        from: 1,
        enterY: 130,
      },
      {
        id: "rd",
        label: "D",
        x: 710,
        y: 75,
        message: "同伴的新提交",
        tone: "text-peach",
        from: 2,
      },
      { id: "la", label: "A", x: 110, y: 205, message: "共同版本" },
      { id: "lb", label: "B", x: 310, y: 205, message: "补充配置" },
      { id: "lc", label: "C", x: 510, y: 205, message: "修复错误" },
      {
        id: "ld",
        label: "D",
        x: 710,
        y: 205,
        message: "同伴的新提交",
        tone: "text-peach",
        from: 3,
        enterY: -130,
      },
    ],
    edges: [
      { a: "ra", b: "rb", from: 1 },
      { a: "rb", b: "rc", from: 1 },
      { a: "rc", b: "rd", from: 2 },
      { a: "la", b: "lb" },
      { a: "lb", b: "lc" },
      { a: "lc", b: "ld", from: 3 },
    ],
    captions: [
      "把另一个 Git 仓库添加为 remote；此时 B、C 只存在于本地。",
      "发送 B、C，并更新对方相应的分支。两边都保有这些提交。",
      "同伴把新提交 D 发送到了这个 remote，本地还没有 D。",
      "获取 D，再整合到本地分支。本例没有分叉；分叉时可能需要合并。",
    ],
  },
};

const scene = computed(() => scenes[props.kind]);
const frame = computed(() =>
  Math.max(0, Math.min(props.step, scene.value.captions.length - 1)),
);
const tall = computed(() => Boolean(scene.value.lanes));
const activeX = computed(() => (frame.value === 1 ? 310 : 710));
const edges = computed(() =>
  scene.value.edges.map((edge) => {
    const a = scene.value.nodes.find((node) => node.id === edge.a)!;
    const b = scene.value.nodes.find((node) => node.id === edge.b)!;
    const middle = (a.x + b.x) / 2;
    return {
      ...edge,
      path: `M ${a.x + 21} ${a.y} C ${middle} ${a.y}, ${middle} ${b.y}, ${b.x - 21} ${b.y}`,
    };
  }),
);
</script>

<template>
  <figure class="git-history" :data-kind="kind" :data-step="frame">
    <svg
      :viewBox="`0 0 820 ${tall ? 270 : 225}`"
      role="img"
      :aria-label="scene.captions[frame]"
    >
      <g v-for="lane in scene.lanes" :key="lane.label">
        <rect
          class="lane"
          x="12"
          :y="lane.y - 22"
          width="796"
          height="122"
          rx="12"
        />
        <text class="lane-label" x="30" :y="lane.y">{{ lane.label }}</text>
      </g>
      <path
        v-for="edge in edges"
        :key="`${edge.a}-${edge.b}`"
        :d="edge.path"
        class="edge"
        :class="{ copy: edge.copy }"
        :style="{ opacity: frame >= (edge.from ?? 0) ? 1 : 0 }"
      />
      <g
        v-for="node in scene.nodes"
        :key="node.id"
        class="commit"
        :class="node.tone ?? 'text-blue'"
        :style="{
          opacity: frame >= (node.from ?? 0) ? 1 : 0,
          transform: `translate(0, ${frame >= (node.from ?? 0) ? 0 : (node.enterY ?? 12)}px)`,
        }"
      >
        <circle :cx="node.x" :cy="node.y" r="21" />
        <text class="commit-id" :x="node.x" :y="node.y + 6">
          {{ node.label }}
        </text>
        <text class="commit-message" :x="node.x" :y="node.y + 43">
          {{ node.message }}
        </text>
      </g>
      <g
        v-if="kind === 'history'"
        class="history-selection text-green"
        :style="{ transform: `translate(${activeX}px, 0)` }"
      >
        <circle cx="0" cy="105" r="29" />
        <text x="0" y="53">当前签出的提交</text>
      </g>
      <text
        v-if="kind === 'cherry-pick' && frame > 0"
        class="copy-label text-green"
        x="650"
        y="134"
      >
        只应用改动
      </text>
    </svg>
    <figcaption aria-live="polite">
      <span class="frame-count"
        >{{ frame + 1 }} / {{ scene.captions.length }}</span
      >
      <span>{{ scene.captions[frame] }}</span>
    </figcaption>
  </figure>
</template>

<style scoped>
.git-history {
  margin: 0.5rem 0 0;
}
svg {
  display: block;
  width: 100%;
  height: 14rem;
  overflow: visible;
}
.lane {
  fill: currentColor;
  fill-opacity: 0.035;
  stroke: currentColor;
  stroke-opacity: 0.18;
}
.lane-label {
  fill: currentColor;
  font-size: 15px;
  font-weight: 650;
}
.edge {
  fill: none;
  stroke: currentColor;
  stroke-width: 3;
  transition: opacity 400ms ease;
}
.edge.copy {
  stroke-dasharray: 7 6;
  stroke-width: 2;
}
.commit {
  transition:
    opacity 400ms ease,
    transform 450ms ease;
}
.commit circle {
  fill: var(--slidev-theme-background, #1e1e2e);
  stroke: currentColor;
  stroke-width: 3;
}
.commit-id {
  fill: currentColor;
  text-anchor: middle;
  font:
    700 17px "JetBrains Mono",
    monospace;
}
.commit-message {
  fill: currentColor;
  text-anchor: middle;
  font-size: 16px;
}
.history-selection {
  transition: transform 450ms ease;
}
.history-selection circle {
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
}
.history-selection text,
.copy-label {
  fill: currentColor;
  text-anchor: middle;
  font-size: 16px;
  font-weight: 650;
}
.git-history figcaption {
  display: flex;
  gap: 0.8rem;
  align-items: baseline;
  min-height: 3.2rem;
  border-top: 1px solid #8884;
  padding-top: 0.7rem;
  font-size: 0.95rem;
  line-height: 1.55;
}
.frame-count {
  flex: none;
  opacity: 0.55;
  font:
    0.75rem "JetBrains Mono",
    monospace;
}
@media (prefers-reduced-motion: reduce) {
  .commit,
  .edge,
  .history-selection {
    transition: none;
  }
}
</style>
