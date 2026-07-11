use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;

use gruffed_analyzer::Analyzer;
use gruffed_builder::ModuleGraphBuilder;
use gruffed_config::GruffedConfig;
use gruffed_console_reporter::{render_json, render_trace_json, ConsoleReporter};

#[derive(Parser)]
#[command(
    name = "gruffed",
    version,
    about = "Module graph analysis for large codebases"
)]
struct Cli {
    /// Path to config file (default: auto-discover gruffed.jsonc)
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output format: "terminal" (default), "json", or "trace"
    #[arg(long, default_value = "terminal")]
    format: String,

    /// Maximum number of clusters included in trace output
    #[arg(long, default_value_t = 80)]
    trace_max_nodes: usize,

    /// Force color output
    #[arg(long)]
    color: bool,

    /// Disable color output
    #[arg(long)]
    no_color: bool,

    /// Root directory to analyze (overrides config)
    #[arg(long)]
    root: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> ExitCode {
    let config = if let Some(config_path) = &cli.config {
        GruffedConfig::from_file(config_path)
    } else {
        let search_dir = cli
            .root
            .as_ref()
            .map(|p| p.as_path())
            .unwrap_or(std::path::Path::new("."));
        GruffedConfig::discover_or_default(search_dir)
    };

    let config = match config {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let root = cli
        .root
        .clone()
        .or_else(|| config.root.clone())
        .unwrap_or_else(|| PathBuf::from("."));

    let root = match root.canonicalize() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error resolving root path {:?}: {}", root, e);
            return ExitCode::FAILURE;
        }
    };

    let use_color = if cli.no_color {
        false
    } else if cli.color {
        true
    } else {
        std::io::stdout().is_terminal()
    };

    let build_start = Instant::now();
    let build_result = match ModuleGraphBuilder::new(&root)
        .with_extensions(config.extensions.clone())
        .with_exclude(config.exclude.clone())
        .with_entrypoints(config.entrypoints.clone())
        .build()
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error building graph: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let build_elapsed = build_start.elapsed();

    let analyze_start = Instant::now();
    let analyzer = Analyzer::from_config(&config.rules);
    let mut report = analyzer.run(&build_result.graph, &build_result.warnings);
    let analyze_elapsed = analyze_start.elapsed();

    report.stats.build_time_ms = build_elapsed.as_millis() as u64;
    report.stats.analyze_time_ms = analyze_elapsed.as_millis() as u64;

    match cli.format.as_str() {
        "trace" => {
            match render_trace_json(&report, &build_result.graph, &root, cli.trace_max_nodes) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    eprintln!("Error formatting trace: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
        "json" => match render_json(&report) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Error formatting JSON: {}", e);
                return ExitCode::FAILURE;
            }
        },
        _ => {
            let reporter = ConsoleReporter::new(use_color);
            print!("{}", reporter.render(&report, &build_result.graph));
        }
    }

    if report.has_errors() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
