use biome_fs::BiomePath;
use camino::Utf8Path;
use std::hint::black_box;

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn normalized_path() -> BiomePath {
    BiomePath::new(black_box(Utf8Path::new(
        "/project/src/module49/file4999.js",
    )))
}

#[divan::bench]
fn path_with_parent_components() -> BiomePath {
    BiomePath::new(black_box(Utf8Path::new(
        "/project/src/module49/../module49/file4999.js",
    )))
}
