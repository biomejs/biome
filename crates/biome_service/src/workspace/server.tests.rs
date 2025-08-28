use biome_fs::MemoryFileSystem;
use biome_rowan::TextSize;

use crate::test_utils::setup_workspace_and_open_project;

use super::*;

#[test]
fn commonjs_file_rejects_import_statement() {
    const FILE_CONTENT: &[u8] = b"import 'foo';";
    const MANIFEST_CONTENT: &[u8] = b"{ \"type\": \"commonjs\" }";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/package.json"), MANIFEST_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    match workspace.get_parse("/project/a.js".into()) {
        Ok(parse) => {
            insta::assert_debug_snapshot!(parse.diagnostics(), @r###"
            [
                ParseDiagnostic {
                    span: Some(
                        0..13,
                    ),
                    message: Illegal use of an import declaration outside of a module,
                    advice: ParserAdvice {
                        advice_list: [
                            Hint(
                                "not allowed inside scripts",
                            ),
                        ],
                    },
                },
            ]
            "###);
        }
        Err(error) => panic!("File not available: {error}"),
    }
}

#[test]
fn store_embedded_nodes_with_current_ranges() {
    const FILE_CONTENT: &str = r#"<html>
    <head>
        <style>
            .#id {}
        </style>
        <script>
            const foo = "bar";
        </script>
    </head>
</html>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/file.html"), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/file.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    let documents = workspace.documents.pin();
    let document = documents.get(&Utf8PathBuf::from("/project/file.html"));

    assert!(document.is_some());

    let document = document.unwrap();
    assert_eq!(document.embedded_scripts.len(), 1);
    assert_eq!(document.embedded_styles.len(), 1);

    let script = document.embedded_scripts.first().unwrap();
    let style = document.embedded_styles.first().unwrap();

    let script_node = script.node();
    assert!(script_node.text_range_with_trivia().start() > TextSize::from(0));

    let style_node = style.node();
    assert!(style_node.text_range_with_trivia().start() > TextSize::from(0));
}

#[test]
fn format_html_with_scripts_and_css() {
    const FILE_CONTENT: &str = r#"<html>
    <head>
        <style>
                            .#id {}
        </style>
        <script>
                            const foo = "bar";
        </script>
    </head>
</html>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/file.html"), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/file.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            path: Utf8PathBuf::from("/project/file.html").into(),
            project_key,
        })
        .unwrap();

    let documents = workspace.documents.pin();
    let document = documents.get(&Utf8PathBuf::from("/project/file.html"));

    assert!(document.is_some());

    let document = document.unwrap();

    let script = document.embedded_scripts.first().unwrap();
    let style = document.embedded_styles.first().unwrap();

    let script_node = script.node();
    dbg!(script_node.text_range_with_trivia());

    let style_node = style.node();
    dbg!(style_node.text_range_with_trivia());
    eprintln!("{}", result.as_code());
}
