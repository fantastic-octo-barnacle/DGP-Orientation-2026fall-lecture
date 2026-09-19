<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{ os: "windows" | "linux" | "macos" }>();
interface TreeRow {
  name: string;
  depth: number;
  muted?: boolean;
  last?: boolean;
  home?: boolean;
  personal?: boolean;
  file?: boolean;
  paired?: boolean;
  ancestors?: boolean[];
}

const rows = computed(() => {
  const windows = props.os === "windows";
  const separator = windows ? "\\" : "/";
  const system = windows
    ? ["Windows", "Program Files"]
    : ["System", "Applications"];
  const systemChildren = windows
    ? ["System32", "Common Files"]
    : ["Library", "Utilities"];
  const files = windows
    ? ["todo.txt", "setup.exe", "report.docx"]
    : ["photo.png", "guide.pdf", "draft.pages"];
  const entries: TreeRow[] =
    props.os === "linux"
      ? [
          { name: "", depth: 0 },
          { name: "etc", depth: 1, muted: true },
          { name: "hosts", depth: 2, last: true, muted: true, file: true },
          { name: "usr", depth: 1, muted: true },
          { name: "bin", depth: 2, last: true },
          { name: "bash", depth: 3, file: true, paired: true },
          { name: "git", depth: 3, file: true, paired: true },
          { name: "python3", depth: 3, last: true, file: true },
          { name: "home", depth: 1, last: true },
          { name: "Alice", depth: 2, home: true, personal: true },
          { name: ".bashrc", depth: 3, file: true, paired: true },
          { name: ".bash_history", depth: 3, file: true, paired: true },
          { name: ".gitconfig", depth: 3, file: true, paired: true },
          { name: "my_projects", depth: 3, last: true, personal: true },
          { name: "hello.py", depth: 4, last: true, file: true },
          { name: "Bob", depth: 2, last: true, home: true, personal: true },
        ]
      : [
          { name: windows ? "C:" : "", depth: 0 },
          ...system.flatMap((name, index): TreeRow[] => [
            { name, depth: 1, muted: true },
            {
              name: systemChildren[index],
              depth: 2,
              last: windows || index === 0,
              muted: true,
            },
            ...(!windows && index === 1
              ? [
                  { name: "Firefox.app", depth: 2, muted: true },
                  { name: "VLC.app", depth: 2, last: true, muted: true },
                ]
              : []),
          ]),
          { name: "Users", depth: 1, last: true },
          { name: "Alice", depth: 2, home: true, personal: true },
          { name: "Desktop", depth: 3, personal: true },
          { name: files[0], depth: 4, last: true, file: true },
          { name: "Downloads", depth: 3, personal: true },
          { name: files[1], depth: 4, last: true, file: true },
          { name: "Documents", depth: 3, last: true, personal: true },
          { name: files[2], depth: 4, last: true, file: true },
          { name: "Bob", depth: 2, last: true, home: true, personal: true },
        ];
  const continuing: boolean[] = [];
  return entries.map((row) => {
    const ancestors = continuing.slice(0, row.depth);
    continuing[row.depth] = !row.last;
    return { ...row, ancestors, label: row.name + (row.file ? "" : separator) };
  });
});
</script>

<template>
  <div class="filesystem-tree">
    <div v-for="row in rows" :key="row.name" class="filesystem-tree-row">
      <span
        v-for="level in row.depth"
        :key="level"
        class="filesystem-tree-indent"
        :class="{
          branch: level === row.depth,
          last: row.last,
          continuation: level < row.depth && row.ancestors[level],
        }"
        aria-hidden="true"
      />
      <span
        :class="{
          muted: row.muted,
          'text-green': row.personal,
          'text-peach': row.file && !row.paired,
          'text-mauve': row.paired,
          'filesystem-home': row.home,
        }"
        >{{ row.label }}</span
      >
    </div>
  </div>
</template>

<style scoped>
.filesystem-tree {
  font-family: "JetBrains Mono", "SFMono-Regular", Consolas, monospace;
  font-size: 0.85rem;
  line-height: 1rem;
}

.filesystem-tree-row {
  display: flex;
  height: 1rem;
  white-space: nowrap;
}

.filesystem-tree-indent {
  position: relative;
  flex: 0 0 2.4em;
}

.branch::before,
.branch::after,
.continuation::before {
  position: absolute;
  left: 0.35em;
  content: "";
  opacity: 0.66;
}

.branch::before,
.continuation::before {
  top: 0;
  bottom: 0;
  border-left: 1px solid currentColor;
}

.branch.last::before {
  bottom: 50%;
}

.branch::after {
  top: 50%;
  width: 1.5em;
  border-top: 1px solid currentColor;
}
</style>
