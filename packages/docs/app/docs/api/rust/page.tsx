import styles from "./page.module.css";

const crates = [
  ["gruffed-core", "Graph data model, strongly typed IDs, report structures, and shared primitives."],
  ["gruffed-builder", "Source discovery, import parsing with oxc, resolution, and graph construction."],
  ["gruffed-analyzer", "Rules engine plus built-in graph rules for cycles, unresolved imports, and chain depth."],
  ["gruffed-config", "JSONC config parsing and rule setting normalization."],
  ["gruffed-console-reporter", "Terminal and JSON rendering for reports."],
  ["gruffed", "The command-line binary that wires config, builder, analyzer, and reporter together."],
] as const;

export default function RustApiPage() {
  return (
    <article className={styles.page}>
      <h1 className={styles.title}>Rust API</h1>
      <p className={styles.lead}>
        gruffed is a Rust workspace organized around build, analyze, and report phases.
        Pre-1.0 APIs are intentionally unstable while the tool settles through real usage.
      </p>

      <section className={styles.grid} aria-label="Rust crates">
        {crates.map(([name, description]) => (
          <div className={styles.crate} key={name}>
            <h2>{name}</h2>
            <p>{description}</p>
          </div>
        ))}
      </section>

      <h2 className={styles.sectionTitle}>Current flow</h2>
      <pre className={styles.code}>{`use gruffed_analyzer::Analyzer;
use gruffed_builder::ModuleGraphBuilder;
use gruffed_config::GruffedConfig;

let config = GruffedConfig::discover_or_default(std::path::Path::new("."))?;
let build = ModuleGraphBuilder::new("./src")
    .with_extensions(config.extensions.clone())
    .with_exclude(config.exclude.clone())
    .with_entrypoints(config.entrypoints.clone())
    .build()?;

let analyzer = Analyzer::from_config(&config.rules);
let report = analyzer.run(&build.graph, &build.warnings);`}</pre>

      <h2 className={styles.sectionTitle}>Rustdoc</h2>
      <p className={styles.lead}>
        Hosted rustdoc should be generated from the release branch with <code>cargo doc</code>.
        Until public docs hosting is configured, the crate list above is the stable map for readers.
      </p>
    </article>
  );
}
