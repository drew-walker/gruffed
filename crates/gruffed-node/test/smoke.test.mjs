import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import { analyzeGraph, buildModuleGraph, freeGraph, renderReport } from "@gruffed/node";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe("@gruffed/node", () => {
  test("builds and analyzes the cycle fixture", () => {
    const root = path.resolve(__dirname, "../../../fixtures/cycle");
    const config = JSON.stringify({
      rules: {
        "no-cycles": "error",
        "no-unresolved": "error",
      },
    });

    const result = buildModuleGraph(root, config);

    try {
      const report = analyzeGraph(result.graphHandle, result.warnings, config);
      expect(report.stats.nodeCount).toBe(3);
      expect(report.findings.some((finding) => finding.ruleId === "no-cycles")).toBe(true);

      const output = renderReport(report, result.graphHandle, false);
      expect(output).toMatch(/no-cycles/);
      expect(output).toMatch(/Circular dependency/);
    } finally {
      freeGraph(result.graphHandle);
    }
  });
});
