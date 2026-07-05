import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
const repos = JSON.parse(fs.readFileSync(path.join(repoRoot, 'scripts/public-repos.json'), 'utf8'));
const destinationRoot = process.argv[2] ?? path.join(process.env.HOME ?? '', 'Projects/gruffed-repo-trials');

if (!destinationRoot) {
  throw new Error('could not determine destination root');
}

fs.mkdirSync(destinationRoot, { recursive: true });

for (const repo of repos) {
  const destination = path.join(destinationRoot, repo.name);
  if (fs.existsSync(path.join(destination, '.git'))) {
    console.log(`skip ${repo.name}: already cloned at ${destination}`);
    continue;
  }

  console.log(`clone ${repo.name}: ${repo.url}`);
  const result = spawnSync('git', ['clone', '--depth', '1', repo.url, destination], {
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    throw new Error(`failed to clone ${repo.name}`);
  }
}

console.log(`trial repos are ready under ${destinationRoot}`);
