#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { analyzeGraph, buildModuleGraph, freeGraph, renderReport } from "../src/index.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const packageJson = JSON.parse(fs.readFileSync(path.resolve(__dirname, "../package.json"), "utf8"));

function printHelp() {
  process.stdout.write(`gruffed ${packageJson.version}

Module graph analysis for JavaScript and TypeScript codebases.

Usage:
  gruffed [--root <dir>] [--config <file>] [--format terminal|json] [--color|--no-color]

Options:
  --root <dir>       Root directory to analyze. Defaults to the current directory.
  --config <file>    JSONC config file to pass to gruffed.
  --format <format>  Output format: terminal or json. Defaults to terminal.
  --color            Force color output.
  --no-color         Disable color output.
  --help             Show this help.
  --version          Show the version.
`);
}

function parseArgs(argv) {
  const options = {
    root: ".",
    config: undefined,
    format: "terminal",
    color: undefined,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];

    switch (arg) {
      case "--help":
      case "-h":
        options.help = true;
        break;
      case "--version":
      case "-V":
        options.version = true;
        break;
      case "--root":
        options.root = readValue(argv, index, arg);
        index += 1;
        break;
      case "--config":
        options.config = readValue(argv, index, arg);
        index += 1;
        break;
      case "--format":
        options.format = readValue(argv, index, arg);
        index += 1;
        break;
      case "--color":
        options.color = true;
        break;
      case "--no-color":
        options.color = false;
        break;
      default:
        throw new Error(`unknown argument: ${arg}`);
    }
  }

  if (options.format !== "terminal" && options.format !== "json") {
    throw new Error(`unsupported format: ${options.format}`);
  }

  return options;
}

function readValue(argv, index, arg) {
  const value = argv[index + 1];
  if (!value || value.startsWith("-")) {
    throw new Error(`${arg} requires a value`);
  }
  return value;
}

function readConfig(configPath) {
  if (!configPath) {
    return undefined;
  }

  return fs.readFileSync(path.resolve(configPath), "utf8");
}

function run() {
  let options;
  try {
    options = parseArgs(process.argv.slice(2));
  } catch (error) {
    process.stderr.write(`Error: ${error.message}\n\n`);
    printHelp();
    return 2;
  }

  if (options.help) {
    printHelp();
    return 0;
  }

  if (options.version) {
    process.stdout.write(`${packageJson.version}\n`);
    return 0;
  }

  const configJson = readConfig(options.config);
  const root = path.resolve(options.root);
  const useColor = options.color ?? process.stdout.isTTY;

  let result;
  try {
    result = buildModuleGraph(root, configJson);
    const report = analyzeGraph(result.graphHandle, result.warnings, configJson ?? '{"rules":{}}');

    if (options.format === "json") {
      process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    } else {
      process.stdout.write(renderReport(report, result.graphHandle, useColor));
    }

    return report.findings.some((finding) => finding.severity === "Error") ? 1 : 0;
  } catch (error) {
    process.stderr.write(`Error: ${error.message}\n`);
    return 1;
  } finally {
    if (result) {
      freeGraph(result.graphHandle);
    }
  }
}

process.exitCode = run();
