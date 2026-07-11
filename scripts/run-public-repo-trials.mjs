import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const repos = JSON.parse(fs.readFileSync(path.join(repoRoot, "scripts/public-repos.json"), "utf8"));
const reposRoot =
  process.env.GRUFFED_TRIAL_REPOS_ROOT ??
  path.join(process.env.HOME ?? "", "Projects/gruffed-repo-trials");
const gruffedBin =
  process.env.GRUFFED_BIN ?? path.join(repoRoot, "packages/gruffed/bin/gruffed.js");
const outDir = path.join(repoRoot, "docs/repo-trials");
const runId = new Date().toISOString().slice(0, 10);
const jsonPath = path.join(outDir, `${runId}-results.json`);
const markdownPath = path.join(outDir, `${runId}-results.md`);
const configPath = path.join(os.tmpdir(), `gruffed-public-repo-trial-${process.pid}.jsonc`);

const config = {
  exclude: [
    "node_modules/**",
    ".git/**",
    ".next/**",
    ".nuxt/**",
    "dist/**",
    "build/**",
    "coverage/**",
    "out/**",
    "tmp/**",
    "temp/**",
    "vendor/**",
    "fixtures/**",
    "__fixtures__/**",
    "**/*.test.ts",
    "**/*.test.tsx",
    "**/*.spec.ts",
    "**/*.spec.tsx",
    "**/*.d.ts",
  ],
  extensions: [".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"],
  rules: {
    "no-cycles": "error",
    "no-unresolved": "error",
  },
};

fs.mkdirSync(outDir, { recursive: true });
fs.writeFileSync(configPath, JSON.stringify(config, null, 2));

function runRepo(repo) {
  const repoPath = path.join(reposRoot, repo.name);
  const started = process.hrtime.bigint();
  const result = spawnSync(
    "node",
    [gruffedBin, "--root", repoPath, "--config", configPath, "--format", "json"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      maxBuffer: 128 * 1024 * 1024,
    },
  );
  const elapsedMs = Number(process.hrtime.bigint() - started) / 1_000_000;

  let parsed = null;
  let parseError = null;
  try {
    parsed = JSON.parse(result.stdout);
  } catch (error) {
    parseError = error.message;
  }

  const findings = parsed?.findings ?? [];
  const ruleCounts = {};
  for (const finding of findings) {
    ruleCounts[finding.ruleId] = (ruleCounts[finding.ruleId] ?? 0) + 1;
  }

  return {
    name: repo.name,
    url: repo.url,
    path: repoPath,
    exitCode: result.status,
    signal: result.signal,
    elapsedMs: Math.round(elapsedMs),
    stdoutBytes: Buffer.byteLength(result.stdout),
    stderr: result.stderr.trim(),
    parseError,
    stats: parsed?.stats ?? null,
    findingCount: findings.length,
    ruleCounts,
    sampleFindings: findings.slice(0, 5),
  };
}

const results = repos.map((repo) => {
  console.log(`trial ${repo.name}`);
  return runRepo(repo);
});

const payload = {
  generatedAt: new Date().toISOString(),
  reposRoot,
  gruffedBin,
  config,
  results,
};

fs.writeFileSync(jsonPath, `${JSON.stringify(payload, null, 2)}\n`);

const lines = [
  "# Public repo trial results",
  "",
  `Generated: ${payload.generatedAt}`,
  "",
  `CLI: \`${gruffedBin}\``,
  "",
  "| Repo | Exit | Time | Nodes | Edges | Findings | Top Rules | Notes |",
  "| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |",
];

for (const result of results) {
  const stats = result.stats ?? {};
  const topRules = Object.entries(result.ruleCounts)
    .sort((a, b) => b[1] - a[1])
    .map(([rule, count]) => `${rule}: ${count}`)
    .join("<br>");
  const notes = [
    result.parseError ? `JSON parse failed: ${result.parseError}` : "",
    result.stderr ? `stderr: ${result.stderr.replaceAll("\n", " ")}` : "",
  ]
    .filter(Boolean)
    .join("<br>");
  lines.push(
    `| ${result.name} | ${result.exitCode ?? ""} | ${result.elapsedMs}ms | ${stats.nodeCount ?? stats.node_count ?? ""} | ${stats.edgeCount ?? stats.edge_count ?? ""} | ${result.findingCount} | ${topRules} | ${notes} |`,
  );
}

lines.push("", "## Sample findings", "");

for (const result of results) {
  lines.push(`### ${result.name}`, "");
  if (result.sampleFindings.length === 0) {
    lines.push("No sample findings.", "");
    continue;
  }

  for (const finding of result.sampleFindings) {
    lines.push(`- \`${finding.ruleId}\` ${finding.file ?? ""}: ${truncate(finding.message, 320)}`);
  }
  lines.push("");
}

fs.writeFileSync(markdownPath, `${lines.join("\n")}\n`);

console.log(`wrote ${jsonPath}`);
console.log(`wrote ${markdownPath}`);

function truncate(value, maxLength) {
  if (!value || value.length <= maxLength) {
    return value ?? "";
  }

  return `${value.slice(0, maxLength - 1)}…`;
}
