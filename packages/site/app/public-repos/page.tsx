import {
  publicRepoResults,
  publicRepoSummary,
  representativeFindings,
} from "../../content/public-repo-results";
import styles from "./page.module.css";

const formatNumber = new Intl.NumberFormat("en-US");

function ms(value: number) {
  return `${formatNumber.format(value)}ms`;
}

export default function PublicReposPage() {
  const seconds = (publicRepoSummary.totalElapsedMs / 1000).toFixed(1);

  return (
    <main className={styles.page}>
      <div className={styles.inner}>
        <p className={styles.kicker}>Public repo trials</p>
        <h1 className={styles.title}>Alpha runs against real JavaScript and TypeScript repos.</h1>
        <p className={styles.copy}>
          These snapshots came from the installable CLI running with default rules on well-known
          open-source repositories. The goal is to show scale, output shape, and early signal while
          keeping the alpha caveat visible.
        </p>

        <section className={styles.stats} aria-label="Trial totals">
          <div className={styles.stat}>
            <span className={styles.statValue}>{publicRepoSummary.repoCount}</span>
            <span className={styles.statLabel}>repositories scanned</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statValue}>
              {formatNumber.format(publicRepoSummary.totalNodes)}
            </span>
            <span className={styles.statLabel}>source nodes discovered</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statValue}>
              {formatNumber.format(publicRepoSummary.totalEdges)}
            </span>
            <span className={styles.statLabel}>module edges connected</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statValue}>{seconds}s</span>
            <span className={styles.statLabel}>total CLI runtime</span>
          </div>
        </section>

        <section className={styles.insights} aria-label="Trial insights">
          <div className={styles.insight}>
            <h2>What looked useful</h2>
            <p>
              Cycle findings were compact enough to inspect in large projects, and the CLI finished
              every run without a crash or invalid JSON output.
            </p>
          </div>
          <div className={styles.insight}>
            <h2>What needs tuning</h2>
            <p>
              Unresolved-import counts are noisy in fixtures, generated snapshots, and repos that
              expect build artifacts. The next pass should add repo-specific excludes and path alias
              support.
            </p>
          </div>
        </section>

        <section className={styles.tableSection} aria-labelledby="results-heading">
          <div className={styles.sectionHeader}>
            <p className={styles.kicker}>Run summary</p>
            <h2 id="results-heading">Default-rule results</h2>
          </div>
          <div className={styles.tableWrap}>
            <table className={styles.table}>
              <thead>
                <tr>
                  <th scope="col">Repo</th>
                  <th scope="col">Time</th>
                  <th scope="col">Nodes</th>
                  <th scope="col">Edges</th>
                  <th scope="col">Findings</th>
                  <th scope="col">Rule split</th>
                  <th scope="col">Read</th>
                </tr>
              </thead>
              <tbody>
                {publicRepoResults.map((result) => (
                  <tr key={result.repo}>
                    <th scope="row">
                      <span className={styles.repoName}>{result.name}</span>
                      <span className={styles.repoSlug}>{result.repo}</span>
                    </th>
                    <td>{ms(result.elapsedMs)}</td>
                    <td>{formatNumber.format(result.nodes)}</td>
                    <td>{formatNumber.format(result.edges)}</td>
                    <td>{formatNumber.format(result.findings)}</td>
                    <td>
                      <span className={styles.ruleSplit}>
                        <span>{result.cycles} cycles</span>
                        <span>{result.unresolved} unresolved</span>
                      </span>
                    </td>
                    <td>{result.note}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <section className={styles.findingsSection} aria-labelledby="findings-heading">
          <div className={styles.sectionHeader}>
            <p className={styles.kicker}>Representative findings</p>
            <h2 id="findings-heading">Examples worth a second look</h2>
          </div>
          <div className={styles.findingGrid}>
            {representativeFindings.map((finding) => (
              <article className={styles.finding} key={`${finding.repo}-${finding.path}`}>
                <div className={styles.findingHeader}>
                  <span>{finding.repo}</span>
                  <code>{finding.rule}</code>
                </div>
                <p className={styles.findingPath}>{finding.path}</p>
                <p className={styles.findingDetail}>{finding.detail}</p>
              </article>
            ))}
          </div>
        </section>

        <p className={styles.note}>
          Generated from a local trial run on {publicRepoSummary.generatedAt}. Repositories were
          scanned as cloned, with default gruffed rules and no project-specific config.
        </p>
      </div>
    </main>
  );
}
