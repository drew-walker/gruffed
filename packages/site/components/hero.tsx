import Image from "next/image";
import { docsUrl } from "../lib/links";
import { InstallCommand } from "./install-command";
import styles from "./hero.module.css";

export function Hero() {
  return (
    <section className={styles.hero}>
      <Image
        className={styles.image}
        src="/images/gruffed-hero.png"
        alt="Module dependency graph with highlighted circular dependency, long chain, and unresolved import diagnostics"
        fill
        priority
        sizes="100vw"
      />
      <div className={styles.wash} />
      <div className={styles.inner}>
        <div className={styles.content}>
          <p className={styles.eyebrow}>Rust speed for TypeScript structure</p>
          <h1 className={styles.title}>gruffed</h1>
          <p className={styles.copy}>
            Fast module-graph analysis for JavaScript and TypeScript codebases.
            Find cycles, unresolved imports, and dependency chains before they harden.
          </p>
          <div className={styles.actions}>
            <InstallCommand command="cargo install gruffed-cli" />
            <a className={styles.cta} href={docsUrl}>
              Read the docs
            </a>
          </div>
          <div className={styles.stats} aria-label="Highlights">
            <span className={styles.stat}>Tarjan SCC cycle detection</span>
            <span className={styles.stat}>Opaque Rust graph handles</span>
            <span className={styles.stat}>Terminal + JSON reports</span>
          </div>
        </div>
      </div>
    </section>
  );
}
