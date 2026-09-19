import { defineCounterConfig } from "slidev-addon-counter/config";

export default defineCounterConfig({
  counters: [
    {
      id: "default",
      defaultLevel: 2,
      levels: [
        {
          level: 1,
          format: "%{:value}.",
        },
        {
          level: 2,
          format: "%{@-1:full}%{:value}",
        },
      ],
    },
  ],
});
