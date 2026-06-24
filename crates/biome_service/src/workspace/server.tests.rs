use super::*;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::UpdateSettingsParams;
use biome_configuration::{
    FormatterConfiguration, JsConfiguration,
    analyzer::AnalyzerSelector,
    javascript::{JsFormatterConfiguration, JsParserConfiguration, JsResolverConfiguration},
};
use biome_formatter::{IndentStyle, LineWidth};
use biome_fs::MemoryFileSystem;
use biome_rowan::TextSize;
use camino::Utf8Path;
use std::str::FromStr;

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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    match workspace.get_parse("/project/a.js".into()) {
        Ok(parse) => {
            insta::assert_debug_snapshot!(parse.diagnostics(), @r#"
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
                    advice_offset: None,
                },
            ]
            "#);
        }
        Err(error) => panic!("File not available: {error}"),
    }
}

#[test]
fn pnpm_workspace_update_reapplies_catalogs() {
    const PACKAGE_JSON: &[u8] = br#"{
  "name": "app",
  "dependencies": {
    "react": "catalog:react19"
  }
}"#;
    const WORKSPACE_V1: &[u8] = br#"catalogs:
  react19:
    react: 19.0.0
"#;
    const WORKSPACE_V2: &[u8] = br#"catalogs:
  react19:
    react: 18.3.1
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/package.json"), PACKAGE_JSON);
    fs.insert(
        Utf8PathBuf::from("/project/pnpm-workspace.yaml"),
        WORKSPACE_V1,
    );

    let fs_for_updates = MemoryFileSystem::from_files(fs.files.0.clone());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    resolver: Some(JsResolverConfiguration {
                        experimental_pnpm_catalogs: Some(Bool(true)),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::Project,
            verbose: false,
        })
        .unwrap();

    let package_manifest = workspace
        .project_layout
        .get_node_manifest_for_package(Utf8Path::new("/project"))
        .expect("package manifest should be indexed");
    let initial_react = package_manifest
        .catalog
        .as_ref()
        .and_then(|catalogs| catalogs.named.get("react19"))
        .and_then(|dependencies| dependencies.get("react"));
    assert_eq!(initial_react, Some("19.0.0"));

    fs_for_updates.insert(
        Utf8PathBuf::from("/project/pnpm-workspace.yaml"),
        WORKSPACE_V2,
    );

    workspace
        .open_file_internal(
            OpenFileReason::Index(IndexTrigger::Update),
            OpenFileParams {
                project_key,
                path: BiomePath::new("/project/pnpm-workspace.yaml"),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            },
        )
        .unwrap();

    let package_manifest = workspace
        .project_layout
        .get_node_manifest_for_package(Utf8Path::new("/project"))
        .expect("package manifest should be indexed");
    let updated_react = package_manifest
        .catalog
        .as_ref()
        .and_then(|catalogs| catalogs.named.get("react19"))
        .and_then(|dependencies| dependencies.get("react"));
    assert_eq!(updated_react, Some("18.3.1"));
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let documents = workspace.documents.pin();
    let document = documents.get(&Utf8PathBuf::from("/project/file.html"));

    assert!(document.is_some());

    let document = document.unwrap();
    let scripts: Vec<_> = document
        .embedded_snippets
        .iter()
        .filter_map(|node| node.as_js_embedded_snippet())
        .collect();
    let styles: Vec<_> = document
        .embedded_snippets
        .iter()
        .filter_map(|node| node.as_css_embedded_snippet())
        .collect();
    assert_eq!(scripts.len(), 1);
    assert_eq!(styles.len(), 1);

    let script = scripts.first().unwrap();
    let style = styles.first().unwrap();

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
            #id { background-color: red; }
        </style>
        <script type="importmap">
            { "imports":{"circle": "https://example.com/shapes/circle.js","square":"./modules/shapes/square.js"} }
        </script>
        <script>
            const foo = "bar";
            function bar() { const object = { ["literal"]: "SOME OTHER STRING" }; return 1; }
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            path: Utf8PathBuf::from("/project/file.html").into(),
            project_key,
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r#"
    <html>
    	<head>
    		<style>
    		#id {
    			background-color: red;
    		}
    		</style>
    		<script type="importmap">
    		{
    			"imports": {
    				"circle": "https://example.com/shapes/circle.js",
    				"square": "./modules/shapes/square.js"
    			}
    		}
    		</script>
    		<script>
    		const foo = "bar";
    		function bar() {
    			const object = { ["literal"]: "SOME OTHER STRING" };
    			return 1;
    		}
    		</script>
    	</head>
    </html>
    "#);
}

