use std::collections::BTreeMap;
use std::num::NonZeroU64;
use std::str::FromStr;
use std::sync::Arc;

use biome_analyze::RuleCategories;
use biome_configuration::analyzer::{RuleGroup, RuleSelector};
use biome_configuration::{
    Configuration, FilesConfiguration, OverrideGlobs, OverridePattern, Overrides,
};
use biome_diagnostics::Diagnostic;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_syntax::{JsFileSource, TextSize};
use biome_plugin_loader::{PluginConfiguration, Plugins};
use camino::Utf8PathBuf;
use insta::{assert_debug_snapshot, assert_snapshot};

use crate::file_handlers::DocumentFileSource;
use crate::projects::ProjectKey;
use crate::{Workspace, WorkspaceError};

use super::{
    CloseFileParams, CloseProjectParams, FileContent, FileFeaturesResult, FileGuard,
    GetModuleGraphParams, GetSyntaxTreeParams, OpenFileParams, OpenProjectParams,
    OpenProjectResult, PullDiagnosticsParams, ScanKind, ScanProjectParams, UpdateKind,
    UpdateModuleGraphParams, UpdateSettingsParams, server,
};

fn create_server() -> (Box<dyn Workspace>, ProjectKey) {
    let workspace = server(Arc::new(MemoryFileSystem::default()), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Default::default(),
            open_uninitialized: true,
        })
        .unwrap();

    (workspace, project_key)
}

