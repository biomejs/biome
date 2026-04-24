use std::{collections::HashMap, fs, path::Path};

use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleCategoriesBuilder,
};
use biome_html_analyze::HtmlAnalyzerServices;
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::HtmlFileSource;
use biome_test_utils::BenchCase;
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

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

fn load_fixtures() -> Result<Vec<(String, String, String)>, std::io::Error> {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures");
    let mut cases = Vec::new();

    fn visit(
        dir: &Path,
        root: &Path,
        cases: &mut Vec<(String, String, String)>,
    ) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit(&path, root, cases)?;
            } else if path.is_file() {
                if matches!(path.extension().and_then(|e| e.to_str()), Some(ext) if ext.eq_ignore_ascii_case("md"))
                {
                    continue;
                }

                let rel = path.strip_prefix(root).unwrap_or(&path);
                let group = rel
                    .iter()
                    .next()
                    .and_then(|segment| segment.to_str())
                    .unwrap_or("root")
                    .to_string();
                let name = path
                    .file_name()
                    .and_then(|segment| segment.to_str())
                    .unwrap_or_default()
                    .to_string();
                let content = fs::read_to_string(&path)?;

                cases.push((group, name, content));
            }
        }

        Ok(())
    }

    visit(&fixtures_root, &fixtures_root, &mut cases)?;
    Ok(cases)
}

fn bench_analyzer(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("html", include_str!("libs-html.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));
    let fixtures = load_fixtures().expect("failed to load fixtures");

    let mut group = criterion.benchmark_group("html_analyzer");

    for lib in libs {
        let test_case = BenchCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                let file_source = HtmlFileSource::try_from(test_case.path()).unwrap_or_else(|_| {
                    panic!("failed to determine file source for {}", test_case.path())
                });
                let parse = parse_html(code, HtmlParserOptions::from(&file_source));

                group.throughput(Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    code,
                    |b, _| {
                        let filter = AnalysisFilter {
                            categories: RuleCategoriesBuilder::default()
                                .with_syntax()
                                .with_lint()
                                .with_assist()
                                .build(),
                            ..AnalysisFilter::default()
                        };
                        let options = AnalyzerOptions::default()
                            .with_configuration(AnalyzerConfiguration::default());

                        b.iter(|| {
                            biome_html_analyze::analyze(
                                &parse.tree(),
                                filter,
                                &options,
                                file_source,
                                biome_html_analyze::HtmlAnalyzerServices::default(),
                                |event| {
                                    black_box(event.diagnostic());
                                    black_box(event.actions(ActionFilter::all()));
                                    ControlFlow::<Never>::Continue(())
                                },
                            );
                        });
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }

    for (group_name, name, content) in fixtures {
        let code = content.as_str();
        let ext = name.rsplit('.').next().unwrap_or_default();
        let file_source = HtmlFileSource::try_from_extension(ext).unwrap_or_else(|_| {
            panic!("failed to determine file source for fixture {group_name}/{name}")
        });
        let parse = parse_html(code, HtmlParserOptions::from(&file_source));
        let id = format!("{group_name}/{name}");

        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(&id), code, |b, _| {
            let filter = AnalysisFilter {
                categories: RuleCategoriesBuilder::default()
                    .with_syntax()
                    .with_lint()
                    .with_assist()
                    .build(),
                ..AnalysisFilter::default()
            };
            let options = AnalyzerOptions::default().with_configuration(
                AnalyzerConfiguration::default().with_jsx_runtime(JsxRuntime::default()),
            );

            b.iter(|| {
                biome_html_analyze::analyze(
                    &parse.tree(),
                    filter,
                    &options,
                    file_source,
                    HtmlAnalyzerServices::default(),
                    |event| {
                        black_box(event.diagnostic());
                        black_box(event.actions(ActionFilter::all()));
                        ControlFlow::<Never>::Continue(())
                    },
                );
            });
        });
    }

    group.finish();
}

criterion_group!(html_analyzer, bench_analyzer);
criterion_main!(html_analyzer);
