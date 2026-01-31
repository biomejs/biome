use biome_markdown_parser::parse_markdown;

#[test]
fn tracks_list_tightness() {
    let source = "- a\n- b\n\npara\n\n- a\n\n  b\n";
    let parse = parse_markdown(source);
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
