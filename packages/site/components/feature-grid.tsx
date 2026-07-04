import { AlertTriangle, GitPullRequestArrow, Route } from "lucide-react";
import styles from "./feature-grid.module.css";

const features = [
  {
    icon: GitPullRequestArrow,
    title: "Circular imports",
    copy: "Find strongly connected components in the module graph and surface cycles as actionable findings.",
  },
  {
    icon: AlertTriangle,
    title: "Unresolved imports",
    copy: "Turn builder resolution warnings into rule findings with source file, specifier, and line number.",
  },
  {
    icon: Route,
    title: "Long chains",
    copy: "Measure import depth from configured entrypoints and flag paths that exceed your architecture budget.",
  },
] as const;

export function FeatureGrid() {
  return (
    <section className={styles.section}>
      <div className={styles.inner}>
        <div className={styles.header}>
          <h2 className={styles.title}>Graph linting for structural problems, not style debates.</h2>
          <p className={styles.copy}>
            gruffed keeps the analyzer focused: build the graph, inspect it immutably, report findings for humans.
          </p>
        </div>
        <div className={styles.grid}>
          {features.map((feature) => {
            const Icon = feature.icon;
            return (
              <article className={styles.card} key={feature.title}>
                <span className={styles.icon} aria-hidden="true">
                  <Icon size={20} />
                </span>
                <h3>{feature.title}</h3>
                <p>{feature.copy}</p>
              </article>
            );
          })}
        </div>
      </div>
    </section>
  );
}
