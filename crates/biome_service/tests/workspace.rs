#[cfg(test)]
mod test {
    use std::num::NonZero;
    use std::path::PathBuf;

    use biome_analyze::RuleCategories;
    use biome_configuration::analyzer::{RuleGroup, RuleSelector};
    use biome_configuration::{PartialConfiguration, PartialFilesConfiguration};
    use biome_fs::{BiomePath, MemoryFileSystem};
    use biome_js_syntax::{JsFileSource, TextSize};
    use biome_service::file_handlers::DocumentFileSource;
    use biome_service::workspace::{
        server, CloseFileParams, FileContent, FileGuard, GetFileContentParams, GetSyntaxTreeParams,
        OpenFileParams, RegisterProjectFolderParams, UnregisterProjectFolderParams,
        UpdateSettingsParams,
    };
    use biome_service::{Workspace, WorkspaceError};

    fn create_server() -> Box<dyn Workspace> {
        let workspace = server(Box::new(MemoryFileSystem::default()));
        workspace
            .register_project_folder(RegisterProjectFolderParams {
                set_as_current_workspace: true,
                path: None,
            })
            .unwrap();

        workspace
    }

    #[test]
    fn debug_control_flow() {
        const SOURCE: &str = "function test () { return; }";
        const GRAPH: &str = "flowchart TB
    block_0[\"<b>block_0</b><br/>Return(JS_RETURN_STATEMENT 19..26)<br/>Return\"]\n\n";

        let workspace = create_server();
        let file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.js"),
                content: FileContent::FromClient(SOURCE.into()),
                version: 0,
                document_file_source: Some(DocumentFileSource::from(JsFileSource::default())),
                persist_node_cache: false,
            },
        )
        .unwrap();

        let cfg = file.get_control_flow_graph(TextSize::from(20)).unwrap();