#[test]
fn format_html_preserves_template_literal_and_block_comment_indentation() {
    // Regression: re-formatting an HTML file whose embedded <script> contains a
    // template literal or whose <style> contains a block comment must not gain
    // extra indentation on each run.
    const FILE_CONTENT: &str = r#"<html>
    <head>
        <script>
            const sql = `
                SELECT *
                FROM users
            `;
        </script>
        <style>
            /*
             * A block comment.
             */
            .foo {
                color: red;
            }
        </style>
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let first = workspace
        .format_file(FormatFileParams {
            path: Utf8PathBuf::from("/project/file.html").into(),
            project_key,
            inline_config: None,
        })
        .unwrap();

    workspace
        .change_file(ChangeFileParams {
            project_key,
            path: BiomePath::new("/project/file.html"),
            content: first.as_code().to_string(),
            version: 1,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let second = workspace
        .format_file(FormatFileParams {
            path: Utf8PathBuf::from("/project/file.html").into(),
            project_key,
            inline_config: None,
        })
        .unwrap();

    assert_eq!(
        first.as_code(),
        second.as_code(),
        "format_file must be idempotent for template literals and block comments"
    );
}

#[test]
fn jsx_everywhere_sets_correct_variant() {
    const TS_FILE_CONTENT: &[u8] = br"
const f = <T1>(arg1: T1) => <T2>(arg2: T2) => {
    return { arg1, arg2 };
}
    ";
    const JS_FILE_CONTENT: &[u8] = br"
function Foo({cond}) {
  return cond ? (
    <True />
  ) : (
    <False />
  );
}
    ";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.ts"), TS_FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/a.js"), JS_FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    let js_conf = JsConfiguration {
        parser: Some(JsParserConfiguration {
            jsx_everywhere: Some(Bool(true)),
            ..Default::default()
        }),
        formatter: Some(JsFormatterConfiguration {
            line_width: Some(LineWidth::try_from(30).unwrap()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let configuration = Configuration {
        javascript: Some(js_conf),
        formatter: Some(FormatterConfiguration {
            indent_style: Some(IndentStyle::Space),
            ..Default::default()
        }),
        ..Default::default()
    };

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/project")),
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

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
            path: BiomePath::new("/project/a.ts"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.js"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let ts_file_source = workspace.get_file_source("/project/a.ts".into(), false);
    let ts = ts_file_source.to_js_file_source().expect("JS file source");
    assert!(ts.is_typescript());
    assert!(!ts.is_jsx());
    match workspace.get_parse("/project/a.ts".into()) {
        Ok(parse) => assert_eq!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }

    let js_file_source = workspace.get_file_source("/project/a.js".into(), false);
    let js = js_file_source.to_js_file_source().expect("JS file source");
    assert!(!js.is_typescript());
    assert!(js.is_jsx());
    match workspace.get_parse("/project/a.js".into()) {
        Ok(parse) => assert_eq!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }
    match workspace.format_file(FormatFileParams {
        project_key,
        path: BiomePath::new("/project/a.js"),
        inline_config: None,
    }) {
        Ok(printed) => {
            insta::assert_snapshot!(printed.as_code(), @r###"
            function Foo({ cond }) {
              return cond ? (
                <True />
              ) : (
                <False />
              );
            }
            "###);
        }
        Err(error) => panic!("File not formatted: {error}"),
    }
}

#[test]
fn jsx_everywhere_disabled_correct_variant() {
    const JS_FILE_CONTENT: &[u8] = br"
function Foo({cond}) {
  return cond ? (
    <True />
  ) : (
    <False />
  );
}
    ";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.js"), JS_FILE_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/a.jsx"), JS_FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    let js_conf = JsConfiguration {
        parser: Some(JsParserConfiguration {
            jsx_everywhere: Some(Bool(false)),
            ..Default::default()
        }),
        formatter: Some(JsFormatterConfiguration {
            line_width: Some(LineWidth::try_from(30).unwrap()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let configuration = Configuration {
        javascript: Some(js_conf),
        formatter: Some(FormatterConfiguration {
            indent_style: Some(IndentStyle::Space),
            ..Default::default()
        }),
        ..Default::default()
    };

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/project")),
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.jsx"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let js_file_source = workspace.get_file_source("/project/a.js".into(), false);
    let js = js_file_source.to_js_file_source().expect("JS file source");
    assert!(!js.is_typescript());
    assert!(!js.is_jsx());
    match workspace.get_parse("/project/a.js".into()) {
        Ok(parse) => assert_ne!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }

    let jsx_file_source = workspace.get_file_source("/project/a.jsx".into(), false);
    let jsx = jsx_file_source.to_js_file_source().expect("JS file source");
    assert!(!jsx.is_typescript());
    assert!(jsx.is_jsx());
    match workspace.get_parse("/project/a.jsx".into()) {
        Ok(parse) => assert_eq!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }
    match workspace.format_file(FormatFileParams {
        project_key,
        path: BiomePath::new("/project/a.jsx"),
        inline_config: None,
    }) {
        Ok(printed) => {
            insta::assert_snapshot!(printed.as_code(), @r###"
            function Foo({ cond }) {
              return cond ? (
                <True />
              ) : (
                <False />
              );
            }
            "###);
        }
        Err(error) => panic!("File not formatted: {error}"),
    }
}

#[test]
fn pull_diagnostics_and_actions_for_js_file() {
    const FILE_CONTENT: &[u8] = br#"debugger"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/file.js"), FILE_CONTENT);

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
            path: BiomePath::new("/project/file.js"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics_and_actions(PullDiagnosticsAndActionsParams {
            path: BiomePath::new("/project/file.js"),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            inline_config: None,
        })
        .unwrap();

    assert!(!result.diagnostics.is_empty(), "Should have diagnostics");
    assert_eq!(result.diagnostics.len(), 1, "Should have one diagnostic");
    assert_eq!(
        result.diagnostics[0].1.len(),
        3,
        "Should have three actions: fix, and two suppression actions"
    );

    insta::assert_debug_snapshot!(result)
}

/// Regression test for https://github.com/biomejs/biome/issues/9506 and
/// https://github.com/biomejs/biome/issues/9479.
///
/// `<script type="speculationrules">` and `<script type="application/ld+json">`
/// contain JSON-like content that is NOT JavaScript. Before this fix, biome's
/// embed registry fallback would treat these as JavaScript, causing false
/// parse errors and incorrect lint diagnostics.
#[test]
fn no_diagnostics_for_unsupported_script_types() {
    // speculationrules content is JSON-like but is NOT JavaScript.
    // application/ld+json content is JSON-LD, also not JavaScript.
    // Both should be silently skipped by the embed detector (no JS parse errors).
    const FILE_CONTENT: &str = r#"<!doctype html>
<html>
  <head>
    <script type="speculationrules">
      {
        "prerender": [
          { "source": "list", "urls": ["/next-page"] }
        ]
      }
    </script>
    <script type="application/ld+json">
      {
        "@context": "https://schema.org",
        "@type": "Article",
        "headline": "Test"
      }
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics_and_actions(PullDiagnosticsAndActionsParams {
            path: BiomePath::new("/project/file.html"),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            inline_config: None,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics for unsupported script types, got: {:#?}",
        result.diagnostics
    );
}

/// Regression test for https://github.com/biomejs/biome/issues/9140.
///
/// Astro allows JSX-style attribute shorthand: `<div {prop} />` is sugar for
/// `<div prop={prop} />`. The HTML/Astro parser forwards JSX-bearing template
/// expressions to the JS parser with `EmbeddingKind::Astro { frontmatter: false }`,
/// and the JS parser must accept the shorthand only in that embedding context.
/// In a regular `.jsx` file the same syntax remains a parse error (covered by
/// `crates/biome_js_parser/tests/js_test_suite/error/jsx_shorthand_attribute_outside_astro.jsx`).
#[test]
fn astro_jsx_shorthand_attribute() {
    const FILE_CONTENT: &str = r#"---
const items = ['a', 'b'];
---
<ul>
  {items.map((item) => <li {item}>row</li>)}
</ul>
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/file.astro"), FILE_CONTENT);

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
            path: BiomePath::new("/project/file.astro"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics_and_actions(PullDiagnosticsAndActionsParams {
            path: BiomePath::new("/project/file.astro"),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            inline_config: None,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics for Astro JSX shorthand attribute, got: {:#?}",
        result.diagnostics
    );
}

#[test]
fn format_js_with_embedded_css() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"const Foo = styled.div`
  display:
    flex;
  color : red ;
`;

const Bar = styled(Component)`
  display:
    flex;
  color : red ;
`;"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r"
    const Foo = styled.div`
    	display: flex;
    	color: red;
    `;

    const Bar = styled(Component)`
    	display: flex;
    	color: red;
    `;
    ");
}

#[test]
fn issue_9975() {
    const FILE_PATH: &str = "/project/file.ts";
    const FILE_CONTENT: &str = r#"styled.div`
  svg:first-of-type {
    margin-left: 0;
  }
`;

styled.div`
  div:not(:last-child) {
    border-bottom: 1px solid black;
  }
`;"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            categories: RuleCategories::default(),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert_eq!(result.parse_errors, 0);
    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics for styled nested selectors, got: {:#?}",
        result.diagnostics
    );
}

#[test]
fn issue_9625() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"const Portfolio = styled.div`
    display: flex;
  align-items: center;
`;

const PortfolioIcon = styled.div`
  ${({ theme }) => css``
  };
`;"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                formatter: Some(FormatterConfiguration {
                    indent_style: Some(IndentStyle::Space),
                    ..Default::default()
                }),
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r"
    const Portfolio = styled.div`
      display: flex;
      align-items: center;
    `;

    const PortfolioIcon = styled.div`
      ${({ theme }) => css``};
    `;
    ");
}

#[test]
fn issue_9994() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"styled.div`
  div:first-of-type {
    color: black;
  }
  background: black;
`;
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Error,
            enforce_assist: false,
        })
        .unwrap();

    assert!(
        diagnostics.diagnostics.is_empty(),
        "Expected no diagnostics for issue #9994, got: {:#?}",
        diagnostics.diagnostics
    );

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r#"
    styled.div`
    	div:first-of-type {
    		color: black;
    	}
    	background: black;
    `;
    "#);
}

#[test]
fn issue_9113() {
    const FILE_PATH: &str = "/project/file.ts";
    const FILE_CONTENT: &str = r#"import styled from 'styled-components';

const Wrapper = styled.div`
  height: 20px;

  @media screen and (min-width: 768px) {
    height: 40px;
  }
`;

const Container = styled.div`
	     	display: grid;
	grid-template-rows: auto;
	grid-gap: 2px;
	margin: 4px 4px 0;

    /* top level seems fine */
	grid-template-columns: repeat(3, 1fr);

    	  @media (min-width: 480px) {
    		    grid-template-columns: repeat(4, 1fr);
	}

	   @media (min-width: 640px) {
		  grid-template-columns: repeat(5, 1fr);
	}

    	@media (min-width: 780px) {
    		grid-template-columns: repeat(6, 1fr);
    	}
`;"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Error,
            enforce_assist: false,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics for embedded CSS, got: {:#?}",
        result.diagnostics
    );

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r#"
    import styled from "styled-components";

    const Wrapper = styled.div`
    	height: 20px;

    	@media screen and (min-width: 768px) {
    		height: 40px;
    	}
    `;

    const Container = styled.div`
    	display: grid;
    	grid-template-rows: auto;
    	grid-gap: 2px;
    	margin: 4px 4px 0;

    	/* top level seems fine */
    	grid-template-columns: repeat(3, 1fr);

    	@media (min-width: 480px) {
    		grid-template-columns: repeat(4, 1fr);
    	}

    	@media (min-width: 640px) {
    		grid-template-columns: repeat(5, 1fr);
    	}

    	@media (min-width: 780px) {
    		grid-template-columns: repeat(6, 1fr);
    	}
    `;
    "#);
}

#[test]
fn format_js_with_embedded_graphql() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"const Foo = gql`
  query PeopleCount {
  people(
       id: $peopleId){
       totalCount
       }}
`;

const Bar = graphql(`
  query PeopleCount {
  people(
       id: $peopleId){
       totalCount
       }}
`);

const Baz = graphql`
  query PeopleCount {
  people(
       id: $peopleId){
       totalCount
       }}
`;"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code(), @r"
    const Foo = gql`
    	query PeopleCount {
    		people(id: $peopleId) {
    			totalCount
    		}
    	}
    `;

    const Bar = graphql(`
    	query PeopleCount {
    		people(id: $peopleId) {
    			totalCount
    		}
    	}
    `);

    const Baz = graphql`
    	query PeopleCount {
    		people(id: $peopleId) {
    			totalCount
    		}
    	}
    `;
    ");
}

#[test]
fn issue_9131() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"
const bulkUpsertTransactionsMutation = graphql(`
  mutation test(
    $input: Test!
  ) {
    test(input: $input) {
      apple
    }
  }
`);

console.log(`test`) // plain template as call argument

const highlight = foo`some tagged template` // unknown tagged template
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code());
}

/// Parenthesized expressions before a graphql tagged template used to crash
/// the formatter because the syntax rewriter removes parentheses, shifting
/// text ranges. The embedding service stores original ranges but the formatter
/// used transformed ranges, causing a mismatch that left orphaned
/// StartEmbedded tags in the document.
///
/// See: https://github.com/biomejs/biome/issues/9484
#[test]
fn issue_9484_parens_before_graphql_call() {
    const FILE_PATH: &str = "/project/file.js";
    const FILE_CONTENT: &str = r#"import {graphql} from "@generated/gql.js";

const a = {}
console.log((a))

const fetchFileUploadUrlQuery =
graphql(`
  query Q {
    field
  }
`);
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code());
}

/// After `format_embedded()` replaces StartEmbedded tags with embedded content
/// containing hard line breaks, `propagate_expand()` must be called again so
/// that enclosing groups learn they need to expand. Without it, elements like
/// `IndentIfGroupBreaks` around the call arguments would not indent because
/// the group mode would still be flat.
#[test]
fn issue_9484_propagate_expand_after_embed() {
    const FILE_PATH: &str = "/project/file.js";
    // Short call where graphql fits on one line without embedding,
    // but embedded formatting inserts hard lines that must expand the group.
    const FILE_CONTENT: &str = r#"const x = foo(graphql`query { a }`, b)
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code());
}

/// Parenthesized JSX return combined with a graphql tagged template literal
/// triggered the same range mismatch as issue_9484_parens_before_graphql_call.
///
/// See: https://github.com/biomejs/biome/issues/9484
#[test]
fn issue_9484_parens_jsx_with_graphql_tag() {
    const FILE_PATH: &str = "/project/file.tsx";
    const FILE_CONTENT: &str = r#"import { graphql, useLazyLoadQuery } from 'react-relay';

export const Page = () => {
  return (<div></div>);
};

const Table = () => {
  const query = useLazyLoadQuery(graphql`
      query Q {
        field
      }
    `, {});
  return <div></div>;
};
"#;

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    experimental_embedded_snippets_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();

    insta::assert_snapshot!(result.as_code());
}

#[test]
fn lsp_language_hints_keep_svelte_source_module_path_semantics() {
    const SVELTE_TS_FILE_PATH: &str = "/project/component.svelte.ts";
    const SVELTE_JS_FILE_PATH: &str = "/project/component.svelte.js";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from(SVELTE_TS_FILE_PATH),
        b"export const count = 1;",
    );
    fs.insert(
        Utf8PathBuf::from(SVELTE_JS_FILE_PATH),
        b"export const count = 1;",
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(SVELTE_TS_FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: Some(DocumentFileSource::from_language_id("typescript", None)),
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(SVELTE_JS_FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: Some(DocumentFileSource::from_language_id("javascript", None)),
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let ts_file_source = workspace.get_file_source(SVELTE_TS_FILE_PATH.into(), false);
    let ts = ts_file_source.to_js_file_source().expect("JS file source");
    assert!(ts.is_svelte_source_module());
    assert!(ts.is_typescript());

    let js_file_source = workspace.get_file_source(SVELTE_JS_FILE_PATH.into(), false);
    let js = js_file_source.to_js_file_source().expect("JS file source");
    assert!(js.is_svelte_source_module());
    assert!(!js.is_typescript());
}

// noUndeclaredClasses

/// A class used in `class="..."` that has no matching `.foo {}` in any `<style>`
/// block should be flagged.
#[test]
fn no_undeclared_classes_reports_unknown_class() {
    const FILE_CONTENT: &str = r#"<style>.card { border: 1px solid; }</style>
<div class="header">Content</div>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        FILE_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert_eq!(
        result.diagnostics.len(),
        1,
        "Expected one diagnostic for undeclared class 'header'"
    );
    assert!(
        format!("{:?}", result.diagnostics[0]).contains("header"),
        "Diagnostic should mention 'header'"
    );
}

/// When every class used in `class="..."` is defined in a `<style>` block,
/// no diagnostics should be emitted.
#[test]
fn no_undeclared_classes_passes_when_class_is_defined() {
    const FILE_CONTENT: &str = r#"<style>.card { border: 1px solid; }</style>
<div class="card">Content</div>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        FILE_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics when class is declared"
    );
}

/// An HTML file with no `<style>` blocks and no linked stylesheets should
/// never emit diagnostics, to avoid false positives on unstyled HTML.
#[test]
fn no_undeclared_classes_silent_without_style_info() {
    const FILE_CONTENT: &str = r#"<div class="anything">Content</div>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        FILE_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics when the file has no style information"
    );
}

/// Multiple classes in one `class` attribute: only undeclared ones flagged.
#[test]
fn no_undeclared_classes_reports_only_undeclared_in_multi_class() {
    const FILE_CONTENT: &str = r#"<style>.card { border: 1px solid; } .title { font-weight: bold; }</style>
<div class="card header title footer">Content</div>"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        FILE_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    // "card" and "title" are declared; "header" and "footer" are not.
    assert_eq!(
        result.diagnostics.len(),
        2,
        "Expected diagnostics for 'header' and 'footer' only"
    );
}

// noUnusedClasses

/// A CSS class that no JS/HTML file imports or references should be flagged.
#[test]
fn no_unused_classes_reports_unreferenced_class() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        b".unused { color: red; }",
    );

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
            path: BiomePath::new("/project/styles.css"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert_eq!(
        result.diagnostics.len(),
        1,
        "Expected one diagnostic for unreferenced class 'unused'"
    );
    assert!(
        format!("{:?}", result.diagnostics[0]).contains("unused"),
        "Diagnostic should mention 'unused'"
    );
}

/// A CSS class that is referenced via `className` in a JSX file that imports
/// the stylesheet should not be flagged.
#[test]
fn no_unused_classes_passes_when_class_is_referenced_in_jsx() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        b".button { color: blue; }",
    );
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        b"import \"./styles.css\";\nexport default () => <div className=\"button\" />;",
    );

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
            path: BiomePath::new("/project/styles.css"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics when class is referenced in importing JSX"
    );
}

/// Only unused classes should be flagged; referenced ones should pass.
#[test]
fn no_unused_classes_reports_only_unreferenced_classes() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        b".used { color: green; } .orphan { color: red; }",
    );
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        b"import \"./styles.css\";\nexport default () => <div className=\"used\" />;",
    );

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
            path: BiomePath::new("/project/styles.css"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedClasses").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert_eq!(
        result.diagnostics.len(),
        1,
        "Expected one diagnostic for unreferenced class 'orphan'"
    );
    assert!(
        format!("{:?}", result.diagnostics[0]).contains("orphan"),
        "Diagnostic should mention 'orphan'"
    );
}

/// A CSS class referenced via a transitive CSS @import chain should not be
/// flagged. If app.jsx imports theme.css which @imports base.css, classes in
/// base.css that are used in app.jsx are considered referenced.
#[test]
fn no_unused_classes_passes_with_transitive_css_import() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/base.css"),
        b".base { box-sizing: border-box; }",
    );
    fs.insert(
        Utf8PathBuf::from("/project/theme.css"),
        b"@import \"./base.css\"; .theme { background: white; }",
    );
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        b"import \"./theme.css\";\nexport default () => <div className=\"base theme\" />;",
    );

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

    // Open all files so that the module graph is fully populated.
    for path in [
        "/project/App.jsx",
        "/project/theme.css",
        "/project/base.css",
    ] {
        workspace
            .open_file(OpenFileParams {
                project_key,
                path: BiomePath::new(path),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
    }

    for path in ["/project/base.css", "/project/theme.css"] {
        let result = workspace
            .pull_diagnostics(PullDiagnosticsParams {
                path: BiomePath::new(path),
                only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedClasses").unwrap()],
                skip: vec![],
                enabled_rules: vec![],
                project_key,
                categories: Default::default(),
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: Severity::Hint,
                enforce_assist: false,
            })
            .unwrap();

        assert!(
            result.diagnostics.is_empty(),
            "Expected no diagnostics for {path} — all classes are transitively referenced"
        );
    }
}

#[test]
fn go_to_definition_named_import() {
    const UTILS_CONTENT: &str = "export function greet() { return 'hello'; }\n";
    const MAIN_CONTENT: &str = "import { greet } from './utils.js';\ngreet();\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/utils.js"),
        UTILS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/main.js"),
        MAIN_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on `greet` in `import { greet }` — byte offset 9 (start of "greet")
    let cursor_range = TextRange::new(TextSize::from(9), TextSize::from(9));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/main.js"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should find a definition");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path.as_path(), Utf8Path::new("/project/utils.js"));
    // The `greet` binding in utils.js starts at byte 16 (after "export function ")
    assert_eq!(range.start(), TextSize::from(16));
    assert_eq!(range.end(), TextSize::from(21));
}

#[test]
fn go_to_definition_default_import() {
    const UTILS_CONTENT: &str = "export default function myFunc() { return 42; }\n";
    const MAIN_CONTENT: &str = "import myFunc from './utils.js';\nmyFunc();\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/utils.js"),
        UTILS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/main.js"),
        MAIN_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on `myFunc` in `import myFunc` — byte offset 7
    let cursor_range = TextRange::new(TextSize::from(7), TextSize::from(7));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/main.js"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should find a definition for default import");
    assert_eq!(definition.matches.len(), 1);
    let (path, _range) = &definition.matches[0];
    assert_eq!(path.as_path(), Utf8Path::new("/project/utils.js"));
}

#[test]
fn go_to_definition_same_file_local_binding() {
    const CONTENT: &str = "const myVar = 42;\nconsole.log(myVar);\n";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/main.js"), CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on `myVar` in `console.log(myVar)` — byte offset 30
    let cursor_range = TextRange::new(TextSize::from(30), TextSize::from(30));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/main.js"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should find a local definition");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path.as_path(), Utf8Path::new("/project/main.js"));
    // `myVar` is declared at byte 6 (after "const ")
    assert_eq!(range.start(), TextSize::from(6));
}

#[test]
fn go_to_definition_returns_none_for_node_modules() {
    const UTILS_CONTENT: &str = "export function helper() {}\n";
    const MAIN_CONTENT: &str = "import { helper } from 'external-pkg';\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/node_modules/external-pkg/index.js"),
        UTILS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/main.js"),
        MAIN_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on `helper` in `import { helper }` — byte offset 9
    let cursor_range = TextRange::new(TextSize::from(9), TextSize::from(9));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/main.js"),
            cursor_range,
        })
        .unwrap();

    match result {
        None => {}
        Some(definition) => {
            assert!(
                definition.matches.is_empty(),
                "should not resolve node_modules imports, got: {:?}",
                definition.matches
            );
        }
    }
}

#[test]
fn go_to_definition_returns_none_for_cursor_on_non_identifier() {
    const CONTENT: &str = "const x = 1;\n";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/main.js"), CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on `=` at byte offset 8
    let cursor_range = TextRange::new(TextSize::from(8), TextSize::from(8));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/main.js"),
            cursor_range,
        })
        .unwrap();

    assert!(
        result.is_none(),
        "should return None when cursor is not on an identifier"
    );
}

#[test]
fn go_to_definition_jsx_classname_to_css() {
    // `.btn { color: red; }\n` — "btn" starts at offset 1
    const CSS_CONTENT: &str = ".btn { color: red; }\n";
    // `import './styles.css';\n<div className="btn" />\n`
    // "btn" in className is at offset 38 (after the opening quote at 37)
    const JSX_CONTENT: &str = "import './styles.css';\n<div className=\"btn\" />\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        CSS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        JSX_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on "btn" inside className="btn" — byte offset 39
    let cursor_range = TextRange::new(TextSize::from(39), TextSize::from(39));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/App.jsx"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve className to CSS class");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/project/styles.css"));
    // "btn" in `.btn` starts at offset 1 (after the dot)
    assert_eq!(range, &TextRange::new(TextSize::from(1), TextSize::from(4)));
}

#[test]
fn go_to_definition_jsx_classname_multiple_classes() {
    const CSS_CONTENT: &str = ".foo { } .bar { } .baz { }\n";
    // `import './styles.css';\n<div className="foo bar baz" />\n`
    const JSX_CONTENT: &str = "import './styles.css';\n<div className=\"foo bar baz\" />\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        CSS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        JSX_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on "bar" inside className="foo bar baz" — "bar" starts at offset 43
    let cursor_range = TextRange::new(TextSize::from(43), TextSize::from(43));

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/App.jsx"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve to .bar in CSS");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/project/styles.css"));
    // ".bar" is at offset 9, so "bar" name starts at 10
    assert_eq!(
        range,
        &TextRange::new(TextSize::from(10), TextSize::from(13))
    );
}

#[test]
fn go_to_definition_html_class_to_css() {
    const CSS_CONTENT: &str = ".header { margin: 0; }\n";
    const HTML_CONTENT: &str =
        "<link rel=\"stylesheet\" href=\"./styles.css\" />\n<div class=\"header\">Hello</div>\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/styles.css"),
        CSS_CONTENT.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        HTML_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on "header" inside class="header" — find the offset
    // `<link rel="stylesheet" href="./styles.css" />\n<div class="header">Hello</div>\n`
    // The `class="header"` part: "header" starts after the quote
    let class_value_start = HTML_CONTENT.find("\"header\"").unwrap() + 1; // after the quote
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/index.html"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve HTML class to CSS class");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/project/styles.css"));
    // "header" in `.header` starts at offset 1
    assert_eq!(range, &TextRange::new(TextSize::from(1), TextSize::from(7)));
}

#[test]
fn go_to_definition_html_class_inline_style() {
    const HTML_CONTENT: &str =
        "<style>.card { padding: 1rem; }</style>\n<div class=\"card\">Content</div>\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        HTML_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    // Cursor on "card" inside class="card"
    let class_value_start = HTML_CONTENT.find("\"card\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/index.html"),
            cursor_range,
        })
        .unwrap();

    // Inline style classes should resolve to the same HTML file
    let definition = result.expect("should resolve HTML class to inline style");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/project/index.html"));
    // "card" in `.card` inside <style> block — must be in parent document coordinates
    let style_offset = HTML_CONTENT.find("<style>").unwrap() + "<style>".len();
    // ".card" starts at offset 0 in snippet, "card" at offset 1
    let expected_start = style_offset + 1;
    let expected_end = expected_start + 4;
    assert_eq!(
        range,
        &TextRange::new(
            TextSize::from(expected_start as u32),
            TextSize::from(expected_end as u32),
        ),
        "range should be in parent document coordinates"
    );
}

/// Regression: `.foobar` must NOT match a lookup for `foo`.
/// Substring matching would incorrectly resolve `foo` to the `.foobar` rule.
#[test]
fn go_to_definition_inline_style_no_substring_match() {
    const HTML_CONTENT: &str =
        "<style>.foobar { color: red; }</style>\n<div class=\"foo\">Content</div>\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        HTML_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    let class_value_start = HTML_CONTENT.find("\"foo\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/index.html"),
            cursor_range,
        })
        .unwrap();

    match result {
        None => {}
        Some(definition) => {
            assert!(
                definition.matches.is_empty(),
                "`.foobar` should not match a lookup for `foo`, got: {:?}",
                definition.matches
            );
        }
    }
}

#[test]
fn go_to_definition_vue_class_to_inline_style() {
    const VUE_CONTENT: &str = "\
<template>
  <div class=\"card\">Hello</div>
</template>

<style>
.card { padding: 1rem; }
</style>
";
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/App.vue"), VUE_CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    let configuration =
        biome_deserialize::json::deserialize_from_json_str::<biome_configuration::Configuration>(
            r#"{ "html": { "experimentalFullSupportEnabled": true } }"#,
            biome_json_parser::JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/")),
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .unwrap();

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
            path: BiomePath::new("/App.vue"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    // Cursor on "card" inside class="card"
    let class_value_start = VUE_CONTENT.find("\"card\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/App.vue"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve Vue class to inline style");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/App.vue"));
    // "card" in `.card` inside <style> block — range must be in parent document coordinates
    let style_offset = VUE_CONTENT.find("<style>").unwrap() + "<style>\n".len();
    // ".card" starts at offset 0 in snippet, "card" at offset 1
    let expected_start = style_offset + 1;
    let expected_end = expected_start + 4;
    assert_eq!(
        range,
        &TextRange::new(
            TextSize::from(expected_start as u32),
            TextSize::from(expected_end as u32),
        ),
        "range should be in parent document coordinates, not snippet-local"
    );
}

#[test]
fn go_to_definition_vue_class_with_script_and_style() {
    const VUE_CONTENT: &str = "\
<script setup>
import { foo } from './file.ts';
foo();
</script>

<div class=\"btn\">Hello</div>

<style>
.btn {
    bottom: 0;
}
</style>
";
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/App.vue"), VUE_CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    let configuration =
        biome_deserialize::json::deserialize_from_json_str::<biome_configuration::Configuration>(
            r#"{ "html": { "experimentalFullSupportEnabled": true } }"#,
            biome_json_parser::JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/")),
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .unwrap();

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
            path: BiomePath::new("/App.vue"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    // Cursor on "btn" inside class="btn"
    let class_value_start = VUE_CONTENT.find("\"btn\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/App.vue"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve Vue class to inline style with script present");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/App.vue"));
    // "btn" in `.btn` inside <style> — must be in parent document coordinates
    let style_offset = VUE_CONTENT.find("<style>").unwrap() + "<style>\n".len();
    let expected_start = style_offset + 1; // skip the dot in `.btn`
    let expected_end = expected_start + 3;
    assert_eq!(
        range,
        &TextRange::new(
            TextSize::from(expected_start as u32),
            TextSize::from(expected_end as u32),
        ),
        "range should be in parent document coordinates when both script and style exist"
    );
}

#[test]
fn go_to_definition_vue_class_to_external_css() {
    const CSS_CONTENT: &str = ".wrapper { display: flex; }\n";
    const VUE_CONTENT: &str = "\
<link rel=\"stylesheet\" href=\"./styles.css\" />
<template>
  <div class=\"wrapper\">Hello</div>
</template>
";
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/styles.css"), CSS_CONTENT.as_bytes());
    fs.insert(Utf8PathBuf::from("/App.vue"), VUE_CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    let configuration =
        biome_deserialize::json::deserialize_from_json_str::<biome_configuration::Configuration>(
            r#"{ "html": { "experimentalFullSupportEnabled": true } }"#,
            biome_json_parser::JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/")),
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::ModulesAndTypes,
        })
        .unwrap();

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on "wrapper" inside class="wrapper"
    let class_value_start = VUE_CONTENT.find("\"wrapper\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/App.vue"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve Vue class to external CSS");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/styles.css"));
    // "wrapper" in `.wrapper` starts at offset 1
    assert_eq!(range, &TextRange::new(TextSize::from(1), TextSize::from(8)));
}

#[test]
fn go_to_definition_html_class_to_css_imported_from_script() {
    const CSS_CONTENT: &str = ".foo { color: red; }\n";
    // Astro-like: CSS imported via JS in a <script> block
    const HTML_CONTENT: &str = "\
<script>
import './styles.css';
</script>

<div class=\"foo\">Hello</div>

<style>
.local { margin: 0; }
</style>
";
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/styles.css"), CSS_CONTENT.as_bytes());
    fs.insert(Utf8PathBuf::from("/index.html"), HTML_CONTENT.as_bytes());

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // Cursor on "foo" inside class="foo"
    let class_value_start = HTML_CONTENT.find("\"foo\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(class_value_start as u32),
        TextSize::from(class_value_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/index.html"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve class to CSS imported from script block");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/styles.css"));
    // "foo" in `.foo` starts at offset 1
    assert_eq!(range, &TextRange::new(TextSize::from(1), TextSize::from(4)));
}

#[test]
fn go_to_definition_css_class_multiple_matches() {
    // `.btn` defined in two separate stylesheets, both imported by a JSX file.
    const CSS_A: &str = ".btn { color: red; }\n";
    const CSS_B: &str = ".btn { font-size: 16px; }\n";
    const JSX_CONTENT: &str = "import './a.css';\nimport './b.css';\n<div className=\"btn\" />\n";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.css"), CSS_A.as_bytes());
    fs.insert(Utf8PathBuf::from("/project/b.css"), CSS_B.as_bytes());
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        JSX_CONTENT.as_bytes(),
    );

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::TypeAware,
            verbose: false,
        })
        .unwrap();

    // "btn" inside className="btn"
    let btn_start = JSX_CONTENT.find("\"btn\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(btn_start as u32),
        TextSize::from(btn_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/App.jsx"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve className to CSS class in both files");
    assert_eq!(definition.matches.len(), 2, "expected two matches");

    let paths: Vec<_> = definition.matches.iter().map(|(p, _)| p.clone()).collect();
    assert!(
        paths.contains(&BiomePath::new("/project/a.css")),
        "should contain a.css"
    );
    assert!(
        paths.contains(&BiomePath::new("/project/b.css")),
        "should contain b.css"
    );

    // Both define `.btn` at the same position: "btn" starts at offset 1
    let expected_range = TextRange::new(TextSize::from(1), TextSize::from(4));
    for (_, range) in &definition.matches {
        assert_eq!(range, &expected_range);
    }
}

#[test]
fn go_to_definition_css_class_via_transitive_import() {
    // App.jsx imports app.css, which @imports components.css.
    // `.card` is defined only in components.css — go-to-definition should find it.
    const COMPONENTS_CSS: &str = ".card { border: 1px solid; }\n";
    const APP_CSS: &str = "@import './components.css';\n.wrapper { display: flex; }\n";
    const JSX_CONTENT: &str = "import './app.css';\n<div className=\"card\" />\n";

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/components.css"),
        COMPONENTS_CSS.as_bytes(),
    );
    fs.insert(Utf8PathBuf::from("/project/app.css"), APP_CSS.as_bytes());
    fs.insert(
        Utf8PathBuf::from("/project/App.jsx"),
        JSX_CONTENT.as_bytes(),
    );

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

    // Cursor on "card" inside className="card"
    let card_start = JSX_CONTENT.find("\"card\"").unwrap() + 1;
    let cursor_range = TextRange::new(
        TextSize::from(card_start as u32),
        TextSize::from(card_start as u32),
    );

    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/App.jsx"),
            cursor_range,
        })
        .unwrap();

    let definition = result.expect("should resolve className to CSS class in transitive import");
    assert_eq!(definition.matches.len(), 1);
    let (path, range) = &definition.matches[0];
    assert_eq!(path, &BiomePath::new("/project/components.css"));
    // "card" in `.card` starts at offset 1 (after the dot)
    assert_eq!(range, &TextRange::new(TextSize::from(1), TextSize::from(5)));
}
#[test]
fn fix_file_is_idempotent_for_template_literals_and_css_block_comments() {
    // Regression: reindent_embedded_code was adding the host indentation prefix
    // to continuation lines inside template literals and CSS block comments, so
    // each successive `biome check --write` stacked another indent level.
    // HTML files exercise the update_snippets → reindent_embedded_code path.
    const FILE_PATH: &str = "/project/page.html";
    const FILE_CONTENT: &str = "<html>\n\t<head>\n\t\t<script>\n\t\t\tconst sql = `\n\t\t\t\tSELECT *\n\t\t\t\tFROM users\n\t\t\t`;\n\t\t</script>\n\t\t<style>\n\t\t\t/*\n\t\t\t * A block comment.\n\t\t\t */\n\t\t\t.foo { color: red; }\n\t\t</style>\n\t</head>\n</html>\n";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let first = workspace
        .fix_file(FixFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            fix_file_mode: FixFileMode::SafeFixes,
            should_format: true,
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            rule_categories: RuleCategories::default(),
            suppression_reason: None,
            inline_config: None,
        })
        .unwrap();

    workspace
        .change_file(ChangeFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: first.code.clone(),
            version: 1,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let second = workspace
        .fix_file(FixFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            fix_file_mode: FixFileMode::SafeFixes,
            should_format: true,
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            rule_categories: RuleCategories::default(),
            suppression_reason: None,
            inline_config: None,
        })
        .unwrap();

    assert_eq!(
        first.code, second.code,
        "fix_file must be idempotent: template literal and block comment continuation lines must not gain an extra indent on each run"
    );
}
