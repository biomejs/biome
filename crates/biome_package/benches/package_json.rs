use biome_deserialize::json::deserialize_from_json_str;
use biome_json_parser::JsonParserOptions;
use biome_package::PackageJson;
use divan::Bencher;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

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

#[divan::bench(name = "package_json")]
fn bench_package_json(bencher: Bencher) {
    bencher
        .with_inputs(|| include_str!("package_bench.json"))
        .bench_values(|code| {
            deserialize_from_json_str::<PackageJson>(
                code,
                JsonParserOptions::default(),
                "package.json",
            )
        });
}
