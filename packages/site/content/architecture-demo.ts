import {
  edgeId,
  nodeId,
  type DemoCase,
  type GraphViewNode,
  validateDemoCases,
} from "../lib/graph-view";

const commit = "6da7f87";
const repoUrl = "https://github.com/drew-walker/gruffed";

function moduleNode(id: string, path: string, x: number, y: number, depth: number): GraphViewNode {
  return {
    id: nodeId(id),
    label: path.split("/").at(-1) ?? path,
    path,
    x,
    y,
    nodeKind: "module",
    childCount: 0,
    depth,
    findingCount: 1,
    sourceUrl: `${repoUrl}/blob/${commit}/${path}`,
  };
}

const cycleNodes = [
  moduleNode("cycle:a", "fixtures/cycle/src/a.ts", 0, 0, 0),
  moduleNode("cycle:b", "fixtures/cycle/src/b.ts", 1, -0.72, 1),
  moduleNode("cycle:c", "fixtures/cycle/src/c.ts", 1, 0.72, 2),
];

const longPaths = ["index", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];
const longNodes = longPaths.map((name, index) =>
  moduleNode(
    `chain:${name}`,
    `fixtures/long-chain/src/${name}.ts`,
    index,
    index % 2 ? 0.24 : -0.24,
    index,
  ),
);

export const architectureDemoCases: readonly DemoCase[] = [
  {
    id: "cycle",
    label: "Circular dependency",
    repository: { name: "gruffed / cycle fixture", url: repoUrl, commit, scannedAt: "2026-07-10" },
    summary:
      "Three modules form a strongly connected component, so none can be initialized independently.",
    command: "gruffed --root fixtures/cycle",
    terminalOutput: `gruffed v0.1.0 — module graph analysis\n\n  Nodes:      3    Edges:      3    Build: 0ms    Analyze: 0ms\n\n  ✖ 1 error\n\n  error  no-cycles\n         Circular dependency detected (3 nodes in cycle): a.ts → b.ts → c.ts → a.ts\n`,
    graph: {
      id: "fixture-cycle",
      semanticZoomLevel: "module",
      totalNodeCount: 3,
      totalEdgeCount: 3,
      visibleNodes: cycleNodes,
      visibleEdges: [
        {
          id: edgeId("cycle:a-b"),
          source: nodeId("cycle:a"),
          target: nodeId("cycle:b"),
          findingIds: ["cycle-1"],
        },
        {
          id: edgeId("cycle:b-c"),
          source: nodeId("cycle:b"),
          target: nodeId("cycle:c"),
          findingIds: ["cycle-1"],
        },
        {
          id: edgeId("cycle:c-a"),
          source: nodeId("cycle:c"),
          target: nodeId("cycle:a"),
          findingIds: ["cycle-1"],
        },
      ],
      findings: [
        {
          id: "cycle-1",
          ruleId: "no-cycles",
          kind: "cycle",
          severity: "error",
          title: "A three-module cycle",
          detail: "Imports return to a.ts through b.ts and c.ts.",
          nodePath: cycleNodes.map((node) => node.id),
          edgePath: [edgeId("cycle:a-b"), edgeId("cycle:b-c"), edgeId("cycle:c-a")],
          sourcePaths: cycleNodes.map((node) => node.path),
        },
      ],
    },
  },
  {
    id: "unresolved",
    label: "Unresolved import",
    repository: {
      name: "gruffed / unresolved fixture",
      url: repoUrl,
      commit,
      scannedAt: "2026-07-10",
    },
    summary:
      "An import leaves the known module graph, identifying the source location and missing specifier.",
    command: "gruffed --root fixtures/unresolved",
    terminalOutput: `gruffed v0.1.0 — module graph analysis\n\n  Nodes:      1    Edges:      0    Build: 0ms    Analyze: 0ms\n\n  ✖ 1 error\n\n  error  no-unresolved\n         Cannot resolve import "./missing" in src/index.ts:1\n`,
    graph: {
      id: "fixture-unresolved",
      semanticZoomLevel: "module",
      totalNodeCount: 1,
      totalEdgeCount: 0,
      visibleNodes: [
        moduleNode("unresolved:index", "fixtures/unresolved/src/index.ts", -0.7, 0, 0),
        {
          id: nodeId("unresolved:missing"),
          label: "./missing",
          path: "./missing",
          x: 0.8,
          y: 0,
          nodeKind: "unresolved",
          childCount: 0,
          depth: 1,
          findingCount: 1,
        },
      ],
      visibleEdges: [
        {
          id: edgeId("unresolved:missing-edge"),
          source: nodeId("unresolved:index"),
          target: nodeId("unresolved:missing"),
          findingIds: ["unresolved-1"],
        },
      ],
      findings: [
        {
          id: "unresolved-1",
          ruleId: "no-unresolved",
          kind: "unresolved",
          severity: "error",
          title: "Import target not found",
          detail: "index.ts imports a module the resolver cannot locate.",
          nodePath: [nodeId("unresolved:index"), nodeId("unresolved:missing")],
          edgePath: [edgeId("unresolved:missing-edge")],
          sourcePaths: ["fixtures/unresolved/src/index.ts:1", "./missing"],
        },
      ],
    },
  },
  {
    id: "long-chain",
    label: "Long dependency chain",
    repository: {
      name: "gruffed / long-chain fixture",
      url: repoUrl,
      commit,
      scannedAt: "2026-07-10",
    },
    summary:
      "A deep import path makes change impact harder to predict and entrypoint startup more fragile.",
    command: "gruffed --root fixtures/long-chain",
    terminalOutput: `gruffed v0.1.0 — module graph analysis\n\n  Nodes:     12    Edges:     11    Build: 0ms    Analyze: 0ms\n\n  ⚠ 1 warning\n\n  warning  no-long-chains\n           Import chain depth 11 exceeds configured maximum 10\n`,
    graph: {
      id: "fixture-long-chain",
      semanticZoomLevel: "module",
      totalNodeCount: 12,
      totalEdgeCount: 11,
      visibleNodes: longNodes,
      visibleEdges: longNodes.slice(0, -1).map((node, index) => ({
        id: edgeId(`chain:${longPaths[index]}-${longPaths[index + 1]}`),
        source: node.id,
        target: longNodes[index + 1].id,
        findingIds: ["chain-1"],
      })),
      findings: [
        {
          id: "chain-1",
          ruleId: "no-long-chains",
          kind: "long-chain",
          severity: "warning",
          title: "Eleven imports from the entrypoint",
          detail: "The configured architecture budget is exceeded at k.ts.",
          nodePath: longNodes.map((node) => node.id),
          edgePath: longNodes
            .slice(0, -1)
            .map((_, index) => edgeId(`chain:${longPaths[index]}-${longPaths[index + 1]}`)),
          sourcePaths: longNodes.map((node) => node.path),
        },
      ],
    },
  },
] as const;

validateDemoCases(architectureDemoCases);
