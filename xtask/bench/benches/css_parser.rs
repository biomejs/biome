use biome_rowan::NodeCache;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::collections::HashMap;
use xtask_bench::{Parse, TestCase};

fn bench_css_parser(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("css", include_str!("libs-css.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let parse = Parse::try_from_case(&test_case).expect("Supported language");

                let code = test_case.code();

                let mut group = criterion.benchmark_group("css_parser");
                group.throughput(criterion::Throughput::Bytes(code.len() as u64));

                group.bench_function(BenchmarkId::new(test_case.filename(), "uncached"), |b| {
                    b.iter(|| {
                        criterion::black_box(parse.parse());
                    })
                });
                group.bench_function(BenchmarkId::new(test_case.filename(), "cached"), |b| {
                    b.iter_batched(
                        || {
                            let mut cache = NodeCache::default();
                            parse.parse_with_cache(&mut cache);
                            cache
                        },
                        |mut cache| {
                            criterion::black_box(parse.parse_with_cache(&mut cache));
                        },
                        BatchSize::SmallInput,
                    )
                });

                group.finish();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

criterion_group!(css_parser, bench_css_parser);
criterion_main!(css_parser);