#[test]
fn debug_control_flow() {
    const SOURCE: &str = "function test () { return; }";
    const GRAPH: &str = "flowchart TB
    block_0[\"<b>block_0</b><br/>Return(JS_RETURN_STATEMENT 19..26)<br/>Return\"]\n\n";

    let (workspace, project_key) = create_server();
    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.js"),
            content: FileContent::from_client(SOURCE),
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
    let (workspace, project_key) = create_server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.d.ts"),
            // the following code snippet can be correctly parsed in .d.ts file but not in .ts file
            content: FileContent::from_client("export const foo: number"),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();

    assert!(file.format_file().is_ok());
}

#[test]
fn correctly_handle_json_files() {
    let (workspace, project_key) = create_server();

    // ".json" file
    let json_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("a.json"),
            content: FileContent::from_client(r#"{"a": 42}"#),
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
            project_key,
            path: BiomePath::new("b.json"),
            content: FileContent::from_client(r#"{"a": 42}//comment"#),
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
            project_key,
            path: BiomePath::new("c.json"),
            content: FileContent::from_client(r#"{"a": 42,}"#),
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
            project_key,
            path: BiomePath::new("d.jsonc"),
            content: FileContent::from_client(r#"{"a": 42}//comment"#),
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
            project_key,
            path: BiomePath::new("e.jsonc"),
            content: FileContent::from_client(r#"{"a": 42,}"#),
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
            project_key,
            path: BiomePath::new(".eslintrc.json"),
            content: FileContent::from_client(r#"{"a": 42}//comment"#),
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
            project_key,
            path: BiomePath::new("project/.vscode/settings.json"),
            content: FileContent::from_client(r#"{"a": 42}//comment"#),
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
            project_key,
            path: BiomePath::new("dir/.eslintrc.json"),
            content: FileContent::from_client(r#"{"a": 42,}"#),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    assert!(
        well_known_json_with_comments_file_with_trailing_commas
            .format_file()
            .is_err()
    );

    // well-known json-with-comments-and-trailing-commas file allows comments and trailing commas
    let well_known_json_with_comments_and_trailing_commas_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("tsconfig.json"),
            content: FileContent::from_client(r#"{"a": 42,}//comment"#),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    assert!(
        well_known_json_with_comments_and_trailing_commas_file
            .format_file()
            .is_ok()
    );
}

#[test]
fn correctly_parses_graphql_files() {
    let (workspace, project_key) = create_server();

    let graphql_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.graphql"),
            content: FileContent::from_client(
                r#"type Query {
  me: User
}

type User {
  id: ID
  name: String
}"#,
            ),
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
    let (workspace, project_key) = create_server();

    let graphql_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.graphql"),
            content: FileContent::from_client(
                r#"query {
  member @deprecated(abc: 123)
}"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    let result = graphql_file.pull_diagnostics(
        RuleCategories::all(),
        vec![RuleSelector::Rule(RuleGroup::Style.as_str(), "useDeprecatedReason").into()],
        vec![],
        true,
    );
    assert!(result.is_ok());
    let diagnostics = result.unwrap().diagnostics;
    assert_eq!(diagnostics.len(), 1)
}

#[test]
fn pull_grit_debug_info() {
    let (workspace, project_key) = create_server();

    let grit_file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.grit"),
            content: FileContent::from_client(
                r#"`function ($args) { $body }` where {
  $args <: contains `x`
}"#,
            ),
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

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.ts"), FILE_A_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/b.ts"), FILE_B_CONTENT);

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
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

    macro_rules! assert_file_a_index {
        () => {{
            let module_graph = workspace
                .get_module_graph(GetModuleGraphParams {})
                .expect("can get module graph");

            assert_eq!(
                module_graph
                    .data
                    .get("/project/a.ts")
                    .map(|module_info| module_info.static_import_paths.clone()),
                Some(BTreeMap::from([(
                    "./b.ts".to_string(),
                    "/project/b.ts".replace('/', std::path::MAIN_SEPARATOR_STR),
                )])),
            );
        }};
    }

    assert_file_a_index!();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.ts"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    assert_file_a_index!();

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new("/project/a.ts"),
        })
        .unwrap();

    assert_file_a_index!();

    workspace
        .close_project(CloseProjectParams { project_key })
        .unwrap();

    let module_graph = workspace
        .get_module_graph(GetModuleGraphParams {})
        .expect("can get module graph");

    assert!(!module_graph.data.contains_key("/project/a.ts"));
}

#[test]
fn too_large_files_are_tracked_but_not_parsed() {
    const FILE_CONTENT: &[u8] = b"console.log(`I'm YUUUGE!`);";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/a.ts"), FILE_CONTENT);

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                files: Some(FilesConfiguration {
                    max_size: Some(NonZeroU64::new(10).unwrap().into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            workspace_directory: None,
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

    assert!(
        workspace
            .get_syntax_tree(GetSyntaxTreeParams {
                project_key,
                path: BiomePath::new("/project/a.ts"),
            })
            .is_err_and(|error| matches!(error, WorkspaceError::FileIgnored(_)))
    );
}

#[test]
fn plugins_are_loaded_and_used_during_analysis() {
    const PLUGIN_CONTENT: &[u8] = br#"
`Object.assign($args)` where {
    register_diagnostic(
        span = $args,
        message = "Prefer object spread instead of `Object.assign()`"
    )
}
"#;

    const FILE_CONTENT: &[u8] = b"const a = Object.assign({ foo: 'bar' });";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/plugin.grit"), PLUGIN_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/a.ts"), FILE_CONTENT);

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                plugins: Some(Plugins(vec![PluginConfiguration::Path(
                    "./plugin.grit".to_string(),
                )])),
                ..Default::default()
            },
            workspace_directory: Some(BiomePath::new("/project")),
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

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new("/project/a.ts"),
            categories: RuleCategories::default(),
            only: Vec::new(),
            skip: Vec::new(),
            enabled_rules: Vec::new(),
            pull_code_actions: true,
        })
        .unwrap();
    assert_debug_snapshot!(result.diagnostics);
    assert_eq!(result.errors, 0);
}

#[test]
fn plugins_can_use_custom_severity() {
    const PLUGIN_CONTENT: &[u8] = br#"
language css;
`$selector { $props }` where {
    $props <: contains `color: $color` as $rule,
    not $selector <: r"\.color-.*",
    register_diagnostic(
        span = $rule,
        message = "Don't set explicit colors. Use `.color-*` classes instead.",
        severity = "warn"
    )
}
"#;

    const FILE_CONTENT: &[u8] = b"p { color: red }";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/plugin.grit"), PLUGIN_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/a.css"), FILE_CONTENT);

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                plugins: Some(Plugins(vec![PluginConfiguration::Path(
                    "./plugin.grit".to_string(),
                )])),
                ..Default::default()
            },
            workspace_directory: Some(BiomePath::new("/project")),
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/a.css"),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new("/project/a.css"),
            categories: RuleCategories::default(),
            only: Vec::new(),
            skip: Vec::new(),
            enabled_rules: Vec::new(),
            pull_code_actions: true,
        })
        .unwrap();
    assert_debug_snapshot!(result.diagnostics);
    assert_eq!(result.errors, 0);
}

