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
