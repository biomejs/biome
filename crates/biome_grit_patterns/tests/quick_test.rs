use biome_grit_parser::parse_grit;
use biome_grit_patterns::{
    GritQuery, GritQueryResult, GritTargetFile, GritTargetLanguage, JsTargetLanguage,
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;

// Test explicit file pattern with $filename binding - this SHOULD work
#[test]
fn test_explicit_file_pattern() {
    let parse_grit_result = parse_grit(
        r#"file(name=$filename, body=contains `console.log($message)`) where {
            $filename <: r".*\.ts$"
        }"#,
    );
    if !parse_grit_result.diagnostics().is_empty() {
        panic!("Cannot parse query:\n{:?}", parse_grit_result.diagnostics());
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
        Vec::new(),
    )
    .expect("could not construct query");

    let body = r#"console.log("Hello");"#;
    let parsed = parse(body, JsFileSource::ts(), JsParserOptions::default());

    let file = GritTargetFile::new("test.ts", parsed.into());
    let GritQueryResult { effects, logs, .. } =
        query.execute(file).expect("could not execute query");

    dbg!(&logs);
    dbg!(&effects);

    // Should have a match since the filename ends with .ts
    assert!(
        !effects.is_empty(),
        "Expected matches with explicit file pattern"
    );
}

// Test that $filename is properly bound in where clauses (auto-wrapped)
// This tests the pattern that gets auto-wrapped by the query compiler
#[test]
fn test_filename_binding() {
    // This is the original failing pattern - simple code snippet with where clause
    let parse_grit_result = parse_grit(
        r#"`console.log($message)` where {
            $filename <: r".*\.ts$"
        }"#,
    );
    if !parse_grit_result.diagnostics().is_empty() {
        panic!("Cannot parse query:\n{:?}", parse_grit_result.diagnostics());
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
        Vec::new(),
    )
    .expect("could not construct query");

    println!("Compiled pattern: {:?}", query.pattern);

    let body = r#"console.log("Hello");"#;
    let parsed = parse(body, JsFileSource::ts(), JsParserOptions::default());

    let file = GritTargetFile::new("test.ts", parsed.into());
    let GritQueryResult { effects, logs, .. } =
        query.execute(file).expect("could not execute query");

    dbg!(&logs);
    dbg!(&effects);

    // Should have a match since the filename ends with .ts
    assert!(
        !effects.is_empty(),
        "Expected matches since filename is test.ts"
    );
}

// Use this test to quickly execute a Grit query against a source snippet.
#[ignore]
#[test]
fn test_query() {
    let parse_grit_result = parse_grit(
        "
        `import $what from $where`
        ",
    );
    if !parse_grit_result.diagnostics().is_empty() {
        panic!("Cannot parse query:\n{:?}", parse_grit_result.diagnostics());
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
        Vec::new(),
    )
    .expect("could not construct query");

    if !query.diagnostics.is_empty() {
        println!("Diagnostics from compiling query:\n{:?}", query.diagnostics);
    }

    let body = r#"import { PrismaClient } from "@prisma/client/runtime";"#;

    let parsed = parse(body, JsFileSource::js_module(), JsParserOptions::default());

    let file = GritTargetFile::new("test.js", parsed.into());
    let GritQueryResult { effects, logs, .. } =
        query.execute(file).expect("could not execute query");

    println!("Effects: {effects:#?}");

    if !logs.is_empty() {
        println!(
            "\n## Logs\n\n{}",
            logs.iter()
                .map(|log| format!(
                    "Message: {}Syntax: {}",
                    log.message,
                    log.syntax_tree.as_deref().unwrap_or_default()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