#[test]
fn plugins_may_use_invalid_span() {
    const PLUGIN_CONTENT: &[u8] = br#"
`Object.assign($args)` where {
    register_diagnostic(
        span = `Object.assign`,
        message = "Prefer object spread instead of `Object.assign()`"
    )
}
"#;

    const FILE_CONTENT: &[u8] = b"const a = Object.assign({ foo: 'bar' });";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from("/project/plugin.grit"), PLUGIN_CONTENT);
    fs.insert(Utf8PathBuf::from("/project/a.ts"), FILE_CONTENT);

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                plugins: Some(Plugins(vec![PluginConfiguration::Path(
                    "./plugin.grit".to_string(),
                )])),
                ..Default::default()
            },
            workspace_directory: Some(BiomePath::new("/project")),
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

    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new("/project/a.ts"),
            categories: RuleCategories::default(),
            only: Vec::new(),
            skip: Vec::new(),
            enabled_rules: Vec::new(),
            pull_code_actions: true,
        })
        .unwrap();
    assert_debug_snapshot!(result.diagnostics);
    assert_eq!(result.errors, 0);
}

#[test]
fn correctly_apply_plugins_in_override() {
    let files: &[(&str, &[u8])] = &[
    (
        "/project/plugin_a.grit",
        br#"`Object.assign($args)` where {
    register_diagnostic(
        span = $args,
        message = "Prefer object spread instead of `Object.assign()`"
    )
}"#,
    ),
    (
        "/project/plugin_b.grit",
        br#"`Object.keys($args)` where {
    register_diagnostic(
        span = $args,
        message = "Consider using `for...in` loop instead of `Object.keys()` for simple object iteration."
    )
}"#,
    ),
    (
        "/project/plugin_c.grit",
        br#"`Object.hasOwn($args)` where {
    register_diagnostic(
        span = $args,
        message = "Ensure compatibility: `Object.hasOwn()` may not be supported in all environments."
    )
}"#,
    ),
    (
        "/project/a.ts",
        br#"
const a = Object.assign({ foo: 'bar' });
const keys = Object.keys({ foo: 'bar' });
const hasOwn = Object.hasOwn({ foo: 'bar' }, 'foo');"#,
    ),
    (
        "/project/lib/b.ts",
        br#"
const a = Object.assign({ foo: 'bar' });
const keys = Object.keys({ foo: 'bar' });
const hasOwn = Object.hasOwn({ foo: 'bar' }, 'foo');"#,
    ),
];

    let fs = MemoryFileSystem::default();
    for (path, content) in files {
        fs.insert(Utf8PathBuf::from(*path), *content);
    }

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                plugins: Some(Plugins(vec![PluginConfiguration::Path(
                    "./plugin_a.grit".to_string(),
                )])),
                overrides: Some(Overrides(vec![
                    OverridePattern {
                        includes: Some(OverrideGlobs::Globs(Box::new([
                            biome_glob::NormalizedGlob::from_str("./lib/**").unwrap(),
                        ]))),
                        plugins: Some(Plugins(vec![PluginConfiguration::Path(
                            "./plugin_b.grit".to_string(),
                        )])),
                        ..OverridePattern::default()
                    },
                    OverridePattern {
                        includes: Some(OverrideGlobs::Globs(Box::new([
                            biome_glob::NormalizedGlob::from_str("./utils/**").unwrap(),
                        ]))),
                        plugins: Some(Plugins(vec![PluginConfiguration::Path(
                            "./plugin_c.grit".to_string(),
                        )])),
                        ..OverridePattern::default()
                    },
                ])),
                ..Default::default()
            },
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

    for (path, expect_diagnosis_count) in [("/project/a.ts", 1), ("/project/lib/b.ts", 2)] {
        workspace
            .open_file(OpenFileParams {
                project_key,
                path: BiomePath::new(path),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
            })
            .unwrap();

        let result = workspace
            .pull_diagnostics(PullDiagnosticsParams {
                project_key,
                path: BiomePath::new(path),
                categories: RuleCategories::default(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: Vec::new(),
                pull_code_actions: true,
            })
            .unwrap();
        // Filter only diagnostics with category name "plugin"
        let plugin_diagnostics: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|diag| diag.category().is_some_and(|cat| cat.name() == "plugin"))
            .collect();
        let snapshot_name = format!("diagnostics_{path}");
        assert_debug_snapshot!(snapshot_name, plugin_diagnostics);
        assert!(plugin_diagnostics.len() == expect_diagnosis_count);
    }
}

