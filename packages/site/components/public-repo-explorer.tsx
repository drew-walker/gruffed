"use client";

import { ExternalLink, FileCode2, GitFork, Route, RotateCcw, Timer } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import styles from "./public-repo-explorer.module.css";

type TraceEdge = { id: string; source: string; target: string; importCount: number };
type TraceData = {
  stats: { totalNodes: number; totalEdges: number; projectedNodes: number; projectedEdges: number; buildTimeMs: number; analyzeTimeMs: number; omittedClusters: number };
  nodes: Array<{ id: string; label: string; moduleCount: number; findingCount: number }>;
  edges: TraceEdge[];
  findings: Array<{ ruleId: string; severity: string; message: string; file: string; path: string[] }>;
};

const featured = [
  { name: "Babel", repo: "babel/babel", commit: "c86e9e4b" },
  { name: "TypeScript", repo: "microsoft/TypeScript", commit: "637d5746" },
  { name: "Svelte", repo: "sveltejs/svelte", commit: "b4d1583a" },
] as const;

type RepoName = (typeof featured)[number]["name"];

const traceLoaders: Record<RepoName, () => Promise<TraceData>> = {
  Babel: () => import("../content/traces/babel-full.json").then((module) => module.default as TraceData),
  TypeScript: () => import("../content/traces/typescript-full.json").then((module) => module.default as TraceData),
  Svelte: () => import("../content/traces/svelte-full.json").then((module) => module.default as TraceData),
};
const traceCache = new Map<RepoName, TraceData>();

const format = new Intl.NumberFormat("en-US");

