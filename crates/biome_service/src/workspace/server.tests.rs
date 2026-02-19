use super::*;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use biome_configuration::{
    FormatterConfiguration, JsConfiguration,
    analyzer::AnalyzerSelector,
    javascript::{JsFormatterConfiguration, JsParserConfiguration},
};
use biome_formatter::{IndentStyle, LineWidth};
use biome_fs::MemoryFileSystem;
use biome_rowan::TextSize;
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
`);"#;

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
    ");
}

// ── noUndeclaredStyles ────────────────────────────────────────────────────────

/// A class used in `class="..."` that has no matching `.foo {}` in any `<style>`
/// block should be flagged.
#[test]
fn no_undeclared_styles_reports_unknown_class() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
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
fn no_undeclared_styles_passes_when_class_is_defined() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
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
fn no_undeclared_styles_silent_without_style_info() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics when the file has no style information"
    );
}

/// Multiple classes in one `class` attribute: only undeclared ones flagged.
#[test]
fn no_undeclared_styles_reports_only_undeclared_in_multi_class() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/index.html"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUndeclaredStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
        })
        .unwrap();

    // "card" and "title" are declared; "header" and "footer" are not.
    assert_eq!(
        result.diagnostics.len(),
        2,
        "Expected diagnostics for 'header' and 'footer' only"
    );
}

// ── noUnusedStyles ────────────────────────────────────────────────────────────

/// A CSS class that no JS/HTML file imports or references should be flagged.
#[test]
fn no_unused_styles_reports_unreferenced_class() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
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
fn no_unused_styles_passes_when_class_is_referenced_in_jsx() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
        })
        .unwrap();

    assert!(
        result.diagnostics.is_empty(),
        "Expected no diagnostics when class is referenced in importing JSX"
    );
}

/// Only unused classes should be flagged; referenced ones should pass.
#[test]
fn no_unused_styles_reports_only_unreferenced_classes() {
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
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new("/project/styles.css"),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedStyles").unwrap()],
            skip: vec![],
            enabled_rules: vec![],
            project_key,
            categories: Default::default(),
            pull_code_actions: false,
            inline_config: None,
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
fn no_unused_styles_passes_with_transitive_css_import() {
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
            })
            .unwrap();
    }

    for path in ["/project/base.css", "/project/theme.css"] {
        let result = workspace
            .pull_diagnostics(PullDiagnosticsParams {
                path: BiomePath::new(path),
                only: vec![AnalyzerSelector::from_str("lint/nursery/noUnusedStyles").unwrap()],
                skip: vec![],
                enabled_rules: vec![],
                project_key,
                categories: Default::default(),
                pull_code_actions: false,
                inline_config: None,
            })
            .unwrap();

        assert!(
            result.diagnostics.is_empty(),
            "Expected no diagnostics for {path} — all classes are transitively referenced"
        );
    }
}
