use biome_package::node_semver::{Range, Version};

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

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn input_ranges() -> impl Iterator<Item = String> {
    vec![
        "~1.2.3".into(),
        "^1.2.3".into(),
        ">=1.2.3".into(),
        "<=1.2.3".into(),
        "1.2".into(),
        "1.2.3 - 2.3.4".into(),
        ">=1.2.9 <2.0.0".into(),
        "1.2.7 || >=1.2.9 <2.0.0".into(),
    ]
    .into_iter()
}

fn input_versions() -> impl Iterator<Item = String> {
    vec![
        "1.0.0-alpha".into(),
        "2.3.533".into(),
        "0.3.533".into(),
        "1.0.0-alpha.1".into(),
        "1.0.0-alpha.beta".into(),
        "1.0.0-beta".into(),
        "1.0.0-beta.2".into(),
        "1.0.0-beta.11".into(),
        "1.0.0-rc.1".into(),
    ]
    .into_iter()
}

#[divan::bench(
    args = input_ranges()
)]
fn ranges(args: &str) {
    args.parse::<Range>().unwrap();
}

#[divan::bench(
    args = input_versions()
)]
fn versions(version: &str) {
    version.parse::<Version>().unwrap();
}
