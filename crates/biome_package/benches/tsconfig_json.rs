use biome_deserialize::json::deserialize_from_json_str;
use biome_json_parser::JsonParserOptions;
use biome_package::TsConfigJson;
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

#[cfg(all(target_os = "linux", target_env = "musl"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[divan::bench(name = "tsconfig_json")]
fn bench_tsconfig_json(bencher: Bencher) {
    bencher
        .with_inputs(|| include_str!("tsconfig_bench.json"))
        .bench_values(|code| {
            deserialize_from_json_str::<TsConfigJson>(
                code,
                JsonParserOptions::default(),
                "tsconfig.json",
            )
        });
}
