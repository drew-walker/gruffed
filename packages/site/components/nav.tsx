import { Braces } from "lucide-react";
import Link from "next/link";
import { docsUrl, githubUrl } from "../lib/links";
import styles from "./nav.module.css";

export function Nav() {
  return (
    <header className={styles.nav}>
      <div className={styles.inner}>
        <Link className={styles.brand} href="/">
          <span className={styles.mark} aria-hidden="true">
            <Braces size={17} strokeWidth={2.4} />
          </span>
          gruffed
        </Link>
        <nav className={styles.links} aria-label="Primary">
          <Link className={styles.link} href="/showcase">
            Showcase
          </Link>
          <Link className={styles.link} href="/public-repos">
            Public repos
          </Link>
          <Link className={styles.link} href="/benchmarks">
            Benchmarks
          </Link>
          <a className={styles.link} href={docsUrl}>
            Docs
          </a>
          <a className={styles.link} href={githubUrl}>
            GitHub
          </a>
        </nav>
      </div>
    </header>
  );
}
