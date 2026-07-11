import Link from "next/link";
import { githubUrl } from "../lib/links";
import styles from "./footer.module.css";

export function Footer() {
  return (
    <footer className={styles.footer}>
      <div className={styles.inner}>
        <span>gruffed v0.1.0 - module graph analysis for JS and TS codebases.</span>
        <div className={styles.links}>
          <Link className={styles.link} href="/demo">
            Demo
          </Link>
          <a className={styles.link} href={githubUrl}>
            GitHub
          </a>
        </div>
      </div>
    </footer>
  );
}
