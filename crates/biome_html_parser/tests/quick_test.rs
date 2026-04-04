use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::HtmlFileSource;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"<Component
	bind:value

/>

"#;

    let source_type = HtmlFileSource::svelte();
    let options = HtmlParserOptions::from(&source_type);
    let root = parse_html(code, options);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
