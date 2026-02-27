use biome_formatter::Printed;
use biome_html_formatter::{HtmlFormatOptions, format_node};
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{HtmlFileSource, HtmlRoot};
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;
use biome_test_utils::BenchCase;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::{collections::HashMap, fs, path::Path};

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
fn load_fixtures() -> Vec<(String, String, String)> {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures");
    let mut cases = Vec::new();

    fn visit(dir: &Path, root: &Path, cases: &mut Vec<(String, String, String)>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, root, cases);
                } else if path.is_file() {
                    if matches!(path.extension().and_then(|e| e.to_str()), Some(ext) if ext.eq_ignore_ascii_case("md"))
                    {
                        continue;
                    }
                    let rel = path.strip_prefix(root).unwrap_or(&path);
                    let group = rel
                        .iter()
                        .next()
                        .and_then(|s| s.to_str())
                        .unwrap_or("root")
                        .to_string();
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string();
                    if let Ok(content) = fs::read_to_string(&path) {
                        cases.push((group, name, content));
                    }
                }
            }
        }
    }

    visit(&fixtures_root, &fixtures_root, &mut cases);
    cases
}

fn bench_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("html", include_str!("libs-html.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));
    let fixtures = load_fixtures();

    let mut group = criterion.benchmark_group("html_formatter");

    for lib in libs {
        let test_case = BenchCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                let file_source = HtmlFileSource::try_from(test_case.path()).unwrap_or_default();
                let parsed = parse_html(code, HtmlParseOptions::from(&file_source));
                group.throughput(Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    &code,
                    |b, _| {
                        fn format(root: HtmlRoot) -> Printed {
                            let formatted =
                                format_node(HtmlFormatOptions::default(), root.syntax(), false)
                                    .unwrap();
                            let printed = formatted.print();
                            drop(formatted);
                            printed.expect("Document to be valid")
                        }
                        b.iter(|| {
                            black_box(format(parsed.tree()));
                        })
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }
    // Benchmark local fixtures with group names included in the benchmark ID
    for (group_name, name, content) in fixtures {
        let code = &content;
        let ext = name
            .rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase_cow();
        let file_source = HtmlFileSource::try_from_extension(&ext).unwrap_or_default();
        let parsed = parse_html(code, HtmlParseOptions::from(&file_source));
        group.throughput(Throughput::Bytes(code.len() as u64));
        let id = format!("{}/{}", group_name, name);
        group.bench_with_input(BenchmarkId::new(&id, "format"), code, |b, _| {
            fn format(root: HtmlRoot) -> Printed {
                let formatted =
                    format_node(HtmlFormatOptions::default(), root.syntax(), false).unwrap();
                let printed = formatted.print();
                drop(formatted);
                printed.expect("Document to be valid")
            }
            b.iter(|| {
                black_box(format(parsed.tree()));
            })
        });
    }

    group.finish();
}

criterion_group!(html_formatter, bench_formatter);
criterion_main!(html_formatter);
