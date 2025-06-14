use biome_grit_parser::parse_grit;
use biome_grit_patterns::{
    GritQuery, GritQueryResult, GritTargetFile, GritTargetLanguage, JsTargetLanguage,
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;

fn main() {
    // Test our import pattern fix
    let parse_grit_result = parse_grit(
        r#"language js

import $imports from "foo""#,
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

    // Test different import types
    let test_cases = vec![
        ("Default import", r#"import bar from "foo";"#),
        ("Named import", r#"import { baz } from "foo";"#),
        ("Multiple named imports", r#"import { qux, quux } from "foo";"#),
        ("Namespace import", r#"import * as foobar from "foo";"#),
        ("Combined import", r#"import defaultExport, { namedExport } from "foo";"#),
        ("Bare import", r#"import "foo";"#),
        ("Different source (should not match)", r#"import something from "different";"#),
    ];

    for (description, code) in test_cases {
        println!("\nTesting: {}", description);
        println!("Code: {}", code);
        
        let file = GritTargetFile::new(
            "test.js",
            parse(code, JsFileSource::tsx(), JsParserOptions::default()).into(),
        );
        
        let GritQueryResult { effects, logs, .. } =
            query.execute(file).expect("could not execute query");

        if effects.is_empty() {
            println!("❌ No matches found");
        } else {
            println!("✅ Found {} match(es)", effects.len());
            for effect in effects {
                println!("  Effect: {:#?}", effect);
            }
        }

        if !logs.is_empty() {
            println!("Logs: {:#?}", logs);
        }
    }
} 