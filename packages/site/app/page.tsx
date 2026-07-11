import { readFileSync } from "node:fs";
import { join } from "node:path";
import Link from "next/link";
import { ArchitectureXRay } from "../components/architecture-xray";
import { FeatureGrid } from "../components/feature-grid";
import { Hero } from "../components/hero";
import { TerminalOutput } from "../components/terminal-output";
import { docsUrl } from "../lib/links";
import styles from "./page.module.css";

function readSample(name: string) {
  return readFileSync(join(process.cwd(), "content", "samples", name), "utf8");
}

export default function HomePage() {
  const output = readSample("vscode-summary.txt");

  return (
    <main className={styles.main}>
      <Hero />
      <ArchitectureXRay compact />
      <FeatureGrid />
      <section className={styles.showcaseBand}>
        <div className={styles.showcaseInner}>
          <div>
            <p className={styles.kicker}>Report shape</p>
            <h2 className={styles.title}>Readable locally, structured enough for CI.</h2>
            <p className={styles.copy}>
              The terminal reporter gives developers the graph size, timings, and grouped findings
              at a glance. JSON output uses the same report model for automation.
            </p>
            <div className={styles.links}>
              <Link className={styles.primary} href="/demo">
                View demo
              </Link>
              <a className={styles.secondary} href={docsUrl}>
                CLI reference
              </a>
            </div>
          </div>
          <TerminalOutput label="sample gruffed output" output={output} />
        </div>
      </section>
    </main>
  );
}
