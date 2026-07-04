import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ShowcaseTabs, type ShowcaseSample } from "../../components/showcase-tabs";
import styles from "./page.module.css";

function readSample(name: string) {
  return readFileSync(join(process.cwd(), "content", "samples", name), "utf8");
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

export default function ShowcasePage() {
  return (
    <main className={styles.page}>
      <div className={styles.inner}>
        <p className={styles.kicker}>Showcase</p>
        <h1 className={styles.title}>Findings that look like the problems developers actually fix.</h1>
        <p className={styles.copy}>
          The reporter keeps output compact: graph stats first, findings grouped by rule, and
          enough context to decide where to start.
        </p>
        <ShowcaseTabs samples={samples} />
      </div>
    </main>
  );
}
