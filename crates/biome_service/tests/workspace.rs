#[cfg(test)]
mod test {
    use biome_analyze::RuleCategories;
    use biome_configuration::analyzer::{RuleGroup, RuleSelector};
    use biome_fs::BiomePath;
    use biome_js_syntax::{JsFileSource, TextSize};
    use biome_service::file_handlers::DocumentFileSource;
    use biome_service::workspace::{
        server, FileGuard, OpenFileParams, RegisterProjectFolderParams,
    };
    use biome_service::Workspace;
    fn create_server() -> Box<dyn Workspace> {
        let workspace = server();
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
        let workspace = create_server();

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
        let workspace = create_server();

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

        // ".jsonc" file allow trailing commas
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
        assert!(jsonc_file.format_file().is_ok());

        // well-known json-with-comments file allows comments
        let well_known_json_with_comments_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new(".eslintrc.json"),
                content: r#"{"a": 42}//comment"#.into(),
                version: 0,
                document_file_source: None,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_file.format_file().is_ok());

        // well-known json-with-comments file allows comments
        let well_known_json_with_comments_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("project/.vscode/settings.json"),
                content: r#"{"a": 42}//comment"#.into(),
                version: 0,
                document_file_source: None,
            },
        )
        .unwrap();
        assert!(well_known_json_with_comments_file.format_file().is_ok());

        // well-known json-with-comments file doesn't allow trailing commas
        let well_known_json_with_comments_file_with_trailing_commas = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("dir/.eslintrc.json"),
                content: r#"{"a": 42,}"#.into(),
                version: 0,
                document_file_source: None,
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

    #[test]
    fn correctly_parses_graphql_files() {
        let workspace = create_server();

        let graphql_file = FileGuard::open(
            workspace.as_ref(),
            OpenFileParams {
                path: BiomePath::new("file.graphql"),
                content: r#"type Query {
  me: User
}

type User {
  id: ID
  name: String
}"#
                .into(),
                version: 0,
                document_file_source: None,
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
                content: r#"query {
  member @deprecated(abc: 123)
}"#
                .into(),
                version: 0,
                document_file_source: None,
            },
        )
        .unwrap();
        let result = graphql_file.pull_diagnostics(
            RuleCategories::all(),
            10,
            vec![RuleSelector::Rule(
                RuleGroup::Nursery,
                "useDeprecatedReason",
            )],
            vec![],
        );
        assert!(result.is_ok());
        let diagnostics = result.unwrap().diagnostics;
        assert_eq!(diagnostics.len(), 1)
    }
}
