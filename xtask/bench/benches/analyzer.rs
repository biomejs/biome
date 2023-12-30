use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use xtask_bench::{Parse, TestCase};

fn bench_analyzer(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("js", include_str!("analyzer-libs-js.txt"));
    all_suites.insert("ts", include_str!("analyzer-libs-ts.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    for lib in libs {
        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let parse = Parse::try_from_case(&test_case).expect("Supported language");

                let code = test_case.code();

                let mut group = criterion.benchmark_group("analyzer");
                group.throughput(criterion::Throughput::Bytes(code.len() as u64));

                let parsed = parse.parse();

                match parsed.analyze() {
                    None => {
                        continue;
                    }
                    Some(analyze) => {
                        group.bench_function(test_case.filename(), |b| {
                            b.iter(|| {
                                analyze.analyze();
                                criterion::black_box(());
                            })
                        });
                    }
                }

                group.finish();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

criterion_group!(analyzer, bench_analyzer);
criterion_main!(analyzer);
