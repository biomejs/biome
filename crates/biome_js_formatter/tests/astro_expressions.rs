//! Formatting of Astro template expression trees. These call [format_node]
//! directly: the workspace routes any JS document with Astro embedding through
//! the legacy `.astro` flow, which parses only the frontmatter, so spec
//! fixtures cannot cover expression sources.

use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_parser::{JsParserOptions, parse};
use biome_languages::{JsFileSource, javascript::JsEmbeddingKind};

/// Parses `source` as the brace-less body of an Astro `{...}` expression and
/// returns it formatted.
fn format_astro_expression(source: &str) -> String {
    let file_source = JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
        frontmatter: false,
        is_class_attribute: false,
    });
    let parsed = parse(source, file_source, JsParserOptions::default());
    assert!(
        parsed.diagnostics().is_empty(),
        "{source:?}: {:?}",
        parsed.diagnostics()
    );
    let formatted = format_node(
        JsFormatOptions::new(file_source),
        &parsed.syntax(),
        Vec::new(),
    )
    .expect("formatting failed");
    formatted
        .print()
        .expect("printing failed")
        .as_code()
        .to_string()
}

#[test]
fn html_comment_between_children_is_printed() {
    insta::assert_snapshot!(format_astro_expression(
        "x &&    <section><!-- single --><p>hi</p></section>"
    ));
}

#[test]
fn multiline_and_empty_html_comments_are_printed() {
    insta::assert_snapshot!(format_astro_expression(
        "x && <section><!-- line one\nline two --><!----></section>"
    ));
}

#[test]
fn comment_only_expression_body_is_printed() {
    insta::assert_snapshot!(format_astro_expression("<!-- only a comment -->"));
}

#[test]
fn html_comment_before_the_expression_is_printed() {
    insta::assert_snapshot!(format_astro_expression("<!-- lead --> <a></a>"));
}
