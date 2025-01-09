use biome_fs::MemoryFileSystem;

use super::*;

#[test]
fn commonjs_file_rejects_import_statement() {
    const FILE_CONTENT: &[u8] = b"import 'foo';";
    const MANIFEST_CONTENT: &[u8] = b"{ \"type\": \"commonjs\" }";

    let mut fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/package.json"), MANIFEST_CONTENT);

    let workspace = WorkspaceServer::new(Box::new(fs));
    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/"),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .scan_project_folder(ScanProjectFolderParams {
            project_key,
            path: Some(BiomePath::new("/")),
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
