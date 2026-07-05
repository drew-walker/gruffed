import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  analyzeGraph,
  buildModuleGraph,
  freeGraph,
  renderReport,
} from '@gruffed/node';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, '../../../fixtures/cycle');
const config = JSON.stringify({
  rules: {
    'no-cycles': 'error',
    'no-unresolved': 'error',
  },
});

const result = buildModuleGraph(root, config);

try {
  const report = analyzeGraph(result.graphHandle, result.warnings, config);
  assert.equal(report.stats.nodeCount, 3);
  assert.ok(report.findings.some((finding) => finding.ruleId === 'no-cycles'));

  const output = renderReport(report, result.graphHandle, false);
  assert.match(output, /no-cycles/);
  assert.match(output, /Circular dependency/);
} finally {
  freeGraph(result.graphHandle);
}
