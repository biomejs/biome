use biome_grit_patterns::{
    compile_pattern_with_options, CompilePatternOptions, GritTargetFile, GritTargetLanguage,
    JsTargetLanguage,
};
use camino::Utf8Path;
use criterion::measurement::WallTime;
use std::collections::HashMap;
use xtask_bench::TestCase;

#[cfg(not(feature = "codspeed"))]
pub use criterion::*;

#[cfg(feature = "codspeed")]
pub use codspeed_criterion_compat::*;

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
fn bench_gritql_search(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("gritql", include_str!("libs-ts.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("gritql_search");
    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                bench_search_group(&mut group, test_case);
            }
            Err(error) => println!("{error:?}"),
        }
    }
    group.finish();
}

pub fn bench_search_group(group: &mut BenchmarkGroup<WallTime>, test_case: TestCase) {
    let target_language = GritTargetLanguage::JsTargetLanguage(JsTargetLanguage);

    let query = compile_pattern_with_options(
        "`getEntityNameForExtendingInterface(errorLocation)`",
        CompilePatternOptions::default().with_path(Utf8Path::new("bench.grit")),
    )
    .unwrap();

    let code = test_case.code();
    let target_file = GritTargetFile::parse(code, test_case.path().to_owned(), target_language);

    group.throughput(Throughput::Bytes(code.len() as u64));
    group.sample_size(10);
    group.bench_with_input(
        BenchmarkId::new(test_case.filename(), "execute"),
        &code,
        |b, _| {
            b.iter(|| {
                let query_result =
                    black_box(query.execute(target_file.clone())).expect("Couldn't execute query");
                for log in query_result.logs.logs() {
                    println!("{log}");
                }
            })
        },
    );
}

criterion_group!(gritql_search, bench_gritql_search);
criterion_main!(gritql_search);
