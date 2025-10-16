use biome_configuration::{
    FormatterConfiguration, JsConfiguration,
    javascript::{JsFormatterConfiguration, JsParserConfiguration},
};
use biome_formatter::{IndentStyle, LineWidth};
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
        })
        .unwrap();

    let result = workspace
        .format_file(FormatFileParams {
            path: Utf8PathBuf::from("/project/file.html").into(),
            project_key,
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
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.jsx"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
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
