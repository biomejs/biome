use biome_grit_patterns::testing::{compile_js_query, make_js_file};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

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

/// Sample JS code with multiple patterns to match against.
const JS_CODE: &str = r#"
import fs from "fs";

console.log("start");

function processItems(items) {
    const results = new Array(items.length);
    for (let i = 0; i < items.length; i++) {
        const item = items[i];
        console.log("processing", item);
        if (typeof item === "undefined") {
            continue;
        }
        results[i] = item.toString();
    }
    console.warn("done", results.length);
    return results;
}

const buffer = new Buffer(1024);
const x = typeof window !== "undefined" ? window : global;
console.log("end");
"#;

fn bench_execute(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("grit_query_execute");

    let patterns = [
        ("code_snippet", "`console.log($msg)`"),
        (
            "where_clause",
            r#"`console.log($msg)` where { $msg <: `"start"` }"#,
        ),
        (
            "or_pattern",
            "or { `console.log($msg)`, `console.warn($msg)` }",
        ),
    ];

    for (name, pattern_src) in &patterns {
        let query = compile_js_query(pattern_src);
        let base_file = make_js_file(JS_CODE);

        group.bench_with_input(BenchmarkId::new("execute", name), pattern_src, |b, _| {
            b.iter(|| {
                black_box(query.execute(base_file.clone()).unwrap());
            });
        });

        group.bench_with_input(
            BenchmarkId::new("execute_optimized", name),
            pattern_src,
            |b, _| {
                b.iter(|| {
                    black_box(query.execute_optimized(base_file.clone()).unwrap());
                });
            },
        );
    }

    group.finish();
}

fn bench_anchor_extraction(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("grit_query_anchor_kinds");

    let patterns = [
        ("code_snippet", "`console.log($msg)`"),
        (
            "or_pattern",
            "or { `console.log($msg)`, `console.warn($msg)` }",
        ),
        ("metavariable", "$x"),
    ];

    for (name, pattern_src) in &patterns {
        let query = compile_js_query(pattern_src);

        group.bench_with_input(BenchmarkId::from_parameter(name), pattern_src, |b, _| {
            b.iter(|| {
                black_box(query.anchor_kinds());
            });
        });
    }

    group.finish();
}

criterion_group!(grit_query, bench_execute, bench_anchor_extraction);
criterion_main!(grit_query);
