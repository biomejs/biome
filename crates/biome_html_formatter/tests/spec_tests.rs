use biome_formatter::FormatError;
use biome_html_formatter::{HtmlFormatOptions, format_node};
use biome_html_parser::parse_html;
use biome_languages::HtmlFileSource;

mod quick_test;
mod spec_test;

#[test]
fn uncertain_svelte_boundaries_prevent_a_partial_rewrite() {
    let source_type = HtmlFileSource::svelte();
    for source in [
        "<!-- biome-ignore format: keep -->\n<div>{@unknown {}</div>\n<p class = \"ok\"></p>",
        "<script lang=\"ts\">  let value = 0\n<p class = \"still-script\"></p>",
        "<p class = \"ok\"></p>\n{#if broken +}{item}{/}",
    ] {
        let parsed = parse_html(source, (&source_type).into());
        assert!(
            matches!(
                format_node(
                    HtmlFormatOptions::new(source_type),
                    &parsed.syntax(),
                    Vec::new()
                ),
                Err(FormatError::SyntaxError)
            ),
            "expected formatting to stop at an uncertain boundary in {source:?}"
        );
    }
}

mod formatter {

    mod html {
        tests_macros::gen_tests! {"tests/specs/html/**/*.{html,vue,svg,svelte,astro}", crate::spec_test::run, ""}
    }
}
