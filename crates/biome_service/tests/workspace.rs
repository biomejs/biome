use biome_fs::BiomePath;
use biome_js_syntax::{JsFileSource, TextSize};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::workspace::{server, FileGuard, OpenFileParams};

#[test]
fn debug_control_flow() {
    const SOURCE: &str = "function test () { return; }";
    const GRAPH: &str = "flowchart TB
    block_0[\"<b>block_0</b><br/>Return(JS_RETURN_STATEMENT 19..26)<br/>Return\"]

";

    let workspace = server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("file.js"),
            content: SOURCE.into(),
            version: 0,
            document_file_source: Some(DocumentFileSource::from(JsFileSource::default())),
        },
    )
    .unwrap();

    let cfg = file.get_control_flow_graph(TextSize::from(20)).unwrap();

    assert_eq!(cfg, GRAPH);
}

#[test]
fn recognize_typescript_definition_file() {
    let workspace = server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("file.d.ts"),
            // the following code snippet can be correctly parsed in .d.ts file but not in .ts file
            content: "export const foo: number".into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();

    assert!(file.format_file().is_ok());
}

#[test]
fn correctly_handle_json_files() {
    let workspace = server();

    // ".json" file
    let json_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("a.json"),
            content: r#"{"a": 42}"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(json_file.format_file().is_ok());

    // ".json" file doesn't allow comments
    let json_file_with_comments = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("b.json"),
            content: r#"{"a": 42}//comment"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(json_file_with_comments.format_file().is_err());

    // ".json" file doesn't allow trailing commas
    let json_file_with_trailing_commas = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("c.json"),
            content: r#"{"a": 42,}"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(json_file_with_trailing_commas.format_file().is_err());

    // ".jsonc" file allows comments
    let jsonc_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("d.jsonc"),
            content: r#"{"a": 42}//comment"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(jsonc_file.format_file().is_ok());

    // ".jsonc" file doesn't allow trailing commas
    let jsonc_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("e.jsonc"),
            content: r#"{"a": 42,}"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(jsonc_file.format_file().is_err());

    // well-known json-with-comments-and-trailing-commas file allows comments and trailing commas
    let well_known_json_with_comments_and_trailing_commas_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            path: BiomePath::new("tsconfig.json"),
            content: r#"{"a": 42,}//comment"#.into(),
            version: 0,
            document_file_source: None,
        },
    )
    .unwrap();
    assert!(well_known_json_with_comments_and_trailing_commas_file
        .format_file()
        .is_ok());
}
