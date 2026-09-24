use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleCategoriesBuilder,
};
use biome_yaml_analyze::analyze;
use biome_yaml_parser::parse_yaml;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn bench_analyzer(criterion: &mut Criterion) {
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../biome_yaml_parser/benches/fixtures/real");
    let mut group = criterion.benchmark_group("yaml_analyze");
    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default()
            .with_syntax()
            .with_lint()
            .with_assist()
            .build(),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default();

    for entry in std::fs::read_dir(fixtures).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("yaml") {
            continue;
        }
        let code = std::fs::read_to_string(&path).unwrap();
        let parse = parse_yaml(&code);
        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(path.file_name().unwrap().to_string_lossy()),
            &code,
            |bencher, _| {
                bencher.iter(|| {
                    analyze(&parse.tree(), filter, &options, |event| {
                        black_box(event.diagnostic());
                        black_box(event.actions(ActionFilter::all()));
                        ControlFlow::<Never>::Continue(())
                    });
                });
            },
        );
    }
    group.finish();
}

criterion_group!(yaml_analyze, bench_analyzer);
criterion_main!(yaml_analyze);