        assert_eq!(cfg, GRAPH);
    }

    #[test]
    fn recognize_typescript_definition_file() {
        let workspace = create_server();

        let file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.d.ts"),
                // the following code snippet can be correctly parsed in .d.ts file but not in .ts file
                content: FileContent::FromClient("export const foo: number".into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();

        assert!(file.format_file().is_ok());
    }

    #[test]
    fn correctly_handle_json_files() {
        let workspace = create_server();

        // ".json" file
        let json_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("a.json"),
                content: FileContent::FromClient(r#"{"a": 42}"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(json_file.format_file().is_ok());

        // ".json" file doesn't allow comments
        let json_file_with_comments = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("b.json"),
                content: FileContent::FromClient(r#"{"a": 42}//comment"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(json_file_with_comments.format_file().is_err());

        // ".json" file doesn't allow trailing commas
        let json_file_with_trailing_commas = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("c.json"),
                content: FileContent::FromClient(r#"{"a": 42,}"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(json_file_with_trailing_commas.format_file().is_err());

        // ".jsonc" file allows comments
        let jsonc_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("d.jsonc"),
                content: FileContent::FromClient(r#"{"a": 42}//comment"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(jsonc_file.format_file().is_ok());

        // ".jsonc" file allow trailing commas
        let jsonc_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("e.jsonc"),
                content: FileContent::FromClient(r#"{"a": 42,}"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(jsonc_file.format_file().is_ok());

        // well-known json-with-comments file allows comments
        let well_known_json_with_comments_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new(".eslintrc.json"),
                content: FileContent::FromClient(r#"{"a": 42}//comment"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_file.format_file().is_ok());

        // well-known json-with-comments file allows comments
        let well_known_json_with_comments_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("project/.vscode/settings.json"),
                content: FileContent::FromClient(r#"{"a": 42}//comment"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_file.format_file().is_ok());

        // well-known json-with-comments file doesn't allow trailing commas
        let well_known_json_with_comments_file_with_trailing_commas = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("dir/.eslintrc.json"),
                content: FileContent::FromClient(r#"{"a": 42,}"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_file_with_trailing_commas
            .format_file()
            .is_err());

        // well-known json-with-comments-and-trailing-commas file allows comments and trailing commas
        let well_known_json_with_comments_and_trailing_commas_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("tsconfig.json"),
                content: FileContent::FromClient(r#"{"a": 42,}//comment"#.into()),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_and_trailing_commas_file
            .format_file()
            .is_ok());
    }

    #[test]
    fn correctly_parses_graphql_files() {
        let workspace = create_server();

        let graphql_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.graphql"),
                content: FileContent::FromClient(
                    r#"type Query {
  me: User
}

type User {
  id: ID
  name: String
}"#
                    .into(),
                ),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        let result = graphql_file.get_syntax_tree();
        assert!(result.is_ok());
        let syntax = result.unwrap().ast;

        assert!(syntax.starts_with("GraphqlRoot"))
    }

    #[test]
    fn correctly_pulls_lint_diagnostics() {
        let workspace = create_server();

        let graphql_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.graphql"),
                content: FileContent::FromClient(
                    r#"query {
  member @deprecated(abc: 123)
}"#
                    .into(),
                ),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        let result = graphql_file.pull_diagnostics(
            RuleCategories::all(),
            10,
            vec![RuleSelector::Rule(
                RuleGroup::Nursery.as_str(),
                "useDeprecatedReason",
            )],
            vec![],
        );
        assert!(result.is_ok());
        let diagnostics = result.unwrap().diagnostics;
        assert_eq!(diagnostics.len(), 1)
    }

    #[test]
    fn pull_grit_debug_info() {
        let workspace = create_server();

        let grit_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.grit"),
                content: FileContent::FromClient(
                    r#"`function ($args) { $body }` where {
  $args <: contains `x`
}"#
                    .into(),
                ),
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            },
        )
        .unwrap();
        let result = grit_file.get_syntax_tree();
        assert!(result.is_ok());
        let syntax = result.unwrap().ast;

        assert!(syntax.starts_with("GritRoot"))
    }

    #[test]
    fn files_loaded_by_the_scanner_are_only_unloaded_when_the_project_is_unregistered() {
        const FILE_A_CONTENT: &[u8] = b"import { bar } from './b.ts';\nfunction foo() {}";
        const FILE_B_CONTENT: &[u8] = b"import { foo } from './a.ts';\nfunction bar() {}";

        let mut fs = MemoryFileSystem::default();
        fs.insert(PathBuf::from("/project/a.ts"), FILE_A_CONTENT);
        fs.insert(PathBuf::from("/project/b.ts"), FILE_B_CONTENT);

        let workspace = server(Box::new(fs));
        workspace
            .register_project_folder(RegisterProjectFolderParams {
                set_as_current_workspace: true,
                path: Some(PathBuf::from("/project")),
            })
            .unwrap();

        workspace.scan_current_project_folder(()).unwrap();

        macro_rules! assert_file_a_content {
            () => {
                assert_eq!(
                    workspace
                        .get_file_content(GetFileContentParams {
                            path: BiomePath::new("/project/a.ts"),
                        })
                        .unwrap(),
                    String::from_utf8(FILE_A_CONTENT.to_vec()).unwrap(),
                );
            };
        }

        assert_file_a_content!();

        workspace
            .open_file(OpenFileParams {
                path: BiomePath::new("/project/a.ts"),
                content: FileContent::FromServer,
                version: 0,
                document_file_source: None,
                persist_node_cache: false,
            })
            .unwrap();

        assert_file_a_content!();

        workspace
            .close_file(CloseFileParams {
                path: BiomePath::new("/project/a.ts"),
            })
            .unwrap();

        assert_file_a_content!();

        workspace
            .unregister_project_folder(UnregisterProjectFolderParams {
                path: PathBuf::from("/project"),
            })
            .unwrap();

        assert!(workspace
            .get_file_content(GetFileContentParams {
                path: BiomePath::new("/project/a.ts"),
            })
            .is_err_and(|error| matches!(error, WorkspaceError::NotFound(_))));
    }

    #[test]
    fn too_large_files_are_tracked_but_not_parsed() {
        const FILE_CONTENT: &[u8] = b"console.log(`I'm YUUUGE!`);";

        let mut fs = MemoryFileSystem::default();
        fs.insert(PathBuf::from("/project/a.ts"), FILE_CONTENT);

        let workspace = server(Box::new(fs));
        workspace
            .register_project_folder(RegisterProjectFolderParams {
                set_as_current_workspace: true,
                path: Some(PathBuf::from("/project")),
            })
            .unwrap();

        workspace
            .update_settings(UpdateSettingsParams {
                configuration: PartialConfiguration {
                    files: Some(PartialFilesConfiguration {
                        max_size: NonZero::new(10),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                vcs_base_path: None,
                gitignore_matches: Vec::new(),
                workspace_directory: None,
            })
            .unwrap();

        workspace.scan_current_project_folder(()).unwrap();

        assert!(workspace
            .get_syntax_tree(GetSyntaxTreeParams {
                path: BiomePath::new("/project/a.ts"),
            })
            .is_err_and(|error| matches!(error, WorkspaceError::FileIgnored(_))));
    }
}
