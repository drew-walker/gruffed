import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    coverage: {
      provider: "v8",
      reporter: ["text", "html"],
      include: ["index.js", "lib/**/*.js", "src/**/*.js"],
      exclude: [
        "**/*.d.ts",
        "**/*.node",
        "bin/**",
        "test/**",
        "types/**",
        "npm/**",
        "node_modules/**",
        "coverage/**",
        ".next/**",
        "out/**",
      ],
      thresholds: {
        branches: 90,
        functions: 90,
        lines: 90,
        statements: 90,
      },
    },
  },
});
