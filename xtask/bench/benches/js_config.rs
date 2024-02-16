use biome_fs::MemoryFileSystem;
use biome_service::configuration::load_configuration;
use biome_service::{ConfigurationBasePath, DynRef};
use std::collections::BTreeMap;
use xtask_bench::{criterion_group, criterion_main, BenchmarkId, Criterion};
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
fn bench_js_config(criterion: &mut Criterion) {
    let configs = BTreeMap::from([("js_config_simple", include_str!("js_config_simple.js"))]);

    let mut group = criterion.benchmark_group("js_config");

    for (config_name, content) in configs {
        group.throughput(criterion::Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(config_name),
            &content,
            |b, _| {
                let mut fs = MemoryFileSystem::default();
                fs.set_working_directory("/");
                fs.insert("/biome.config.js".into(), content);

                b.iter(|| {
                    criterion::black_box(
                        load_configuration(
                            &DynRef::Borrowed(&mut fs),
                            ConfigurationBasePath::default(),
                        )
                        .unwrap(),
                    );
                });
            },
        );
    }

    group.finish();
}

criterion_group!(js_config, bench_js_config);
criterion_main!(js_config);
