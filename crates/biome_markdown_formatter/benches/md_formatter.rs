use biome_formatter::Printed;
use biome_markdown_formatter::context::MdFormatOptions;
use biome_markdown_formatter::format_node;
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::{fs, path::Path};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;
fn bench_formatter(criterion: &mut Criterion) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../biome_markdown_parser/benches/fixtures/real");

    let mut group = criterion.benchmark_group("md_formatter");

    for entry in fs::read_dir(fixtures).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }

        let code = fs::read_to_string(&path).unwrap();
        let parsed = parse_markdown(&code, MarkdownParserOptions::default());
        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(path.file_name().unwrap().to_string_lossy()),
            &code,
            |b, _| {
                fn format(root: &biome_markdown_syntax::MarkdownSyntaxNode) -> Printed {
                    let formatted = format_node(MdFormatOptions::default(), root).unwrap();
                    let printed = formatted.print();
                    drop(formatted);
                    printed.expect("Document to be valid")
                }
                b.iter(|| black_box(format(&parsed.syntax())));
            },
        );
    }
    group.finish();
}

criterion_group!(md_formatter, bench_formatter);
criterion_main!(md_formatter);
