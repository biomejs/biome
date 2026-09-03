use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_languages::HtmlFileSource;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_parser(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("html_parser_walltime");

    for (name, extension, code) in [
        (
            "walltime/high-depth.html",
            "html",
            include_str!("fixtures/walltime/high-depth.html"),
        ),
        (
            "walltime/attribute-heavy.html",
            "html",
            include_str!("fixtures/walltime/attribute-heavy.html"),
        ),
        (
            "walltime/astro-components.astro",
            "astro",
            include_str!("fixtures/walltime/astro-components.astro"),
        ),
        (
            "walltime/svelte-control-flow.svelte",
            "svelte",
            include_str!("fixtures/walltime/svelte-control-flow.svelte"),
        ),
        (
            "walltime/vue-expressions.vue",
            "vue",
            include_str!("fixtures/walltime/vue-expressions.vue"),
        ),
    ] {
        let file_source = HtmlFileSource::try_from_extension(extension).unwrap_or_default();
        group.bench_with_input(BenchmarkId::new(name, "uncached"), &code, |b, code| {
            b.iter(|| black_box(parse_html(code, HtmlParserOptions::from(&file_source))));
        });
    }

    group.finish();
}

criterion_group!(html_parser_walltime, bench_parser);
criterion_main!(html_parser_walltime);
