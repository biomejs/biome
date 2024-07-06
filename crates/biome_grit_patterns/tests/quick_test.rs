use std::path::Path;

use biome_grit_parser::parse_grit;
use biome_grit_patterns::{GritQuery, GritTargetFile, GritTargetLanguage, JsTargetLanguage};

// Use this test to quickly execute a Grit query against a source snippet.
#[ignore]
#[test]
fn test_query() {
    // TODO: Still need to implement autowrapping.
    let parse_grit_result = parse_grit("file(body = contains bubble `\"hello\"`)");
    if !parse_grit_result.diagnostics().is_empty() {
        println!(
            "Diagnostics from parsing query:\n{:?}",
            parse_grit_result.diagnostics()
        );
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        Path::new("quick_test.js"),
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
    )
    .expect("could not construct query");

    if !query.diagnostics.is_empty() {
        println!("Diagnostics from compiling query:\n{:?}", query.diagnostics);
    }

    let file = GritTargetFile {
        path: "test.js".into(),
        content: r#"
function hello() {
    console
        .log("hello");
}
"#
        .to_string(),
    };
    let results = query.execute(file).expect("could not execute query");

    println!("Results: {results:?}");
}
