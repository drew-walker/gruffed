import { MultiDirectedGraph } from "graphology";
import Sigma from "sigma";
import type { Attributes } from "graphology-types";
import type { GraphRenderer, GraphRendererOptions } from "./graph-renderer";
import type { GraphNodeId, GraphView } from "./graph-view";

const colors = {
  node: "#405064",
  selected: "#08736f",
  dimmed: "#cbd5df",
  unresolved: "#c43d4d",
  edge: "#91a0b2",
  warning: "#d18b24",
};

export class SigmaGraphRenderer implements GraphRenderer {
  private graph = new MultiDirectedGraph();
  private sigma: Sigma | null = null;
  private container: HTMLElement | null = null;
  private options: GraphRendererOptions | null = null;
  private selectedNodes = new Set<string>();
  private selectedEdges = new Set<string>();
  private contextLossHandler: ((event: Event) => void) | null = null;

  mount(container: HTMLElement, view: GraphView, options: GraphRendererOptions): void {
    this.destroy();
    this.container = container;
    this.options = options;
    this.loadView(view);

    this.sigma = new Sigma(this.graph, container, {
      allowInvalidContainer: false,
      renderEdgeLabels: false,
      labelDensity: 0.8,
      labelGridCellSize: 110,
      labelRenderedSizeThreshold: 7,
      labelColor: { color: "#c8d3df" },
      minCameraRatio: 0.08,
      maxCameraRatio: 5,
      defaultNodeColor: colors.node,
      defaultEdgeColor: colors.edge,
      nodeReducer: (node, data) => this.reduceNode(node, data),
      edgeReducer: (edge, data) => this.reduceEdge(edge, data),
    });

    this.sigma.on("clickNode", ({ node }) => options.onNodeSelect(node as GraphNodeId));
    this.sigma.on("clickStage", () => options.onNodeSelect(null));
    this.sigma.on("enterNode", ({ node }) => options.onNodeHover(node as GraphNodeId));
    this.sigma.on("leaveNode", () => options.onNodeHover(null));

    this.contextLossHandler = (event) => {
      event.preventDefault();
      options.onContextLost();
    };
    for (const canvas of container.querySelectorAll("canvas")) {
      canvas.addEventListener("webglcontextlost", this.contextLossHandler);
    }
  }

  setView(view: GraphView): void {
    if (!this.container || !this.options) return;
    const container = this.container;
    const options = this.options;
    this.mount(container, view, options);
  }

  setSelection(nodeIds: readonly GraphNodeId[], edgeIds: readonly string[]): void {
    this.selectedNodes = new Set(nodeIds);
    this.selectedEdges = new Set(edgeIds);
    this.sigma?.refresh();
  }

  fit(nodeIds: readonly GraphNodeId[] = []): void {
    if (!this.sigma) return;
    if (nodeIds.length === 1) {
      const position = this.sigma.getNodeDisplayData(nodeIds[0]);
      if (position) {
        this.sigma
          .getCamera()
          .animate(
            { x: position.x, y: position.y, ratio: 0.45 },
            { duration: this.options?.reducedMotion ? 0 : 320 },
          );
        return;
      }
    }
    this.sigma.getCamera().animatedReset({ duration: this.options?.reducedMotion ? 0 : 320 });
  }

  destroy(): void {
    if (this.container && this.contextLossHandler) {
      for (const canvas of this.container.querySelectorAll("canvas")) {
        canvas.removeEventListener("webglcontextlost", this.contextLossHandler);
      }
    }
    this.sigma?.kill();
    this.sigma = null;
    this.container = null;
    this.options = null;
    this.contextLossHandler = null;
    this.selectedNodes.clear();
    this.selectedEdges.clear();
    this.graph = new MultiDirectedGraph();
  }

  private loadView(view: GraphView): void {
    this.graph.clear();
    for (const node of view.visibleNodes) {
      this.graph.addNode(node.id, {
        x: node.x,
        y: node.y,
        size:
          node.nodeKind === "unresolved"
            ? 11
            : Math.min(18, 5 + Math.sqrt(Math.max(1, node.childCount)) * 1.15),
        label: node.label,
        color:
          node.nodeKind === "unresolved"
            ? colors.unresolved
            : node.findingCount > 0
              ? colors.warning
              : colors.node,
        nodeKind: node.nodeKind,
      });
    }
    for (const edge of view.visibleEdges) {
      this.graph.addDirectedEdgeWithKey(edge.id, edge.source, edge.target, {
        size: Math.min(5, 0.7 + Math.log2(1 + (edge.importCount ?? 1)) * 0.55),
        color: colors.edge,
        type: "arrow",
      });
    }
  }

  private reduceNode(node: string, data: Attributes): Attributes {
    if (this.selectedNodes.size === 0) return data;
    const selected = this.selectedNodes.has(node);
    return {
      ...data,
      color: selected ? colors.selected : colors.dimmed,
      highlighted: selected,
      zIndex: selected ? 2 : 0,
      label: selected ? data.label : null,
    };
  }

  private reduceEdge(edge: string, data: Attributes): Attributes {
    if (this.selectedEdges.size === 0) return data;
    const selected = this.selectedEdges.has(edge);
    return {
      ...data,
      color: selected ? colors.warning : "#dce3ea",
      size: selected ? 3.5 : 1,
      hidden: false,
      zIndex: selected ? 2 : 0,
    };
  }
}
