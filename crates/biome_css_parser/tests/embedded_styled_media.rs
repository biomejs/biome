use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssFileSource, EmbeddingKind};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn parses_media_queries_in_styled_embeds() {
    let code = r#"
height: 20px;

@media screen and (min-width: 768px) {
  height: 40px;
}
"#;

    let parsed = parse_css(
        code,
        CssFileSource::css().with_embedding_kind(EmbeddingKind::Styled),
        CssParserOptions::default(),
    );

    let syntax = parsed.syntax();
    assert!(
        parsed.diagnostics().is_empty(),
        "Expected styled embed to parse without diagnostics, got: {:#?}",
        parsed.diagnostics()
    );
    assert!(
        !has_bogus_nodes_or_empty_slots(&syntax),
        "Styled embed contains bogus nodes or empty slots:\n{syntax:#?}"
    );
}
