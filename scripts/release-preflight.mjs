import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const workDir = fs.mkdtempSync(path.join(os.tmpdir(), "gruffed-release-preflight-"));
const packDir = path.join(workDir, "packs");
const npmInstallDir = path.join(workDir, "npm-install");
const npmCacheDir = path.join(workDir, "npm-cache");
const cargoInstallRoot = path.join(workDir, "cargo-install");

fs.mkdirSync(packDir, { recursive: true });
fs.mkdirSync(npmInstallDir, { recursive: true });
fs.mkdirSync(npmCacheDir, { recursive: true });

function run(command, args, options = {}) {
  const pretty = [command, ...args].join(" ");
  console.log(`\n$ ${pretty}`);
  const result = spawnSync(command, args, {
    cwd: options.cwd ?? repoRoot,
    stdio: "inherit",
    env: {
      ...process.env,
      ...options.env,
    },
  });

  if (result.status !== 0) {
    throw new Error(`command failed (${result.status}): ${pretty}`);
  }
}

function firstPacked(name) {
  const file = fs
    .readdirSync(packDir)
    .find((entry) => entry === name || entry.startsWith(`${name}-`));

  if (!file) {
    throw new Error(`could not find packed tarball for ${name} in ${packDir}`);
  }

  return path.join(packDir, file);
}

console.log(`Using temporary preflight directory: ${workDir}`);

run("cargo", ["build"]);
run("cargo", ["test"]);
run("pnpm", ["node:build"]);
run("pnpm", ["test"]);

run("cargo", ["package", "-p", "gruffed-core", "--allow-dirty", "--no-verify", "--offline"]);

run("pnpm", ["--filter", "@gruffed/node", "pack", "--pack-destination", packDir]);
run("pnpm", ["--filter", "gruffed", "pack", "--pack-destination", packDir]);

const nodeTarball = firstPacked("gruffed-node");
const gruffedTarball = firstPacked("gruffed");

fs.writeFileSync(
  path.join(npmInstallDir, "package.json"),
  JSON.stringify({ type: "module", private: true }, null, 2),
);

run("npm", ["install", nodeTarball, gruffedTarball], {
  cwd: npmInstallDir,
  env: {
    npm_config_cache: npmCacheDir,
  },
});
run(
  "node",
  [
    "node_modules/.bin/gruffed",
    "--root",
    path.join(repoRoot, "fixtures/simple"),
    "--config",
    path.join(repoRoot, "fixtures/simple/gruffed.jsonc"),
    "--format",
    "json",
  ],
  {
    cwd: npmInstallDir,
  },
);
run(
  "node",
  [
    "-e",
    "import('gruffed').then((mod) => { if (typeof mod.buildModuleGraph !== 'function') throw new Error('missing buildModuleGraph export'); })",
  ],
  {
    cwd: npmInstallDir,
  },
);

run("cargo", [
  "install",
  "--path",
  path.join(repoRoot, "crates/gruffed"),
  "--root",
  cargoInstallRoot,
  "--debug",
  "--force",
  "--locked",
  "--offline",
]);
run(path.join(cargoInstallRoot, "bin/gruffed"), [
  "--root",
  path.join(repoRoot, "fixtures/simple"),
  "--format",
  "json",
]);

console.log(`\nRelease preflight passed. Artifacts were written under ${workDir}`);
