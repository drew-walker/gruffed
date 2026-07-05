export type PublicRepoResult = {
  name: string;
  repo: string;
  elapsedMs: number;
  nodes: number;
  edges: number;
  findings: number;
  cycles: number;
  unresolved: number;
  note: string;
};

export type RepresentativeFinding = {
  repo: string;
  rule: "no-cycles" | "no-unresolved";
  path: string;
  detail: string;
};

export const publicRepoSummary = {
  generatedAt: "2026-07-05",
  repoCount: 12,
  totalNodes: 73850,
  totalEdges: 18027,
  totalFindings: 6707,
  totalElapsedMs: 6176,
  totalCycles: 105,
  totalUnresolved: 6602,
};

export const publicRepoResults: PublicRepoResult[] = [
  {
    name: "Vite",
    repo: "vitejs/vite",
    elapsedMs: 310,
    nodes: 1285,
    edges: 1374,
    findings: 135,
    cycles: 17,
    unresolved: 118,
    note: "Cycles surfaced in playground fixtures and module-runner internals.",
  },
  {
    name: "Vitest",
    repo: "vitest-dev/vitest",
    elapsedMs: 122,
    nodes: 1251,
    edges: 1716,
    findings: 54,
    cycles: 15,
    unresolved: 39,
    note: "Most unresolved imports came from test and coverage fixtures.",
  },
  {
    name: "pnpm",
    repo: "pnpm/pnpm",
    elapsedMs: 168,
    nodes: 1286,
    edges: 1257,
    findings: 48,
    cycles: 18,
    unresolved: 30,
    note: "Detected fixture-level unresolved imports and several package cycles.",
  },
  {
    name: "ESLint",
    repo: "eslint/eslint",
    elapsedMs: 111,
    nodes: 1497,
    edges: 1109,
    findings: 23,
    cycles: 0,
    unresolved: 23,
    note: "Findings were unresolved imports in tools and test paths.",
  },
  {
    name: "Prettier",
    repo: "prettier/prettier",
    elapsedMs: 516,
    nodes: 5710,
    edges: 47,
    findings: 2015,
    cycles: 2,
    unresolved: 2013,
    note: "Many unresolved imports point at generated or fixture-only files.",
  },
  {
    name: "Astro",
    repo: "withastro/astro",
    elapsedMs: 262,
    nodes: 2008,
    edges: 2878,
    findings: 176,
    cycles: 14,
    unresolved: 162,
    note: "Large cycles appeared in runtime, asset, and build-plugin paths.",
  },
  {
    name: "Remix",
    repo: "remix-run/remix",
    elapsedMs: 100,
    nodes: 1106,
    edges: 2026,
    findings: 12,
    cycles: 12,
    unresolved: 0,
    note: "Default run produced cycle findings without unresolved noise.",
  },
  {
    name: "TypeScript",
    repo: "microsoft/TypeScript",
    elapsedMs: 2505,
    nodes: 38240,
    edges: 2075,
    findings: 2073,
    cycles: 11,
    unresolved: 2062,
    note: "The largest trial; unresolved imports mostly came from compiler test cases.",
  },
  {
    name: "Babel",
    repo: "babel/babel",
    elapsedMs: 1620,
    nodes: 17547,
    edges: 1199,
    findings: 669,
    cycles: 7,
    unresolved: 662,
    note: "Benchmarks and generated-package paths dominated unresolved findings.",
  },
  {
    name: "Vue Core",
    repo: "vuejs/core",
    elapsedMs: 49,
    nodes: 318,
    edges: 989,
    findings: 37,
    cycles: 8,
    unresolved: 29,
    note: "Compiler and reactivity packages produced representative cycles.",
  },
  {
    name: "Svelte",
    repo: "sveltejs/svelte",
    elapsedMs: 368,
    nodes: 3402,
    edges: 2835,
    findings: 1463,
    cycles: 0,
    unresolved: 1463,
    note: "Snapshot and playground output paths created the bulk of unresolved findings.",
  },
  {
    name: "Tailwind CSS",
    repo: "tailwindlabs/tailwindcss",
    elapsedMs: 45,
    nodes: 200,
    edges: 522,
    findings: 2,
    cycles: 1,
    unresolved: 1,
    note: "A compact run with one large source-map cycle and one upgrade import finding.",
  },
];

export const representativeFindings: RepresentativeFinding[] = [
  {
    repo: "Vite",
    rule: "no-cycles",
    path: "packages/vite/src/module-runner/createImportMeta.ts",
    detail:
      "An 8-node cycle moved through the module runner evaluator, HMR handler, sourcemap interceptor, and runner types.",
  },
  {
    repo: "Astro",
    rule: "no-cycles",
    path: "packages/astro/src/runtime/server/transition.ts",
    detail:
      "A 175-node cycle connected server transition rendering, manifest serialization, routing, and build-plugin paths.",
  },
  {
    repo: "Remix",
    rule: "no-cycles",
    path: "packages/ui/src/runtime/core/vnode.ts",
    detail:
      "A small runtime cycle linked vnode, component, and JSX modules.",
  },
  {
    repo: "Vue Core",
    rule: "no-cycles",
    path: "packages/reactivity/src/effectScope.ts",
    detail:
      "The reactivity package exposed a 9-node cycle through effect scope, deps, refs, computed state, and watchers.",
  },
  {
    repo: "Tailwind CSS",
    rule: "no-cycles",
    path: "packages/tailwindcss/src/source-maps/source-map.ts",
    detail:
      "A 31-node source-map cycle moved through compatibility, variants, compile, and AST utilities.",
  },
  {
    repo: "TypeScript",
    rule: "no-unresolved",
    path: "tests/cases/transpile/declarationEmitPartialNodeReuse.ts",
    detail:
      "Several compiler test cases import deliberately incomplete fixture paths, which shows why alpha runs need repo-specific excludes.",
  },
];
