export type GraphNodeId = string & { readonly __graphNodeId: unique symbol };
export type GraphEdgeId = string & { readonly __graphEdgeId: unique symbol };

export type GraphNodeKind = "module" | "package" | "project" | "unresolved";
export type GraphFindingKind = "cycle" | "unresolved" | "long-chain";

export type GraphViewNode = {
  id: GraphNodeId;
  label: string;
  path: string;
  x: number;
  y: number;
  nodeKind: GraphNodeKind;
  parentId?: GraphNodeId;
  childCount: number;
  depth: number;
  findingCount: number;
  sourceUrl?: string;
};

export type GraphViewEdge = {
  id: GraphEdgeId;
  source: GraphNodeId;
  target: GraphNodeId;
  findingIds: string[];
  importCount?: number;
};

export type GraphFinding = {
  id: string;
  ruleId: "no-cycles" | "no-unresolved" | "no-long-chains";
  kind: GraphFindingKind;
  severity: "error" | "warning";
  title: string;
  detail: string;
  nodePath: GraphNodeId[];
  edgePath: GraphEdgeId[];
  sourcePaths: string[];
};

/** A bounded, renderer-neutral projection of the authoritative Rust graph. */
export type GraphView = {
  id: string;
  semanticZoomLevel: "module" | "directory" | "package";
  totalNodeCount: number;
  totalEdgeCount: number;
  visibleNodes: GraphViewNode[];
  visibleEdges: GraphViewEdge[];
  findings: GraphFinding[];
};

export type DemoCase = {
  id: string;
  label: string;
  repository: {
    name: string;
    url: string;
    commit: string;
    scannedAt: string;
  };
  summary: string;
  command: string;
  terminalOutput: string;
  graph: GraphView;
};

export function nodeId(value: string): GraphNodeId {
  return value as GraphNodeId;
}

export function edgeId(value: string): GraphEdgeId {
  return value as GraphEdgeId;
}

export function validateDemoCases(cases: readonly DemoCase[]): void {
  for (const demo of cases) {
    const nodes = new Set(demo.graph.visibleNodes.map((node) => node.id));
    const edges = new Map(demo.graph.visibleEdges.map((edge) => [edge.id, edge]));

    if (nodes.size !== demo.graph.visibleNodes.length || edges.size !== demo.graph.visibleEdges.length) {
      throw new Error(`Duplicate graph identifiers in demo case ${demo.id}`);
    }

    for (const node of demo.graph.visibleNodes) {
      if (!Number.isFinite(node.x) || !Number.isFinite(node.y)) {
        throw new Error(`Invalid coordinates for ${node.id}`);
      }
      if (node.parentId && !nodes.has(node.parentId)) {
        throw new Error(`Missing parent ${node.parentId}`);
      }
    }

    for (const edge of demo.graph.visibleEdges) {
      if (!nodes.has(edge.source) || !nodes.has(edge.target)) {
        throw new Error(`Invalid endpoints for ${edge.id}`);
      }
    }

    for (const finding of demo.graph.findings) {
      if (finding.nodePath.some((id) => !nodes.has(id))) {
        throw new Error(`Finding ${finding.id} references a missing node`);
      }
      if (finding.edgePath.some((id) => !edges.has(id))) {
        throw new Error(`Finding ${finding.id} references a missing edge`);
      }
    }
  }
}
