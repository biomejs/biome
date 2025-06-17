use super::*;
use crate::workspace::ScanKind;
use biome_fs::MemoryFileSystem;
use crossbeam::channel::bounded;
use tokio::sync::watch;

#[test]
fn commonjs_file_rejects_import_statement() {
    const FILE_CONTENT: &[u8] = b"import 'foo';";
    const MANIFEST_CONTENT: &[u8] = b"{ \"type\": \"commonjs\" }";

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/package.json"), MANIFEST_CONTENT);

    let (watcher_tx, _) = bounded(0);
    let (service_data_tx, _) = watch::channel(ServiceDataNotification::Updated);
    let workspace = WorkspaceServer::new(Box::new(fs), watcher_tx, service_data_tx, None);
    let result = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/"),
            open_uninitialized: true,
            skip_rules: None,
            only_rules: None,
        })
        .unwrap();

    workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key: result.project_key,
            path: Some(BiomePath::new("/")),
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
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
