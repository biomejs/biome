use biome_string_case::StrLikeExtension;
use std::hint::black_box;

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(consts = [0, 32, 256], args = [8, 64, 512])]
fn optimized<const PREFIX: usize>(bencher: divan::Bencher, size: usize) {
    let prefix = "shared-prefix-".repeat(PREFIX.div_ceil(14));
    let prefix = &prefix[..PREFIX];
    let entries: Vec<_> = (0..size)
        .map(|i| format!("{prefix}property-{:04x}", i * 2))
        .collect();
    let queries: Vec<_> = (0..128)
        .map(|i| {
            let mut key = format!("{prefix}property-{:04x}", (i * 313) % (size * 2 + 1));
            if i % 3 == 0 {
                key.make_ascii_uppercase();
            } else if i % 3 == 1 {
                key[..PREFIX + 1].make_ascii_uppercase();
            }
            key
        })
        .collect();
    bencher
        .counter(divan::counter::ItemsCount::new(queries.len()))
        .bench_local(|| {
            for query in &queries {
                let _ = black_box(black_box(&entries).binary_search_by(|entry| {
                    entry
                        .as_str()
                        .cmp_ignore_ascii_case(black_box(query.as_str()))
                }));
            }
        });
}

#[divan::bench]
fn keywords(bencher: divan::Bencher) {
    let entries = [
        "align-content",
        "align-items",
        "animation",
        "background",
        "border",
        "bottom",
        "box-shadow",
        "color",
        "display",
        "fill",
        "flex",
        "float",
        "font",
        "gap",
        "grid",
        "height",
        "inset",
        "justify-content",
        "left",
        "line-height",
        "margin",
        "opacity",
        "order",
        "overflow",
        "padding",
        "position",
        "right",
        "stroke",
        "top",
        "transform",
        "width",
        "z-index",
    ];
    let queries = [
        "COLOR",
        "not-a-property",
        "a",
        "",
        "Background",
        "DISPLAY",
        "z-index",
        "FLEX",
        "Font",
        "width",
        "padding",
        "Border",
        "alignment",
        "Position",
        "transform",
        "zzz",
        "LEFT",
        "float",
        "Gap",
        "opacity",
        "grid",
        "height",
        "order",
        "stroke",
    ];
    bencher
        .counter(divan::counter::ItemsCount::new(queries.len()))
        .bench_local(|| {
            for query in queries {
                let _ = black_box(
                    black_box(&entries)
                        .binary_search_by(|entry| entry.cmp_ignore_ascii_case(black_box(query))),
                );
            }
        });
}
