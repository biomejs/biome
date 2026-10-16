use biome_css_formatter::{context::CssFormatOptions, format_node};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssUrlValueRaw;
use biome_languages::CssFileSource;
use biome_rowan::AstNode;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn ordinary_urls_with_leading_delimiters_round_trip() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for payload in ["[a", "{a", "]a", "}a"] {
            let source = format!(".a {{background:url({payload}); color:red;}}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
            assert_eq!(parsed.syntax().to_string(), source);
            assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
            let url = parsed
                .syntax()
                .descendants()
                .find_map(CssUrlValueRaw::cast)
                .unwrap();
            assert_eq!(url.value_token().unwrap().text_trimmed(), payload);
            let options = CssFormatOptions::new(source_type);
            let formatted = format_node(options.clone(), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            let second = format_node(options, &reparsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert_eq!(formatted.as_code(), second.as_code());
            assert!(formatted.as_code().contains(&format!("url({payload})")));
        }
    }
}
