use biome_formatter::TrailingNewline;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_parser::{JsParserOptions, parse};
use biome_languages::{DocumentFileSource, JsFileSource};
use biome_service::workspace::{
    ChangeFileParams, FileContent, FormatFileParams, GetFormatterIRParams, OpenFileParams,
    OpenProjectParams, server,
};
use std::sync::Arc;

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
const c = [
  , /* this */
];
    "#;
    let source_type = JsFileSource::tsx();

    let fs = MemoryFileSystem::default();
    let workspace = server(Arc::new(fs), None);

    let project = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(""),
            open_uninitialized: true,
        })
        .unwrap();

    let path = BiomePath::new("test.tsx");
    workspace
        .open_file(OpenFileParams {
            project_key: project.project_key,
            path: path.clone(),
            content: FileContent::FromClient {
                content: src.to_string(),
                version: 0,
            },
            document_file_source: Some(DocumentFileSource::from(source_type)),
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    // Print IR
    if let Ok(ir) = workspace.get_formatter_ir(GetFormatterIRParams {
        project_key: project.project_key,
        path: path.clone(),
    }) {
        println!("{ir}");
    }

    let printed = workspace
        .format_file(FormatFileParams {
            project_key: project.project_key,
            path: path.clone(),
            inline_config: None,
        })
        .unwrap();

    eprintln!("{}", printed.as_code());

    // Idempotency check
    workspace
        .change_file(ChangeFileParams {
            project_key: project.project_key,
            path: path.clone(),
            content: printed.as_code().to_string(),
            version: 1,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let re_printed = workspace
        .format_file(FormatFileParams {
            project_key: project.project_key,
            path: path.clone(),
            inline_config: None,
        })
        .unwrap();

    similar_asserts::assert_eq!(
        re_printed.as_code(),
        printed.as_code(),
        "Formatter is not idempotent"
    );
}

#[ignore]
#[test]
// temporary repro for biome-zed#164 — mimics workspace format_on_type
fn on_type_repro() {
    use biome_js_formatter::format_sub_tree;
    use biome_rowan::TokenAtOffset;

    // Valid code: the user just typed the `}` closing the inner function body.
    let src = "class A {\n  foo() {\n    return 1;\n  }\n}\n";
    // offset right after the inner `}` (line 3 `  }`)
    let offset =
        biome_rowan::TextSize::from(u32::try_from(src.find("  }").unwrap() + "  }".len()).unwrap());

    let source_type = JsFileSource::ts();
    let parsed = parse(src, source_type, JsParserOptions::default());
    let tree = parsed.syntax();
    eprintln!("has errors: {}", parsed.has_errors());

    let token = match tree.token_at_offset(offset) {
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(token, _) => token,
    };
    eprintln!("token at offset: {:?} {:?}", token.kind(), token.text());
    let root_node = token.parent().unwrap();
    eprintln!(
        "formatting subtree: {:?} range {:?}",
        root_node.kind(),
        root_node.text_range_with_trivia()
    );

    let printed = format_sub_tree(JsFormatOptions::new(source_type), &root_node).unwrap();
    eprintln!("printed range: {:?}", printed.range());
    eprintln!("--- input slice ---");
    eprintln!("{}", &src[printed.range().unwrap()]);
    eprintln!("--- formatted ---");
    eprintln!("{}", printed.as_code());

    // Alternative: what format_range would produce for the same spot
    let ranged = biome_js_formatter::format_range(
        JsFormatOptions::new(source_type),
        &tree,
        root_node.text_trimmed_range(),
    )
    .unwrap();
    eprintln!("format_range printed range: {:?}", ranged.range());
    eprintln!("--- format_range input slice ---");
    eprintln!("{}", &src[ranged.range().unwrap()]);
    eprintln!("--- format_range formatted ---");
    eprintln!("{}", ranged.as_code());
}

#[test]
fn test_trailing_newline_enabled() {
    let src = r#"const a = 1;"#;
    let source_type = JsFileSource::js_module();
    let tree = parse(src, source_type, JsParserOptions::default());
    let options =
        JsFormatOptions::new(source_type).with_trailing_newline(TrailingNewline::from(true));

    let doc = format_node(options, &tree.syntax(), Vec::new()).unwrap();
    let result = doc.print().unwrap();

    // With trailing newline enabled (default), should end with newline
    assert!(
        result.as_code().ends_with('\n'),
        "Expected code to end with newline"
    );
}

#[test]
fn test_trailing_newline_disabled() {
    let src = r#"const a = 1;"#;
    let source_type = JsFileSource::js_module();
    let tree = parse(src, source_type, JsParserOptions::default());
    let options =
        JsFormatOptions::new(source_type).with_trailing_newline(TrailingNewline::from(false));

    let doc = format_node(options, &tree.syntax(), Vec::new()).unwrap();
    let result = doc.print().unwrap();

    // With trailing newline disabled, should NOT end with newline
    assert!(
        !result.as_code().ends_with('\n'),
        "Expected code to NOT end with newline"
    );
    assert!(
        !result.as_code().ends_with('\r'),
        "Expected code to NOT end with carriage return"
    );
}
