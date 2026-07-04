import Link from "next/link";
import styles from "./not-found.module.css";

export default function NotFound() {
  return (
    <main className={styles.page}>
      <section className={styles.panel}>
        <h1 className={styles.title}>Page not found</h1>
        <p className={styles.copy}>
          The docs page you asked for is not part of the current gruffed reference.
        </p>
        <Link className={styles.link} href="/docs/cli">
          Open CLI docs
        </Link>
      </section>
    </main>
  );
}
