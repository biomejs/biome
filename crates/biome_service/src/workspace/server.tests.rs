use super::*;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use biome_configuration::{
    FormatterConfiguration, JsConfiguration,
    javascript::{JsFormatterConfiguration, JsParserConfiguration},
};
use biome_formatter::{IndentStyle, LineWidth};
use biome_fs::MemoryFileSystem;
use biome_rowan::TextSize;

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
