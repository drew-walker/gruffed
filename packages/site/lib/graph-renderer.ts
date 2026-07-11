import type { GraphNodeId, GraphView } from "./graph-view";

export type GraphRendererOptions = {
  reducedMotion: boolean;
  onNodeSelect: (nodeId: GraphNodeId | null) => void;
  onNodeHover: (nodeId: GraphNodeId | null) => void;
  onContextLost: () => void;
};

export interface GraphRenderer {
  mount(container: HTMLElement, view: GraphView, options: GraphRendererOptions): void;
  setView(view: GraphView): void;
  setSelection(nodeIds: readonly GraphNodeId[], edgeIds: readonly string[]): void;
  fit(nodeIds?: readonly GraphNodeId[]): void;
  destroy(): void;
}
