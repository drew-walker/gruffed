"use client";

import { useEffect, useRef } from "react";
import type { GraphNodeId, GraphView } from "../lib/graph-view";
import { SigmaGraphRenderer } from "../lib/sigma-graph-renderer";
import styles from "./architecture-xray.module.css";

export default function SigmaGraphCanvas({
  view,
  selectedNodeIds,
  selectedEdgeIds,
  focusToken,
  onNodeSelect,
  onNodeHover,
  onContextLost,
}: Readonly<{
  view: GraphView;
  selectedNodeIds: readonly GraphNodeId[];
  selectedEdgeIds: readonly string[];
  focusToken: number;
  onNodeSelect: (nodeId: GraphNodeId | null) => void;
  onNodeHover: (nodeId: GraphNodeId | null) => void;
  onContextLost: () => void;
}>) {
  const containerRef = useRef<HTMLDivElement>(null);
  const rendererRef = useRef<SigmaGraphRenderer | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;
    const renderer = new SigmaGraphRenderer();
    rendererRef.current = renderer;
    renderer.mount(containerRef.current, view, {
      reducedMotion: window.matchMedia("(prefers-reduced-motion: reduce)").matches,
      onNodeSelect,
      onNodeHover,
      onContextLost,
    });
    renderer.setSelection(selectedNodeIds, selectedEdgeIds);
    renderer.fit();
    return () => renderer.destroy();
  }, [view, onNodeSelect, onNodeHover, onContextLost]);

  useEffect(() => {
    rendererRef.current?.setSelection(selectedNodeIds, selectedEdgeIds);
  }, [selectedNodeIds, selectedEdgeIds]);

  useEffect(() => {
    rendererRef.current?.fit(selectedNodeIds.length === 1 ? selectedNodeIds : []);
  }, [focusToken, selectedNodeIds]);

  return <div ref={containerRef} className={styles.canvas} aria-hidden="true" />;
}
