import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ShowcaseTabs, type ShowcaseSample } from "../../components/showcase-tabs";
import { demoRepoScans, demoRepoScanSummary } from "../../content/demo-repo-scans";
import styles from "./page.module.css";

const formatNumber = new Intl.NumberFormat("en-US");

function readSample(name: string) {
  return readFileSync(join(process.cwd(), "content", "samples", name), "utf8");
}

function ms(value: number) {
  return `${formatNumber.format(value)}ms`;
}

const samples: ShowcaseSample[] = [
  {
    id: "cycles",
    label: "Cycles",
    output: readSample("vscode-cycles.txt"),
  },
  {
    id: "unresolved",
    label: "Unresolved imports",
    output: readSample("vscode-unresolved.txt"),
  },
  {
    id: "summary",
    label: "Mixed summary",
    output: readSample("vscode-summary.txt"),
  },
];

export default function DemoPage() {
  const seconds = (demoRepoScanSummary.totalElapsedMs / 1000).toFixed(1);

  return (
    <main className={styles.page}>
      <div className={styles.inner}>
        <p className={styles.kicker}>Demo</p>
        <h1 className={styles.title}>See how gruffed reports graph problems and scales up.</h1>
        <p className={styles.copy}>
          Sample reports show the CLI output developers work with locally. The real-repo scan
          snapshot shows the size of projects gruffed can parse quickly, while leaving raw findings
          for maintainers to review with their own project context.
        </p>

        <section className={styles.samples} aria-labelledby="samples-heading">
          <div className={styles.sectionHeader}>
            <p className={styles.kicker}>Report samples</p>
            <h2 id="samples-heading">Findings in the shape developers act on.</h2>
            <p>
              Output stays compact: graph stats first, findings grouped by rule, and enough path
              context to decide where to start.
            </p>
          </div>
          <ShowcaseTabs samples={samples} />
        </section>

        <section className={styles.scale} aria-labelledby="scale-heading">
          <div className={styles.sectionHeader}>
            <p className={styles.kicker}>Scale check</p>
            <h2 id="scale-heading">A quick pass over real open-source repositories.</h2>
            <p>
              These runs used the installable CLI with default rules. We publish size and runtime
              numbers here; raw findings belong in maintainers' own repos with project-specific
              configuration.
            </p>
          </div>

          <div className={styles.stats} aria-label="Trial totals">
            <div className={styles.stat}>
              <span className={styles.statValue}>{demoRepoScanSummary.repoCount}</span>
              <span className={styles.statLabel}>repositories scanned</span>
            </div>
            <div className={styles.stat}>
              <span className={styles.statValue}>
                {formatNumber.format(demoRepoScanSummary.totalNodes)}
              </span>
              <span className={styles.statLabel}>source nodes discovered</span>
            </div>
            <div className={styles.stat}>
              <span className={styles.statValue}>
                {formatNumber.format(demoRepoScanSummary.totalEdges)}
              </span>
              <span className={styles.statLabel}>module edges connected</span>
            </div>
            <div className={styles.stat}>
              <span className={styles.statValue}>{seconds}s</span>
              <span className={styles.statLabel}>total CLI runtime</span>
            </div>
          </div>

          <div className={styles.tableWrap}>
            <table className={styles.table}>
              <thead>
                <tr>
                  <th scope="col">Repo</th>
                  <th scope="col">Time</th>
                  <th scope="col">Nodes</th>
                  <th scope="col">Edges</th>
                </tr>
              </thead>
              <tbody>
                {demoRepoScans.map((result) => (
                  <tr key={result.repo}>
                    <th scope="row">
                      <span className={styles.repoName}>{result.name}</span>
                      <span className={styles.repoSlug}>{result.repo}</span>
                    </th>
                    <td>{ms(result.elapsedMs)}</td>
                    <td>{formatNumber.format(result.nodes)}</td>
                    <td>{formatNumber.format(result.edges)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <p className={styles.note}>
            Snapshot generated on {demoRepoScanSummary.generatedAt}. Results are not a benchmark
            suite; they are a coarse smoke test for parser coverage and release readiness.
          </p>
        </section>
      </div>
    </main>
  );
}