#[test]
fn test_order() {
    for items in FileFeaturesResult::PROTECTED_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}

#[test]
fn debug_type_info() {
    let (workspace, project_key) = create_server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.ts"),
            content: FileContent::from_client(
                r#"
function foo(name: string, age: number): Person {
    return new Person(string, age)
}
class Person {
    #name: string
    #age: number
    constructor(name: string, age: number) {
        this.#name = name;
        this.#age = age;
    }
}
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    let result = file.get_type_info();
    assert!(result.is_ok());
    assert_snapshot!(result.unwrap());
}

#[test]
fn debug_registered_types() {
    let (workspace, project_key) = create_server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.ts"),
            content: FileContent::from_client(
                r#"
function foo(name: string, age: number): Person {
    return new Person(string, age)
}
class Person {
    #name: string
    #age: number
    constructor(name: string, age: number) {
        this.#name = name;
        this.#age = age;
    }
}
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    let result = file.get_registered_types();
    assert!(result.is_ok());
    assert_snapshot!(result.unwrap());
}

#[test]
fn debug_semantic_model() {
    let (workspace, project_key) = create_server();

    let file = FileGuard::open(
        workspace.as_ref(),
        OpenFileParams {
            project_key,
            path: BiomePath::new("file.ts"),
            content: FileContent::from_client(
                r#"
function foo(name: string, age: number): Person {
    return new Person(string, age)
}
class Person {
    #name: string
    #age: number
    constructor(name: string, age: number) {
        this.#name = name;
        this.#age = age;
    }
}
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        },
    )
    .unwrap();
    let result = file.get_semantic_model();
    assert!(result.is_ok());
    assert_snapshot!(result.unwrap());
}

#[test]
fn debug_module_graph() {
    let fs = MemoryFileSystem::default();

    let workspace = server(Arc::new(fs), None);
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: Utf8PathBuf::from("/project").into(),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/file.js"),
            content: FileContent::from_client(
                r#"
import { filter, debounce } from "./utils.js";

async function test() {
    const {squash} = import("./dynamic.js");
}
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/utils.js"),

            content: FileContent::from_client(
                r#"
export const filter = function filter() {};

export const debounce = function debounce() {};
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new("/project/dynamic.js"),

            content: FileContent::from_client(
                r#"
export const squash = function squash() {};
"#,
            ),
            document_file_source: None,
            persist_node_cache: false,
        })
        .unwrap();

    workspace
        .update_module_graph(UpdateModuleGraphParams {
            path: BiomePath::new("/project/file.js"),
            update_kind: UpdateKind::AddOrUpdate,
        })
        .unwrap();
    workspace
        .update_module_graph(UpdateModuleGraphParams {
            path: BiomePath::new("/project/utils.js"),
            update_kind: UpdateKind::AddOrUpdate,
        })
        .unwrap();

    workspace
        .update_module_graph(UpdateModuleGraphParams {
            path: BiomePath::new("/project/dynamic.js"),
            update_kind: UpdateKind::AddOrUpdate,
        })
        .unwrap();

    let result = workspace.get_module_graph(GetModuleGraphParams {}).unwrap();

    assert_debug_snapshot!(result)
}
