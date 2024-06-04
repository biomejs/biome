use biome_grit_parser::parse_grit;
use biome_grit_patterns::{GritQuery, GritTargetLanguage, JsTargetLanguage};

// Use this test to quickly execute a Grit query against an source snippet.
#[ignore]
#[test]
fn test_query() {
    let parse_result = parse_grit("`console.log('hello')`");
    if !parse_result.diagnostics().is_empty() {
        println!(
            "Diagnostics from parsing query:\n{:?}",
            parse_result.diagnostics()
        );
    }

    let query = GritQuery::from_node(
        parse_result.tree(),
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
    )
    .expect("could not construct query");

    query.execute().expect("could not execute query");
}
