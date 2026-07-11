"use client";

import { track } from "@vercel/analytics";
import { AlertTriangle, ExternalLink, Focus, GitBranch, RotateCcw, Route } from "lucide-react";
import dynamic from "next/dynamic";
import { useCallback, useMemo, useState } from "react";
import { architectureDemoCases } from "../content/architecture-demo";
import type { GraphNodeId } from "../lib/graph-view";
import styles from "./architecture-xray.module.css";

const SigmaGraphCanvas = dynamic(() => import("./sigma-graph-canvas"), {
  ssr: false,
  loading: () => <div className={styles.canvasLoading}>Preparing WebGL graph…</div>,
});

const icons = {
  cycle: GitBranch,
  unresolved: AlertTriangle,
  "long-chain": Route,
} as const;

export function ArchitectureXRay({ compact = false }: Readonly<{ compact?: boolean }>) {
  const [caseId, setCaseId] = useState(architectureDemoCases[0].id);
  const activeCase =
    architectureDemoCases.find((demo) => demo.id === caseId) ?? architectureDemoCases[0];
  const [findingId, setFindingId] = useState(activeCase.graph.findings[0].id);
  const [selectedNode, setSelectedNode] = useState<GraphNodeId | null>(null);
  const [hoveredNode, setHoveredNode] = useState<GraphNodeId | null>(null);
  const [focusToken, setFocusToken] = useState(0);
  const [contextLost, setContextLost] = useState(false);

  const finding =
    activeCase.graph.findings.find((item) => item.id === findingId) ?? activeCase.graph.findings[0];
  const selectedNodeIds = selectedNode ? [selectedNode] : finding.nodePath;
  const selectedEdgeIds = selectedNode ? [] : finding.edgePath;
  const inspectedNode = activeCase.graph.visibleNodes.find(
    (node) => node.id === (hoveredNode ?? selectedNode),
  );

  const selectCase = useCallback((id: string) => {
    const next = architectureDemoCases.find((demo) => demo.id === id) ?? architectureDemoCases[0];
    setCaseId(next.id);
    setFindingId(next.graph.findings[0].id);
    setSelectedNode(null);
    setHoveredNode(null);
    setContextLost(false);
    setFocusToken((value) => value + 1);
    track("architecture_demo_case_selected", { case: next.id });
  }, []);

  const selectNode = useCallback(
    (id: GraphNodeId | null) => {
      setSelectedNode(id);
      setFocusToken((value) => value + 1);
      if (id) track("architecture_demo_node_selected", { case: caseId });
    },
    [caseId],
  );
  const handleContextLost = useCallback(() => setContextLost(true), []);

  const sourceNodes = useMemo(
    () => activeCase.graph.visibleNodes.filter((node) => finding.nodePath.includes(node.id)),
    [activeCase, finding],
  );
  const FindingIcon = icons[finding.kind];

  return (
    <section
      className={`${styles.section} ${compact ? styles.compact : ""}`}
      aria-labelledby={`xray-title-${compact ? "compact" : "full"}`}
    >
      <div className={styles.inner}>
        <header className={styles.header}>
          <div>
            <p className={styles.kicker}>Architecture X-Ray</p>
            <h2 id={`xray-title-${compact ? "compact" : "full"}`}>
              Trace the problem, not a graph hairball.
            </h2>
          </div>
          <p>
            Explore verified findings from gruffed’s fixture repository. The WebGL view is a bounded
            projection of the Rust-owned graph—the same boundary designed to grow into tiled,
            million-node navigation.
          </p>
        </header>

        <div className={styles.caseTabs} role="tablist" aria-label="Graph finding examples">
          {architectureDemoCases.map((demo) => (
            <button
              key={demo.id}
              role="tab"
              aria-selected={demo.id === activeCase.id}
              className={demo.id === activeCase.id ? styles.activeTab : styles.caseTab}
              onClick={() => selectCase(demo.id)}
            >
              {demo.label}
            </button>
          ))}
        </div>

        <div className={styles.explorer}>
          <div className={styles.graphPanel}>
            <div className={styles.graphBar}>
              <div>
                <span className={styles.liveDot} aria-hidden="true" /> WebGL graph
                <span className={styles.graphStats}>
                  {activeCase.graph.visibleNodes.length} visible / {activeCase.graph.totalNodeCount}{" "}
                  total nodes
                </span>
              </div>
              <div className={styles.graphActions}>
                <button
                  type="button"
                  onClick={() => {
                    setSelectedNode(null);
                    setFocusToken((value) => value + 1);
                  }}
                  aria-label="Fit graph to viewport"
                >
                  <Focus size={16} />
                </button>
                <button
                  type="button"
                  onClick={() => selectCase(activeCase.id)}
                  aria-label="Reset graph selection"
                >
                  <RotateCcw size={16} />
                </button>
              </div>
            </div>
            <div className={styles.canvasWrap}>
              <SigmaGraphCanvas
                key={activeCase.id}
                view={activeCase.graph}
                selectedNodeIds={selectedNodeIds}
                selectedEdgeIds={selectedEdgeIds}
                focusToken={focusToken}
                onNodeSelect={selectNode}
                onNodeHover={setHoveredNode}
                onContextLost={handleContextLost}
              />
              {inspectedNode && (
                <div className={styles.nodeTooltip}>
                  <strong>{inspectedNode.label}</strong>
                  <span>{inspectedNode.path}</span>
                </div>
              )}
              {contextLost && (
                <div className={styles.contextLost} role="status">
                  WebGL context lost. Reload or switch cases to restore the graph.
                </div>
              )}
            </div>
          </div>

          <aside className={styles.detailPanel} aria-label="Selected finding">
            <div className={styles.findingHeading}>
              <span
                className={finding.severity === "error" ? styles.errorIcon : styles.warningIcon}
              >
                <FindingIcon size={19} />
              </span>
              <div>
                <span>{finding.ruleId}</span>
                <h3>{finding.title}</h3>
              </div>
            </div>
            <p className={styles.findingDetail}>{finding.detail}</p>
            <p className={styles.repoMeta}>
              <a
                href={`${activeCase.repository.url}/tree/${activeCase.repository.commit}`}
                target="_blank"
                rel="noreferrer"
                onClick={() => track("architecture_demo_source_clicked", { case: activeCase.id })}
              >
                {activeCase.repository.name} <ExternalLink size={13} />
              </a>
              <span>
                commit {activeCase.repository.commit} · scanned {activeCase.repository.scannedAt}
              </span>
            </p>

            <div className={styles.treeHeader}>
              <GitBranch size={14} />
              <span>Repository</span>
              <code>{activeCase.repository.name.split(" / ")[0]}</code>
            </div>
            <ol className={styles.pathList} aria-label="Finding dependency path">
              {sourceNodes.map((node) => (
                <li key={node.id}>
                  <button
                    type="button"
                    aria-pressed={selectedNode === node.id}
                    onClick={() => selectNode(selectedNode === node.id ? null : node.id)}
                  >
                    <span className={styles.treeLine} aria-hidden="true" />
                    <span className={styles.fileIcon} aria-hidden="true">
                      TS
                    </span>
                    <span>
                      <strong>{node.label}</strong>
                      <small>
                        {node.path.replace(`${node.path.split("/src/")[0]}/src/`, "src/")}
                      </small>
                    </span>
                  </button>
                  {node.sourceUrl && (
                    <a
                      href={node.sourceUrl}
                      target="_blank"
                      rel="noreferrer"
                      aria-label={`Open ${node.path} on GitHub`}
                    >
                      <ExternalLink size={14} />
                    </a>
                  )}
                </li>
              ))}
            </ol>
          </aside>
        </div>

        <div className={styles.cliPanel} aria-label={`${finding.ruleId} command-line output`}>
          <div className={styles.cliChrome}>
            <i />
            <i />
            <i />
            <span>gruffed — analysis</span>
          </div>
          <div className={styles.cliBody}>
            <p>
              <span className={styles.cliPrompt}>$</span> gruffed analyze{" "}
              <span className={styles.cliMuted}>fixtures/{activeCase.id}</span>
            </p>
            <p className={styles.cliMuted}>
              scanned <b>{activeCase.graph.totalNodeCount}</b> modules and{" "}
              <b>{activeCase.graph.totalEdgeCount}</b> imports in <b>0ms</b>
            </p>
            <p>
              <span className={finding.severity === "error" ? styles.cliError : styles.cliWarn}>
                {finding.severity === "error" ? "✖" : "▲"} {finding.severity}
              </span>{" "}
              <span className={styles.cliRule}>{finding.ruleId}</span>
            </p>
            <p className={styles.cliMessage}>{finding.title}</p>
            <p className={styles.cliPath}>{finding.sourcePaths.join("  →  ")}</p>
            <p className={styles.cliSummary}>
              {finding.severity === "error" ? "1 error" : "1 warning"}{" "}
              <span>· process finished with status {finding.severity === "error" ? "1" : "0"}</span>
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
