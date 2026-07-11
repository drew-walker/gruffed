import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

import * as gruffed from "gruffed";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe("gruffed", () => {
  test("re-exports the native API and detects unresolved imports", () => {
    const root = path.resolve(__dirname, "../../../fixtures/unresolved");
    const config = JSON.stringify({
      rules: {
        "no-unresolved": "error",
      },
    });

    const result = gruffed.buildModuleGraph(root, config);

    try {
      const report = gruffed.analyzeGraph(result.graphHandle, result.warnings, config);
      expect(report.findings.some((finding) => finding.ruleId === "no-unresolved")).toBe(true);
    } finally {
      gruffed.freeGraph(result.graphHandle);
    }
  });
});
