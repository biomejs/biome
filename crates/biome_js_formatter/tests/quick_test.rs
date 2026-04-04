use biome_formatter::TrailingNewline;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_service::workspace::{
    ChangeFileParams, DocumentFileSource, FileContent, FormatFileParams, GetFormatterIRParams,
    OpenFileParams, OpenProjectParams, server,
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

#[test]
fn test_trailing_newline_enabled() {
    let src = r#"const a = 1;"#;
    let source_type = JsFileSource::js_module();
    let tree = parse(src, source_type, JsParserOptions::default());
    let options =
        JsFormatOptions::new(source_type).with_trailing_newline(TrailingNewline::from(true));

    let doc = format_node(options, &tree.syntax(), false).unwrap();
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

    let doc = format_node(options, &tree.syntax(), false).unwrap();
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
