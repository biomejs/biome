use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"{#snippet f()}
    <p></p>
{/snippet}"#;

    let options = HtmlParseOptions::default().with_single_text_expression();
    let root = parse_html(code, options);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
