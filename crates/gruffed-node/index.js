import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

function candidateTriples() {
  const { arch, platform } = process;

  if (platform === 'darwin') {
    return arch === 'arm64' ? ['darwin-arm64'] : ['darwin-x64'];
  }

  if (platform === 'win32') {
    if (arch === 'arm64') return ['win32-arm64-msvc'];
    if (arch === 'ia32') return ['win32-ia32-msvc'];
    return ['win32-x64-msvc'];
  }

  if (platform === 'linux') {
    if (arch === 'arm64') return ['linux-arm64-gnu', 'linux-arm64-musl'];
    if (arch === 'arm') return ['linux-arm-gnueabihf', 'linux-arm-musleabihf'];
    return ['linux-x64-gnu', 'linux-x64-musl'];
  }

  return [`${platform}-${arch}`];
}

function loadNativeBinding() {
  const triples = candidateTriples();
  const localCandidates = [
    ...triples.map((triple) => `gruffed-node.${triple}.node`),
    'gruffed-node.node',
  ];

  for (const fileName of localCandidates) {
    const filePath = path.join(__dirname, fileName);
    if (fs.existsSync(filePath)) {
      return require(filePath);
    }
  }

  const packageCandidates = triples.map((triple) => `@gruffed/node-${triple}`);
  for (const packageName of packageCandidates) {
    try {
      return require(packageName);
    } catch (error) {
      if (!isMissingOptionalPackage(error, packageName)) {
        throw error;
      }
    }
  }

  throw new Error(
    `Could not find a native gruffed binding for ${process.platform}/${process.arch}. ` +
      `Looked for local files: ${localCandidates.join(', ')}; ` +
      `optional packages: ${packageCandidates.join(', ')}`,
  );
}

function isMissingOptionalPackage(error, packageName) {
  return (
    error?.code === 'MODULE_NOT_FOUND' && error.message.includes(packageName)
  );
}

const native = loadNativeBinding();

export const analyzeGraph = native.analyzeGraph;
export const buildModuleGraph = native.buildModuleGraph;
export const freeGraph = native.freeGraph;
export const renderReport = native.renderReport;
