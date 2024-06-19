use biome_grit_parser::parse_grit;
use biome_grit_patterns::{GritQuery, GritTargetLanguage, GritTargetTree, JsTargetLanguage};
use biome_js_parser::{parse_module, JsParserOptions};

// Use this test to quickly execute a Grit query against a source snippet.
#[ignore]
#[test]
fn test_query() {
    let parse_grit_result = parse_grit("`console.log('hello')` => `doei()`");
    if !parse_grit_result.diagnostics().is_empty() {
        println!(
            "Diagnostics from parsing query:\n{:?}",
            parse_grit_result.diagnostics()
        );
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
    )
    .expect("could not construct query");

    if !query.diagnostics.is_empty() {
        println!("Diagnostics from compiling query:\n{:?}", query.diagnostics);
    }

    println!("Query pattern: {:#?}", query.pattern);

    let parse_js_result = parse_module(
        r#"
function hello() {
    console
        .log("hello");
}
"#,
        JsParserOptions::default(),
    );
    if !parse_js_result.diagnostics().is_empty() {
        println!(
            "Diagnostics from parsing JS snippet:\n{:?}",
            parse_js_result.diagnostics()
        );
    }

    let tree = GritTargetTree::new(parse_js_result.syntax().into());
    let effects = query.execute(&tree).expect("could not execute query");

    println!("Effects: {effects:?}");
}
