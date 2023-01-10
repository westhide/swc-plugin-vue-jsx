import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: ["test/**"],
    exclude: ["test/**/__snapshots__"],
    benchmark: {
      include: ["bench/**"],
    },
    environment: "node",
    globals: true,
    transformMode: {
      web: [/\.tsx$/],
    },
  },
});
