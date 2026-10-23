use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};

#[test]
fn tracks_list_tightness() {
    let source = "- a\n- b\n\npara\n\n- a\n\n  b\n";
    let parse = parse_markdown(source, MarkdownParserOptions::default());
    let tightness = parse.list_tightness();

    assert_eq!(tightness.len(), 2);

    let mut found_tight = false;
    let mut found_loose = false;

    for entry in tightness {
        let start: usize = entry.range.start().into();
        let end: usize = entry.range.end().into();
        let text = &source[start..end];

        if text.contains("- a\n- b") {
            assert!(entry.is_tight);
            found_tight = true;
        }

        if text.contains("- a\n\n  b") {
            assert!(!entry.is_tight);
            found_loose = true;
        }
    }

    assert!(found_tight);
    assert!(found_loose);
}

#[test]
fn nested_list_does_not_leak_blank_line_into_following_list() {
    let source = "- outer\n\n  * nested\n1. x\n";
    let parse = parse_markdown(source, MarkdownParserOptions::default());

    let following = parse
        .list_tightness()
        .iter()
        .find(|entry| &source[entry.range] == "1. x\n")
        .expect("the ordered list is recorded");

    assert!(following.is_tight);
}
