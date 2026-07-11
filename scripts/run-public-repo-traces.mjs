import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const workspace = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const projectsRoot = process.env.GRUFFED_TRACE_PROJECTS_ROOT ?? path.join(process.env.HOME ?? '', 'Projects');
const binary = process.env.GRUFFED_BIN ?? path.join(workspace, 'target/release/gruffed');
const config = path.join(workspace, 'scripts/public-repo-trace.jsonc');
const outputDir = path.join(workspace, 'packages/site/content/traces');
const repositories = [
  { name: 'babel', directory: 'babel' },
  { name: 'typescript', directory: 'TypeScript' },
  { name: 'svelte', directory: 'svelte' },
];

fs.mkdirSync(outputDir, { recursive: true });

for (const repository of repositories) {
  const root = path.join(projectsRoot, repository.directory);
  for (const variant of [{ suffix: '', maxNodes: '80' }, { suffix: '-full', maxNodes: '2000' }]) {
    const result = spawnSync(binary, [
      '--root', root,
      '--config', config,
      '--format', 'trace',
      '--trace-max-nodes', variant.maxNodes,
    ], { cwd: workspace, encoding: 'utf8', maxBuffer: 128 * 1024 * 1024 });

    if (result.error || !result.stdout.trim()) {
      throw result.error ?? new Error(`Trace generation produced no output for ${repository.name}: ${result.stderr}`);
    }

    const trace = JSON.parse(result.stdout);
    if (trace.schemaVersion !== 1 || !Array.isArray(trace.nodes) || !Array.isArray(trace.edges)) {
      throw new Error(`Invalid trace artifact for ${repository.name}`);
    }

    const output = path.join(outputDir, `${repository.name}${variant.suffix}.json`);
    const temporary = `${output}.tmp`;
    fs.writeFileSync(temporary, `${JSON.stringify(trace, null, 2)}\n`);
    fs.renameSync(temporary, output);
    console.log(`${repository.name}${variant.suffix}: ${trace.stats.totalNodes} modules, ${trace.stats.totalEdges} imports, ${trace.stats.projectedNodes} visible nodes`);
  }
}
