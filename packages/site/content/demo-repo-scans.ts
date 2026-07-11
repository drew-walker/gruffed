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
  totalEdges: 23322,
  totalElapsedMs: 8191,
};

export const demoRepoScans: DemoRepoScan[] = [
  { name: "Vite", repo: "vitejs/vite", elapsedMs: 165, nodes: 1285, edges: 1488 },
  { name: "Vitest", repo: "vitest-dev/vitest", elapsedMs: 179, nodes: 1251, edges: 1754 },
  { name: "pnpm", repo: "pnpm/pnpm", elapsedMs: 285, nodes: 1286, edges: 1265 },
  { name: "ESLint", repo: "eslint/eslint", elapsedMs: 178, nodes: 1497, edges: 1134 },
  { name: "Prettier", repo: "prettier/prettier", elapsedMs: 600, nodes: 5710, edges: 2005 },
  { name: "Astro", repo: "withastro/astro", elapsedMs: 637, nodes: 2008, edges: 2912 },
  { name: "Remix", repo: "remix-run/remix", elapsedMs: 157, nodes: 1106, edges: 2026 },
  { name: "TypeScript", repo: "microsoft/TypeScript", elapsedMs: 3199, nodes: 38240, edges: 2114 },
  { name: "Babel", repo: "babel/babel", elapsedMs: 2131, nodes: 17547, edges: 1434 },
  { name: "Vue Core", repo: "vuejs/core", elapsedMs: 53, nodes: 318, edges: 1001 },
  { name: "Svelte", repo: "sveltejs/svelte", elapsedMs: 560, nodes: 3402, edges: 5667 },
  { name: "Tailwind CSS", repo: "tailwindlabs/tailwindcss", elapsedMs: 47, nodes: 200, edges: 522 },
];