export function PublicRepoExplorer() {
  const [repoName, setRepoName] = useState<RepoName>(featured[0].name);
  const [trace, setTrace] = useState<TraceData | null>(traceCache.get(featured[0].name) ?? null);
  const [pathKind, setPathKind] = useState<"cycles" | "chains">("cycles");
  const [selectedPath, setSelectedPath] = useState(0);
  const repo = featured.find((item) => item.name === repoName) ?? featured[0];
  const cycles = useMemo(() => trace ? cyclePaths(trace) : [], [trace]);
  const chains = useMemo(() => trace ? longestChains(trace).slice(0, 5) : [], [trace]);
  const paths = pathKind === "cycles" ? cycles : chains;
  const activePath = paths[Math.min(selectedPath, Math.max(0, paths.length - 1))];

  useEffect(() => {
    let cancelled = false;
    const cached = traceCache.get(repoName);
    setTrace(cached ?? null);
    if (!cached) {
      traceLoaders[repoName]().then((loaded) => {
        traceCache.set(repoName, loaded);
        if (!cancelled) setTrace(loaded);
      });
    }
    return () => { cancelled = true; };
  }, [repoName]);

  function selectRepo(name: RepoName) {
    setRepoName(name);
    setSelectedPath(0);
  }

  return (
    <section className={styles.section} aria-labelledby="public-repo-heading">
      <div className={styles.header}>
        <div>
          <p className={styles.kicker}>Public repository scans</p>
          <h2 id="public-repo-heading">See gruffed operate at real project scale.</h2>
        </div>
        <p>Ranked structural paths extracted from complete scans of large open-source TypeScript and JavaScript repositories, generated with the installable CLI and the same Rust graph builder.</p>
      </div>

      <div className={styles.tabs} role="tablist" aria-label="Featured public repositories">
        {featured.map((item) => (
          <button key={item.name} role="tab" aria-selected={item.name === repo.name} onClick={() => selectRepo(item.name)}>{item.name}</button>
        ))}
      </div>

      <div className={styles.panel}>
        <div className={styles.repoIdentity}>
          <span className={styles.repoMark}><GitFork size={23} /></span>
          <div>
            <span>github.com</span>
            <h3>{repo.repo}</h3>
          </div>
          <a href={`https://github.com/${repo.repo}/tree/${repo.commit}`} target="_blank" rel="noreferrer">View pinned source <ExternalLink size={14} /></a>
        </div>

        <div className={styles.metrics}>
          <article><FileCode2 size={18} /><span>Source modules</span><strong>{trace ? format.format(trace.stats.totalNodes) : "—"}</strong></article>
          <article><GitFork size={18} /><span>Import edges</span><strong>{trace ? format.format(trace.stats.totalEdges) : "—"}</strong></article>
          <article><Timer size={18} /><span>Graph build</span><strong>{trace ? `${trace.stats.buildTimeMs.toLocaleString()}ms` : "—"}</strong></article>
        </div>

        <div className={styles.pathExplorer} aria-busy={!trace}>
          {!trace ? <div className={styles.traceLoading}>Loading pinned structural paths…</div> : <>
          <div className={styles.pathHeader}>
            <div><span>Structural paths</span><strong>Show the problems, not the entire graph.</strong></div>
            <div className={styles.pathToggle} role="tablist" aria-label="Structural path type">
              <button type="button" role="tab" aria-selected={pathKind === "cycles"} onClick={() => { setPathKind("cycles"); setSelectedPath(0); }}><RotateCcw size={14} /> Cycles <b>{cycles.length}</b></button>
              <button type="button" role="tab" aria-selected={pathKind === "chains"} onClick={() => { setPathKind("chains"); setSelectedPath(0); }}><Route size={14} /> Longest chains <b>{chains.length}</b></button>
            </div>
          </div>
          <div className={styles.pathBody}>
            <ol className={styles.pathList} aria-label={pathKind === "cycles" ? "Largest circular dependencies" : "Longest dependency chains"}>
              {paths.map((path, index) => <li key={`${path.kind}-${path.nodes.join("-")}`}><button type="button" aria-pressed={index === selectedPath} onClick={() => setSelectedPath(index)}><span>{String(index + 1).padStart(2, "0")}</span><strong>{path.kind === "cycle" ? `${path.nodes.length}-module cycle` : `${path.nodes.length - 1}-hop chain`}</strong><small>{shortPath(path.nodes[0])} → {shortPath(path.nodes.at(-1) ?? "")}</small></button></li>)}
            </ol>
            <div className={styles.pathDetail}>
              {activePath ? <>
                <div className={styles.pathDetailHeader}><span>{activePath.kind === "cycle" ? "Circular dependency" : "Dependency depth"}</span><strong>{activePath.nodes.length} modules</strong><small>{activePath.kind === "cycle" ? "Following the final import returns to the first module." : "A deepest acyclic route through the repository graph."}</small></div>
                <ol className={styles.pathSteps}>{activePath.nodes.map((node, index) => <li key={`${node}-${index}`}><span>{index + 1}</span><div><strong>{shortPath(node)}</strong><code>{node}</code></div></li>)}</ol>
                {activePath.kind === "cycle" && <div className={styles.returnEdge}><RotateCcw size={14} /> returns to {shortPath(activePath.nodes[0])}</div>}
              </> : <p className={styles.emptyPath}>No {pathKind === "cycles" ? "cycles" : "dependency chains"} were found in this scan.</p>}
            </div>
          </div>
          </>}
        </div>

        <p className={styles.note}>Pinned at commit {repo.commit}. Cycles are confirmed analyzer findings. Longest chains are derived deterministically from the complete graph after collapsing strongly connected components.</p>
      </div>
    </section>
  );
}

type StructuralPath = { kind: "cycle" | "chain"; nodes: string[] };

function cyclePaths(trace: TraceData): StructuralPath[] {
  return trace.findings.filter((finding) => finding.ruleId === "no-cycles").sort((a, b) => b.path.length - a.path.length || a.file.localeCompare(b.file)).map((finding) => ({ kind: "cycle", nodes: finding.path }));
}

