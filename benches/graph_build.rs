use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use gruffed_builder::ModuleGraphBuilder;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name)
        .join("src")
}

fn bench_build_simple(c: &mut Criterion) {
    c.bench_function("build_simple", |b| {
        b.iter(|| {
            black_box(
                ModuleGraphBuilder::new(fixture_path("simple"))
                    .build()
                    .unwrap(),
            )
        })
    });
}

fn bench_build_long_chain(c: &mut Criterion) {
    c.bench_function("build_long_chain", |b| {
        b.iter(|| {
            black_box(
                ModuleGraphBuilder::new(fixture_path("long-chain"))
                    .build()
                    .unwrap(),
            )
        })
    });
}

fn bench_build_synthetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_synthetic");
    group.sample_size(10);

    for &n in &[100, 500, 2000] {
        let dir = generate_synthetic_project(n);
        group.bench_with_input(BenchmarkId::from_parameter(n), &dir, |b, dir| {
            b.iter(|| {
                black_box(ModuleGraphBuilder::new(dir).build().unwrap());
            })
        });
    }

    group.finish();
}

fn generate_synthetic_project(file_count: usize) -> PathBuf {
    let dir = tempfile::tempdir().unwrap().into_path();
    for i in 0..file_count {
        // Each file imports 5 other modules and has ~50 lines of content
        // to simulate realistic parse cost.
        let mut content = String::with_capacity(2048);
        for j in 0..5 {
            let target = (i + j + 1) % file_count;
            content.push_str(&format!(
                "import {{ func{j} }} from \"./mod{target}\";\n",
                j = j,
                target = target,
            ));
        }
        content.push_str(&format!("\nexport const x{i} = {i};\n\n", i = i));
        for k in 0..40 {
            content.push_str(&format!(
                "const helper{k} = (a: number, b: number): number => a * b + {k};\n",
                k = k
            ));
        }
        content.push_str(&format!(
            "export function func0(x: number): number {{ return helper0(x, x{i}); }}\n",
            i = i
        ));
        std::fs::write(dir.join(format!("mod{i}.ts", i = i)), content).unwrap();
    }
    dir
}

criterion_group!(
    benches,
    bench_build_simple,
    bench_build_long_chain,
    bench_build_synthetic
);
criterion_main!(benches);
