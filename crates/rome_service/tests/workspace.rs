use biome_fs::RomePath;
use biome_js_syntax::TextSize;
use rome_service::workspace::{server, FileGuard, Language, OpenFileParams};

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
            path: RomePath::new("file.js"),
            content: SOURCE.into(),
            version: 0,
            language_hint: Language::JavaScript,
        },
    )
    .unwrap();

    let cfg = file.get_control_flow_graph(TextSize::from(20)).unwrap();

    assert_eq!(cfg, GRAPH);
}
