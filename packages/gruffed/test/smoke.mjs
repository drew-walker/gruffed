import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import * as gruffed from 'gruffed';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, '../../../fixtures/unresolved');
const config = JSON.stringify({
  rules: {
    'no-unresolved': 'error',
  },
});

const result = gruffed.buildModuleGraph(root, config);

try {
  const report = gruffed.analyzeGraph(result.graphHandle, result.warnings, config);
  assert.ok(report.findings.some((finding) => finding.ruleId === 'no-unresolved'));
} finally {
  gruffed.freeGraph(result.graphHandle);
}
