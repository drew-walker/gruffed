export type DemoRepoScan = {
  name: string;
  repo: string;
  elapsedMs: number;
  nodes: number;
  edges: number;
};

export const demoRepoScanSummary = {
  generatedAt: "2026-07-05",
  repoCount: 12,
  totalNodes: 73850,
  totalEdges: 18027,
  totalElapsedMs: 6176,
};

export const demoRepoScans: DemoRepoScan[] = [
  {
    name: "Vite",
    repo: "vitejs/vite",
    elapsedMs: 310,
    nodes: 1285,
    edges: 1374,
  },
  {
    name: "Vitest",
    repo: "vitest-dev/vitest",
    elapsedMs: 122,
    nodes: 1251,
    edges: 1716,
  },
  {
    name: "pnpm",
    repo: "pnpm/pnpm",
    elapsedMs: 168,
    nodes: 1286,
    edges: 1257,
  },
  {
    name: "ESLint",
    repo: "eslint/eslint",
    elapsedMs: 111,
    nodes: 1497,
    edges: 1109,
  },
  {
    name: "Prettier",
    repo: "prettier/prettier",
    elapsedMs: 516,
    nodes: 5710,
    edges: 47,
  },
  {
    name: "Astro",
    repo: "withastro/astro",
    elapsedMs: 262,
    nodes: 2008,
    edges: 2878,
  },
  {
    name: "Remix",
    repo: "remix-run/remix",
    elapsedMs: 100,
    nodes: 1106,
    edges: 2026,
  },
  {
    name: "TypeScript",
    repo: "microsoft/TypeScript",
    elapsedMs: 2505,
    nodes: 38240,
    edges: 2075,
  },
  {
    name: "Babel",
    repo: "babel/babel",
    elapsedMs: 1620,
    nodes: 17547,
    edges: 1199,
  },
  {
    name: "Vue Core",
    repo: "vuejs/core",
    elapsedMs: 49,
    nodes: 318,
    edges: 989,
  },
  {
    name: "Svelte",
    repo: "sveltejs/svelte",
    elapsedMs: 368,
    nodes: 3402,
    edges: 2835,
  },
  {
    name: "Tailwind CSS",
    repo: "tailwindlabs/tailwindcss",
    elapsedMs: 45,
    nodes: 200,
    edges: 522,
  },
];
