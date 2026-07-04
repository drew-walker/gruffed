import { BenchmarkChart, type BenchmarkPoint } from "../../components/benchmark-chart";
import data from "../../content/benchmarks.json";
import styles from "./page.module.css";

const benchmarks = data satisfies BenchmarkPoint[];
const largest = benchmarks[benchmarks.length - 1];
const speedup = Math.round(largest.sequentialMs / largest.parallelMs);

export default function BenchmarksPage() {
  return (
    <main className={styles.page}>
      <div className={styles.inner}>
        <p className={styles.kicker}>Benchmarks</p>
        <h1 className={styles.title}>Static benchmark snapshots for graph build performance.</h1>
        <p className={styles.copy}>
          gruffed is designed around Rust-owned graph memory, parallel parsing, and adjacency
          indexes. These committed snapshots make release-to-release performance easy to compare.
        </p>
        <BenchmarkChart data={benchmarks} />
        <section className={styles.stats} aria-label="Benchmark highlights">
          <div className={styles.stat}>
            <span className={styles.statValue}>{largest.files.toLocaleString()}</span>
            <span className={styles.statLabel}>files in largest sample</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statValue}>{largest.parallelMs}ms</span>
            <span className={styles.statLabel}>parallel build snapshot</span>
          </div>
          <div className={styles.stat}>
            <span className={styles.statValue}>{speedup}x</span>
            <span className={styles.statLabel}>faster than sequential baseline</span>
          </div>
        </section>
        <p className={styles.note}>
          Benchmark data is static and sample-based until release automation publishes canonical
          numbers from `cargo bench`.
        </p>
      </div>
    </main>
  );
}
