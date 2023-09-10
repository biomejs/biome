mod features;
mod language;
mod test_case;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

use biome_rowan::NodeCache;
use criterion::{BatchSize, BenchmarkId};

pub use crate::features::analyzer::benchmark_analyze_lib;
use crate::features::analyzer::AnalyzerMeasurement;
pub use crate::features::formatter::benchmark_format_lib;
use crate::features::formatter::{run_format, FormatterMeasurement};
pub use crate::features::parser::benchmark_parse_lib;
use crate::features::parser::ParseMeasurement;
use crate::language::Parse;
use crate::test_case::TestCase;

/// What feature to benchmark
#[derive(Eq, PartialEq)]
pub enum FeatureToBenchmark {
    /// benchmark of the parser
    Parser,
    /// benchmark of the formatter
    Formatter,
    /// benchmark of the analyzer
    Analyzer,
}

impl FromStr for FeatureToBenchmark {
    type Err = pico_args::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parser" => Ok(Self::Parser),
            "formatter" => Ok(Self::Formatter),
            "analyzer" => Ok(Self::Analyzer),
            _ => Err(pico_args::Error::OptionWithoutAValue("feature")),
        }
    }
}

impl Display for FeatureToBenchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureToBenchmark::Parser => write!(f, "parser"),
            FeatureToBenchmark::Formatter => write!(f, "formatter"),
            FeatureToBenchmark::Analyzer => write!(f, "analyzer"),
        }
    }
}

/// If groups the summary by their category and creates a small interface
/// where each bench result can create their summary
pub enum BenchmarkSummary {
    Parser(ParseMeasurement),
    Formatter(FormatterMeasurement),
    Analyzer(AnalyzerMeasurement),
}

impl BenchmarkSummary {
    pub fn summary(&self) -> String {
        match self {
            BenchmarkSummary::Parser(result) => result.summary(),
            BenchmarkSummary::Formatter(result) => result.summary(),
            BenchmarkSummary::Analyzer(result) => result.summary(),
        }
    }
}

impl Display for BenchmarkSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkSummary::Parser(result) => std::fmt::Display::fmt(&result, f),
            BenchmarkSummary::Formatter(result) => std::fmt::Display::fmt(&result, f),
            BenchmarkSummary::Analyzer(result) => std::fmt::Display::fmt(&result, f),
        }
    }
}

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

pub struct RunArgs {
    pub filter: String,
    pub criterion: bool,
    pub baseline: Option<String>,
    pub feature: FeatureToBenchmark,
    pub suites: String,
}

pub fn run(args: RunArgs) {
    let regex = regex::Regex::new(args.filter.as_str()).unwrap();

    let mut all_suites = HashMap::new();
    if args.feature == FeatureToBenchmark::Analyzer {
        all_suites.insert("js", include_str!("analyzer-libs-js.txt"));
        all_suites.insert("ts", include_str!("analyzer-libs-ts.txt"));
    } else {
        all_suites.insert("js", include_str!("libs-js.txt"));
        all_suites.insert("ts", include_str!("libs-ts.txt"));
        all_suites.insert("json", include_str!("libs-json.txt"));
    }

    let mut libs = vec![];
    let suites_to_run = args.suites.split(',');
    for suite in suites_to_run {
        match suite {
            "*" => {
                libs.extend(all_suites.values().flat_map(|suite| suite.lines()));
            }
            key => match all_suites.get(key) {
                Some(suite) => libs.extend(suite.lines()),
                None => {
                    eprintln!("Unknown suite: {key}");
                }
            },
        }
    }

    let mut summary = vec![];

    for lib in libs {
        if !regex.is_match(lib) {
            continue;
        }

        let test_case = TestCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let parse = Parse::try_from_case(&test_case).expect("Supported language");

                let code = test_case.code();

                // Do all steps with criterion now
                if args.criterion {
                    let mut criterion = criterion::Criterion::default()
                        .without_plots()
                        .measurement_time(Duration::new(10, 0));
                    if let Some(ref baseline) = args.baseline {
                        criterion = criterion.save_baseline(baseline.to_string());
                    }
                    let mut group = criterion.benchmark_group(args.feature.to_string());
                    group.throughput(criterion::Throughput::Bytes(code.len() as u64));

                    match args.feature {
                        FeatureToBenchmark::Parser => {
                            group.bench_function(
                                BenchmarkId::new(test_case.filename(), "uncached"),
                                |b| {
                                    b.iter(|| {
                                        criterion::black_box(parse.parse());
                                    })
                                },
                            );
                            group.bench_function(
                                BenchmarkId::new(test_case.filename(), "cached"),
                                |b| {
                                    b.iter_batched(
                                        || {
                                            let mut cache = NodeCache::default();
                                            parse.parse_with_cache(&mut cache);
                                            cache
                                        },
                                        |mut cache| {
                                            criterion::black_box(
                                                parse.parse_with_cache(&mut cache),
                                            );
                                        },
                                        BatchSize::SmallInput,
                                    )
                                },
                            );
                        }
                        FeatureToBenchmark::Formatter => {
                            let parsed = parse.parse();

                            match parsed.format_node() {
                                None => {
                                    continue;
                                }
                                Some(format_node) => {
                                    group.bench_function(test_case.filename(), |b| {
                                        b.iter(|| {
                                            criterion::black_box(run_format(&format_node));
                                        })
                                    });
                                }
                            }
                        }
                        FeatureToBenchmark::Analyzer => {
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
                        }
                    }

                    group.finish();
                }

                let result = match args.feature {
                    FeatureToBenchmark::Parser => benchmark_parse_lib(&test_case, &parse),
                    FeatureToBenchmark::Formatter => {
                        let parsed = parse.parse();
                        let format_node = parsed
                            .format_node()
                            .expect("Expect formatting to be supported");

                        benchmark_format_lib(test_case.filename(), &format_node)
                    }
                    FeatureToBenchmark::Analyzer => {
                        let parsed = parse.parse();
                        let analyze = parsed.analyze().expect("Expect analyze to be supported");
                        benchmark_analyze_lib(&test_case, &analyze)
                    }
                };

                summary.push(result.summary());

                println!("Benchmark: {}", lib);
                println!("{}", result);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Summary");
    println!("-------");
    for l in summary {
        println!("{}", l);
    }
}
