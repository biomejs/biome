use biome_js_parser::{JsParserOptions, parse_js_with_offset};
use biome_js_syntax::JsFileSource;
use biome_rowan::TextSize;

#[test]
fn test_offset_parsing_basic() {
    let js_code = "console.log('hello');";
    let base_offset = TextSize::from(100);

    let parse = parse_js_with_offset(
        js_code,
        base_offset,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    // Verify no parsing errors
    assert!(!parse.has_errors(), "Parse should not have errors");

    // Verify the base offset is correctly set
    assert_eq!(parse.base_offset(), base_offset);

    // Verify the syntax tree text ranges are offset
    let syntax = parse.syntax();
    let root_range = syntax.text_range_with_trivia();

    // The root should start at the base offset
    assert_eq!(root_range.start(), base_offset);

    // The end should be base_offset + length of the text
    let expected_end = base_offset + TextSize::from(js_code.len() as u32);
    assert_eq!(root_range.end(), expected_end);
}

#[test]
fn test_offset_parsing_vs_regular_parsing() {
    let js_code = "function test() { return 42; }";
    let base_offset = TextSize::from(50);

    // Parse with offset
    let offset_parse = parse_js_with_offset(
        js_code,
        base_offset,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    // Parse normally
    let normal_parse = biome_js_parser::parse_js_with_cache(
        js_code,
        JsFileSource::js_module(),
        JsParserOptions::default(),
        &mut biome_rowan::NodeCache::default(),
    );

    // Both should have same number of errors (hopefully none)
    assert_eq!(offset_parse.has_errors(), normal_parse.has_errors());

    // The offset parse should have all ranges shifted by base_offset
    let offset_range = offset_parse.syntax().text_range_with_trivia();
    let normal_range = normal_parse.syntax().text_range_with_trivia();

    assert_eq!(offset_range.start(), normal_range.start() + base_offset);
    assert_eq!(offset_range.end(), normal_range.end() + base_offset);

    // The text content should be identical
    assert_eq!(
        offset_parse.syntax().inner().text_with_trivia().to_string(),
        normal_parse.syntax().text_with_trivia().to_string()
    );
}
