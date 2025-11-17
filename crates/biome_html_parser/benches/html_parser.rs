use std::{collections::HashMap, fs, path::Path};

use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};
use biome_html_parser::{HtmlParseOptions, parse_html, parse_html_with_cache};
use biome_html_syntax::HtmlFileSource;
use biome_rowan::NodeCache;
use biome_string_case::StrLikeExtension;
use biome_test_utils::BenchCase;
use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};

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

fn bench_parser(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("html", include_str!("libs-html.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let fixtures = load_fixtures();

    let mut group = criterion.benchmark_group("html_parser");
    for lib in libs {
        let test_case = BenchCase::try_from(lib);
        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                group.throughput(Throughput::Bytes(code.len() as u64));
                let file_source = HtmlFileSource::try_from(test_case.path()).unwrap_or_default();
                group.bench_with_input(
                    BenchmarkId::new(test_case.filename(), "uncached"),
                    &code,
                    |b, _| {
                        b.iter(|| {
                            let result =
                                black_box(parse_html(code, HtmlParseOptions::from(&file_source)));
                            if !result.diagnostics().is_empty() {
                                let truncated = result.into_diagnostics().iter().take(20).cloned().collect::<Vec<_>>();
                                for diagnostic in truncated {
                                    let diagnostic = diagnostic
                                        .with_file_source_code(code)
                                        .with_file_path(test_case.filename());
                                    println!("{}", print_diagnostic_to_string(&diagnostic));
                                }
                                panic!("Parsing errors detected in benchmark. Only some of the diagnostics are printed above.");
                            }
                        })
                    },
                );
                group.bench_with_input(
                    BenchmarkId::new(test_case.filename(), "cached"),
                    &code,
                    |b, _| {
                        b.iter_batched(
                            || {
                                let mut cache = NodeCache::default();
                                parse_html_with_cache(
                                    code,
                                    &mut cache,
                                    HtmlParseOptions::from(&file_source),
                                );
                                cache
                            },
                            |mut cache| {
                                parse_html_with_cache(
                                    code,
                                    &mut cache,
                                    HtmlParseOptions::from(&file_source),
                                );
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }

    // Benchmark local fixtures (recursively discovered), including their group names
    for (group_name, name, content) in fixtures {
        let code = content.as_str();
        let mut diagnostics = vec![];
        group.throughput(Throughput::Bytes(code.len() as u64));
        let ext = name
            .rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase_cow();
        let file_source = HtmlFileSource::try_from_extension(&ext).unwrap_or_default();

        let id = format!("{}/{}", group_name, name);
        group.bench_with_input(BenchmarkId::new(&id, "uncached"), &code, |b, _| {
            b.iter(|| {
                let result = black_box(parse_html(code, HtmlParseOptions::from(&file_source)));
                diagnostics.extend(result.into_diagnostics());
            })
        });

        for diagnostic in diagnostics {
            let diagnostic = diagnostic.with_file_source_code(code).with_file_path(&id);
            println!("{}", print_diagnostic_to_string(&diagnostic));
        }

        group.bench_with_input(BenchmarkId::new(&id, "cached"), &code, |b, _| {
            b.iter_batched(
                || {
                    let mut cache = NodeCache::default();
                    parse_html_with_cache(code, &mut cache, HtmlParseOptions::from(&file_source));
                    cache
                },
                |mut cache| {
                    parse_html_with_cache(code, &mut cache, HtmlParseOptions::from(&file_source));
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(html_parser, bench_parser);
criterion_main!(html_parser);