function longestChains(trace: TraceData): StructuralPath[] {
  const ids = trace.nodes.map((node) => node.id);
  const label = new Map(trace.nodes.map((node) => [node.id, node.label]));
  const outgoing = new Map(ids.map((id) => [id, [] as string[]]));
  for (const edge of trace.edges) outgoing.get(edge.source)?.push(edge.target);
  const components = stronglyConnectedComponents(ids, outgoing);
  const componentOf = new Map<string, number>();
  components.forEach((members, component) => members.forEach((id) => componentOf.set(id, component)));
  const dag = new Map(components.map((_, component) => [component, new Set<number>()]));
  for (const edge of trace.edges) { const source = componentOf.get(edge.source); const target = componentOf.get(edge.target); if (source !== undefined && target !== undefined && source !== target) dag.get(source)?.add(target); }
  const memo = new Map<number, number[]>();
  const bestFrom = (component: number): number[] => {
    const known = memo.get(component); if (known) return known;
    let best = [component];
    for (const next of dag.get(component) ?? []) { const candidate = [component, ...bestFrom(next)]; if (candidate.length > best.length) best = candidate; }
    memo.set(component, best); return best;
  };
  const candidates = components.map((_, component) => bestFrom(component)).sort((a, b) => b.length - a.length);
  const unique = new Set<string>();
  const result: StructuralPath[] = [];
  for (const path of candidates) {
    const nodeIds = materializePath(path, components, componentOf, trace.edges, outgoing);
    const nodes = nodeIds.map((id) => label.get(id) ?? id);
    const signature = nodes.slice(0, 3).join("|");
    if (nodes.length < 2 || unique.has(signature)) continue;
    unique.add(signature); result.push({ kind: "chain", nodes });
    if (result.length === 5) break;
  }
  return result;
}

function materializePath(componentPath: readonly number[], components: readonly string[][], componentOf: Map<string, number>, edges: TraceEdge[], outgoing: Map<string, string[]>): string[] {
  if (componentPath.length === 0) return [];
  if (componentPath.length === 1) return [[...components[componentPath[0]]].sort()[0]];
  const result: string[] = [];
  for (let index = 0; index < componentPath.length - 1; index++) {
    const sourceComponent = componentPath[index]; const targetComponent = componentPath[index + 1];
    const bridge = edges.find((edge) => componentOf.get(edge.source) === sourceComponent && componentOf.get(edge.target) === targetComponent);
    if (!bridge) continue;
    if (result.length === 0) result.push(bridge.source);
    const current = result.at(-1);
    if (current && current !== bridge.source) result.push(...pathWithinComponent(current, bridge.source, sourceComponent, componentOf, outgoing).slice(1));
    if (result.at(-1) !== bridge.target) result.push(bridge.target);
  }
  return result;
}

function pathWithinComponent(start: string, goal: string, component: number, componentOf: Map<string, number>, outgoing: Map<string, string[]>): string[] {
  if (start === goal) return [start];
  const queue = [start]; const previous = new Map<string, string | null>([[start, null]]);
  for (let cursor = 0; cursor < queue.length; cursor++) {
    const current = queue[cursor];
    for (const next of outgoing.get(current) ?? []) {
      if (componentOf.get(next) !== component || previous.has(next)) continue;
      previous.set(next, current); queue.push(next);
      if (next === goal) { const path = [goal]; let step: string | null = current; while (step) { path.push(step); step = previous.get(step) ?? null; } return path.reverse(); }
    }
  }
  return [start, goal];
}

function stronglyConnectedComponents(ids: readonly string[], outgoing: Map<string, string[]>): string[][] {
  let nextIndex = 0;
  const index = new Map<string, number>(); const low = new Map<string, number>(); const stack: string[] = []; const onStack = new Set<string>(); const components: string[][] = [];
  const visit = (id: string) => { index.set(id, nextIndex); low.set(id, nextIndex++); stack.push(id); onStack.add(id); for (const next of outgoing.get(id) ?? []) { if (!index.has(next)) { visit(next); low.set(id, Math.min(low.get(id) ?? 0, low.get(next) ?? 0)); } else if (onStack.has(next)) low.set(id, Math.min(low.get(id) ?? 0, index.get(next) ?? 0)); } if (low.get(id) === index.get(id)) { const component: string[] = []; let member = ""; do { member = stack.pop() ?? ""; onStack.delete(member); if (member) component.push(member); } while (member !== id); components.push(component); } };
  for (const id of ids) if (!index.has(id)) visit(id);
  return components;
}

function shortPath(path: string) { return path.split("/").at(-1) ?? path; }
