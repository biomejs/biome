use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    // Test simple binding
    let code = r#"<template><div v-for="item in items">{{ item }}</div></template>"#;

    let options = HtmlParseOptions::default()
        .with_vue()
        .with_double_text_expression();
    let root = parse_html(code, options);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
