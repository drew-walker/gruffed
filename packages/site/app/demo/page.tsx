import { PublicRepoExplorer } from "../../components/public-repo-explorer";
import { demoRepoScans, demoRepoScanSummary } from "../../content/demo-repo-scans";
import styles from "./page.module.css";

const formatNumber = new Intl.NumberFormat("en-US");

export default function DemoPage() {
  return (
    <main className={styles.page}>
      <div className={styles.inner}>
        <p className={styles.kicker}>Demo</p>
        <h1 className={styles.title}>See how gruffed reports graph problems and scales up.</h1>
        <p className={styles.copy}>
          Explore the largest cycles and deepest dependency chains found in pinned Babel,
          TypeScript, and Svelte scans. Every displayed path comes from the complete Rust-owned
          module graph.
        </p>

        <PublicRepoExplorer />

        <section className={styles.scanSummary} aria-labelledby="scan-summary-heading">
          <div className={styles.summaryHeader}>
            <div>
              <p className={styles.kicker}>Broader scan summary</p>
              <h2 id="scan-summary-heading">Twelve repositories at a glance.</h2>
            </div>
            <p>The detailed explorer above uses newly pinned trace artifacts. This table preserves the wider release-readiness scan for quick comparison without opening every graph.</p>
          </div>
          <div className={styles.summaryStats} aria-label="Public scan totals">
            <span><strong>{demoRepoScanSummary.repoCount}</strong> repositories</span>
            <span><strong>{formatNumber.format(demoRepoScanSummary.totalNodes)}</strong> modules</span>
            <span><strong>{formatNumber.format(demoRepoScanSummary.totalEdges)}</strong> imports</span>
            <span><strong>{(demoRepoScanSummary.totalElapsedMs / 1000).toFixed(1)}s</strong> combined runtime</span>
          </div>
          <div className={styles.tableWrap}>
            <table className={styles.table}>
              <thead><tr><th scope="col">Repository</th><th scope="col">Runtime</th><th scope="col">Modules</th><th scope="col">Imports</th></tr></thead>
              <tbody>{demoRepoScans.map((result) => (
                <tr key={result.repo}>
                  <th scope="row"><div className={styles.repoCell}><img src={`https://github.com/${result.repo.split("/")[0]}.png?size=64`} width="30" height="30" loading="lazy" alt="" /><span><strong>{result.name}</strong><small>{result.repo}</small></span></div></th>
                  <td>{result.elapsedMs.toLocaleString()}ms</td>
                  <td>{formatNumber.format(result.nodes)}</td>
                  <td>{formatNumber.format(result.edges)}</td>
                </tr>
              ))}</tbody>
            </table>
          </div>
          <p className={styles.summaryNote}>Broader snapshot generated {demoRepoScanSummary.generatedAt}; retained as a coarse parser-coverage summary rather than a controlled benchmark.</p>
        </section>
      </div>
    </main>
  );
}
