use super::*;
use crate::workspace::ScanKind;
use biome_fs::MemoryFileSystem;
use biome_rowan::TextSize;
use crossbeam::channel::bounded;
use tokio::sync::watch;

#[test]
fn commonjs_file_rejects_import_statement() {
    const FILE_CONTENT: &[u8] = b"import 'foo';";
    const MANIFEST_CONTENT: &[u8] = b"{ \"type\": \"commonjs\" }";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/package.json"), MANIFEST_CONTENT);

    let (watcher_tx, _) = bounded(0);
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let result = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/"),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key: result.project_key,
            path: Some(BiomePath::new("/")),
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
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
fn store_embedded_nodes_with_corrent_ranges() {
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

    let (watcher_tx, _) = bounded(0);
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Arc::new(fs), watcher_tx, service_data_tx, None);
    let result = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/"),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key: result.project_key,
            path: Some(BiomePath::new("/")),
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    let documents = workspace.documents.pin();
    let document = documents.get(&Utf8PathBuf::from("/project/file.html"));

    assert!(document.is_some());

    let document = document.unwrap();
    assert_eq!(document._embedded_scripts.len(), 1);
    assert_eq!(document._embedded_styles.len(), 1);

    let script = document._embedded_scripts.first().unwrap();
    let style = document._embedded_styles.first().unwrap();

    let script_node = script.node();
    assert!(script_node.text_range_with_trivia().start() > TextSize::from(0));

    let style_node = style.node();
    assert!(style_node.text_range_with_trivia().start() > TextSize::from(0));
}
