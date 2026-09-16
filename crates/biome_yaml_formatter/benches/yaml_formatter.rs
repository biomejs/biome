use biome_formatter::Printed;
use biome_test_utils::BenchCase;
use biome_yaml_formatter::{YamlFormatOptions, format_node};
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

fn bench_formatter(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("yaml_formatter");
    for url in include_str!("libs-yaml.txt").lines() {
        let test_case = match BenchCase::try_from(url) {
            Ok(test_case) => test_case,
            Err(error) => {
                eprintln!("failed to load YAML benchmark `{url}`: {error}");
                continue;
            }
        };
        let code = test_case.code();
        let parsed = parse_yaml(code);
        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(test_case.filename()),
            &code,
            |bencher, _| {
                fn format(root: &biome_yaml_syntax::YamlSyntaxNode) -> Printed {
                    let formatted = format_node(YamlFormatOptions::default(), root).unwrap();
                    let printed = formatted.print();
                    drop(formatted);
                    printed.expect("Document to be valid")
                }
                bencher.iter(|| black_box(format(&parsed.syntax())));
            },
        );
    }
    group.finish();
}

criterion_group!(yaml_formatter, bench_formatter);
criterion_main!(yaml_formatter);
