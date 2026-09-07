use super::*;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::{FixFileMode, NoopQueryProvider, UpdateSettingsParams};
use biome_analyze::{RuleCategories, RuleCategoriesBuilder};
use biome_configuration::{
    FormatterConfiguration, HtmlConfiguration, JsConfiguration,
    analyzer::AnalyzerSelector,
    javascript::{JsFormatterConfiguration, JsParserConfiguration, JsResolverConfiguration},
    json::{JsonConfiguration, JsonFormatterConfiguration},
};
use biome_css_syntax::CssLanguage;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
use biome_db::testing::{Events, function_query_will_execute_count_by_name};
use biome_formatter::{IndentStyle, LineWidth, QuoteStyle};
use biome_fs::MemoryFileSystem;
use biome_js_syntax::JsLanguage;
use biome_json_formatter::context::TrailingCommas;
#[cfg(feature = "lang_html")]
use biome_languages::HtmlFileSource;
use biome_languages::css::CssEmbeddingKind;
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use salsa::plumbing::AsId;
use std::panic::AssertUnwindSafe;
use std::str::FromStr;
use std::time::{Duration, Instant};

fn wait_until(timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    while !predicate() {
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::yield_now();
    }
    true
}

#[cfg(all(feature = "module_graph", feature = "lang_js"))]
fn take_salsa_events(events: &Events) -> Vec<salsa::Event> {
    std::mem::take(&mut *events.0.lock().unwrap())
}

#[cfg(feature = "plugins")]
#[test]
fn close_project_removes_descendant_plugin_caches() {
    let (workspace, project_key) =
        setup_workspace_and_open_project(MemoryFileSystem::default(), "/project");
    let plugin_caches = workspace.server.plugin_caches.pin();
    plugin_caches.insert(Utf8PathBuf::from("/project"), PluginCache::default());
    plugin_caches.insert(
        Utf8PathBuf::from("/project/packages/nested"),
        PluginCache::default(),
    );
    plugin_caches.insert(
        Utf8PathBuf::from("/project-sibling"),
        PluginCache::default(),
    );
    drop(plugin_caches);

    workspace
        .close_project(CloseProjectParams { project_key })
        .unwrap();

    let plugin_caches = workspace.server.plugin_caches.pin();
    assert!(!plugin_caches.contains_key(Utf8Path::new("/project")));
    assert!(!plugin_caches.contains_key(Utf8Path::new("/project/packages/nested")));
    assert!(plugin_caches.contains_key(Utf8Path::new("/project-sibling")));
}

fn assert_settings_query_routes(db_state: DbState) {
    const PATH: &str = "/project/file.js";
    const SOURCE: &str = "knownGlobal; const value={foo:\"bar\"};";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = db_state;
    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap()
        .project_key;
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::from_client(SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let formatted = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(PATH),
            inline_config: None,
        })
        .unwrap();
    assert!(formatted.as_code().contains("\"bar\""));

    let range = TextRange::new(TextSize::from(0), TextSize::from(SOURCE.len() as u32));
    workspace
        .format_range(FormatRangeParams {
            project_key,
            path: BiomePath::new(PATH),
            range,
            inline_config: None,
        })
        .unwrap();
    let object_end = TextSize::from((SOURCE.find('}').unwrap() + 1) as u32);
    workspace
        .format_on_type(FormatOnTypeParams {
            project_key,
            path: BiomePath::new(PATH),
            offset: object_end,
            inline_config: None,
        })
        .unwrap();

    let no_undeclared =
        AnalyzerSelector::from_str("lint/correctness/noUndeclaredVariables").unwrap();
    let diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![no_undeclared],
            skip: vec![],
            enabled_rules: vec![no_undeclared],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();
    assert_eq!(diagnostics.diagnostics.len(), 1);

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                javascript: Some(JsConfiguration {
                    formatter: Some(JsFormatterConfiguration {
                        quote_style: Some(QuoteStyle::Single),
                        ..Default::default()
                    }),
                    globals: Some(["knownGlobal".into()].into_iter().collect()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    let formatted = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(PATH),
            inline_config: None,
        })
        .unwrap();
    assert!(formatted.as_code().contains("'bar'"));
    let diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![no_undeclared],
            skip: vec![],
            enabled_rules: vec![no_undeclared],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();
    assert!(diagnostics.diagnostics.is_empty());

    let inline_configuration = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        r#"{
                "javascript": { "formatter": { "quoteStyle": "single" } },
                "overrides": [{
                    "includes": ["**/*.js"],
                    "javascript": { "formatter": { "quoteStyle": "double" } }
                }]
            }"#,
        biome_json_parser::JsonParserOptions::default(),
        "",
    )
    .into_deserialized()
    .unwrap();
    let inline = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(PATH),
            inline_config: Some(inline_configuration),
        })
        .unwrap();
    assert!(inline.as_code().contains("\"bar\""));
    let persistent = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(PATH),
            inline_config: None,
        })
        .unwrap();
    assert!(persistent.as_code().contains("'bar'"));
}

#[test]
fn settings_query_routes_in_shared_and_owned_modes() {
    assert_settings_query_routes(DbState::default());
    assert_settings_query_routes(DbState::lsp());
}

#[test]
fn json_language_hint_preserves_path_specific_sources() {
    const BIOME_JSON: &str = r#"{"formatter": {}}"#;
    const BIOME_JSONC: &str = "{\n// comment\n\"formatter\": {},\n}";
    const PACKAGE_JSON: &str = r#"{"name":"example"}"#;

    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/biome.json"),
        BIOME_JSON.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/.biome.jsonc"),
        BIOME_JSONC.as_bytes(),
    );
    fs.insert(
        Utf8PathBuf::from("/project/package.json"),
        PACKAGE_JSON.as_bytes(),
    );
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                json: Some(JsonConfiguration {
                    formatter: Some(JsonFormatterConfiguration {
                        trailing_commas: Some(TrailingCommas::All),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    for path in [
        "/project/biome.json",
        "/project/.biome.jsonc",
        "/project/package.json",
    ] {
        workspace
            .open_file(OpenFileParams {
                project_key,
                path: BiomePath::new(path),
                content: FileContent::FromServer,
                document_file_source: Some(JsonFileSource::json().into()),
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
    }

    let source = workspace
        .get_file_source(Utf8Path::new("/project/biome.json"), false)
        .to_json_file_source()
        .unwrap();
    assert!(source.kind().is_biome_json());
    assert!(!source.allow_trailing_commas());

    let source = workspace
        .get_file_source(Utf8Path::new("/project/.biome.jsonc"), false)
        .to_json_file_source()
        .unwrap();
    assert!(source.kind().is_biome_json());
    assert!(source.allow_comments());
    assert!(source.allow_trailing_commas());

    let source = workspace
        .get_file_source(Utf8Path::new("/project/package.json"), false)
        .to_json_file_source()
        .unwrap();
    assert!(source.kind().is_package_json());

    let formatted = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new("/project/biome.json"),
            inline_config: None,
        })
        .unwrap();
    assert!(!formatted.as_code().contains("\"formatter\": {},"));

    let formatted = workspace
        .format_file(FormatFileParams {
            project_key,
            path: BiomePath::new("/project/.biome.jsonc"),
            inline_config: None,
        })
        .unwrap();
    assert!(formatted.as_code().contains("\"formatter\": {},"));
}

#[test]
fn settings_query_preserves_sequential_analyzer_rule_options() {
    const PATH: &str = "/project/file.js";
    const SOURCE: &str = "console.log('allowed');";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    let configuration = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        r#"{
            "linter": {
                "rules": {
                    "suspicious": {
                        "noConsole": {
                            "level": "error",
                            "options": { "allow": ["log"] }
                        }
                    }
                }
            },
            "overrides": [{
                "includes": ["**/*.js"],
                "linter": { "rules": { "suspicious": "on" } }
            }]
        }"#,
        biome_json_parser::JsonParserOptions::default(),
        "",
    )
    .into_deserialized()
    .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration,
            workspace_directory: Some(BiomePath::new("/project")),
            extended_configurations: Vec::new(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::from_client(SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let no_console = AnalyzerSelector::from_str("lint/suspicious/noConsole").unwrap();
    for inline_config in [None, Some(Configuration::default())] {
        let diagnostics = workspace
            .pull_diagnostics(PullDiagnosticsParams {
                project_key,
                path: BiomePath::new(PATH),
                categories: RuleCategoriesBuilder::default().with_lint().build(),
                only: vec![no_console],
                skip: Vec::new(),
                enabled_rules: vec![no_console],
                include_code_fix: false,
                inline_config,
                max_diagnostics: None,
                diagnostic_level: Severity::Hint,
                enforce_assist: false,
            })
            .unwrap();
        assert!(diagnostics.diagnostics.is_empty());
    }
}

#[test]
fn settings_query_uses_inline_analyzer_override_indices() {
    const PATH: &str = "/project/src/file.js";
    const SOURCE: &str = "debugger;\nconsole.log('value');";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    let project_configuration =
        biome_deserialize::json::deserialize_from_json_str::<Configuration>(
            r#"{
                "linter": {
                    "enabled": true,
                    "rules": {
                        "recommended": false,
                        "suspicious": { "noConsole": "warn" }
                    }
                },
                "overrides": [{
                    "includes": ["**/tests/*.js"],
                    "linter": {
                        "rules": { "suspicious": { "noDebugger": "error" } }
                    }
                }]
            }"#,
            biome_json_parser::JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: project_configuration,
            workspace_directory: Some(BiomePath::new("/project")),
            extended_configurations: Vec::new(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::from_client(SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let inline_configuration = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        r#"{
                "overrides": [{
                    "includes": ["**/src/*.js"],
                    "linter": {
                        "rules": { "suspicious": { "noConsole": "error" } }
                    }
                }]
            }"#,
        biome_json_parser::JsonParserOptions::default(),
        "",
    )
    .into_deserialized()
    .unwrap();
    let diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: Vec::new(),
            skip: Vec::new(),
            enabled_rules: Vec::new(),
            include_code_fix: false,
            inline_config: Some(inline_configuration),
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert_eq!(diagnostics.diagnostics.len(), 1);
    assert_eq!(diagnostics.diagnostics[0].severity(), Severity::Error);
}

#[test]
fn parse_capability_returns_not_found_for_unregistered_file() {
    const PATH: &str = "/project/missing.json";

    let (workspace, project_key) =
        setup_workspace_and_open_project(MemoryFileSystem::default(), "/project");
    let (_, settings, query) = {
        let db = workspace.as_workspace().get_db();
        workspace
            .as_workspace()
            .project_get_settings_query(&db, project_key, Utf8Path::new(PATH), None)
            .expect("project settings must exist")
    };
    let settings = workspace.as_workspace().settings_handle_with_query(
        &settings,
        EditorFeatures::default(),
        query,
    );
    let parse = workspace
        .features
        .get_deprecated_capabilities(JsonFileSource::json().into())
        .parser
        .parse
        .expect("JSON parser capability must exist");

    let result = parse(&BiomePath::new(PATH), &settings, workspace.get_db());

    assert!(matches!(result, Err(WorkspaceError::NotFound(_))));
}

#[cfg(feature = "html_embeds")]
#[test]
fn workspace_embeds_use_registered_source_for_custom_extension() {
    const PATH: &str = "/project/component.custom";
    const SOURCE: &str = "<script>const value = 1;</script>";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            document_file_source: Some(HtmlFileSource::html().into()),
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let snippets = workspace
        .as_workspace()
        .get_parse_snippets(project_key, &BiomePath::new(PATH))
        .unwrap();

    assert_eq!(snippets.len(), 1);
}

#[cfg(feature = "html_embeds")]
#[test]
fn workspace_svelte_snippets_use_svelte_semantics() {
    const PATH: &str = "/project/component.svelte";
    const SOURCE: &str = "<script>const store = {};</script><p>{$store}</p>";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
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
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let no_undeclared =
        AnalyzerSelector::from_str("lint/correctness/noUndeclaredVariables").unwrap();
    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![no_undeclared],
            skip: vec![],
            enabled_rules: vec![no_undeclared],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    assert!(result.diagnostics.is_empty(), "{:#?}", result.diagnostics);
}

#[cfg(all(feature = "lang_html", feature = "html_embeds"))]
#[test]
fn process_file_state_holds_one_database_snapshot() {
    const PATH: &str = "/project/file.html";
    const SOURCE: &str = "<script>const oldValue = 1;</script>";
    const UPDATED_SOURCE: &str = "<script>const newValue = 2;</script>";
    const TIMEOUT: Duration = Duration::from_secs(5);

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap()
        .project_key;
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
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
            path: BiomePath::new(PATH),
            content: FileContent::from_client(SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let attempts = workspace
        .server
        .process_file_state_parse_attempts
        .load(Ordering::Acquire);
    workspace
        .server
        .pause_process_file_state_after_parse
        .store(true, Ordering::Release);

    let (paused, setter_pending, parsed, changed) = std::thread::scope(|scope| {
        let parse_workspace = &workspace;
        let parsed = scope.spawn(move || {
            let server = parse_workspace.as_workspace();
            let db = server.get_db();
            let (_, settings, query) = server
                .project_get_settings_query(&db, project_key, Utf8Path::new(PATH), None)
                .unwrap();
            let settings =
                server.settings_handle_with_query(&settings, EditorFeatures::default(), query);
            let state = server
                .process_file_state_from_db(&BiomePath::new(PATH), &settings, &db)
                .unwrap();
            let host = state.parsed.send_node().into_source_text();
            let snippet = state
                .iter_snippets()
                .next()
                .unwrap()
                .parsed()
                .clone()
                .embedded_syntax::<JsLanguage>()
                .text_with_trivia()
                .to_string();
            (host, snippet)
        });
        let paused = wait_until(TIMEOUT, || {
            workspace
                .server
                .process_file_state_parse_attempts
                .load(Ordering::Acquire)
                > attempts
        });
        let change_workspace = &workspace;
        let changed = scope.spawn(move || {
            change_workspace.change_file(ChangeFileParams {
                project_key,
                path: BiomePath::new(PATH),
                content: UPDATED_SOURCE.into(),
                version: 1,
                inline_config: None,
                editor_features: None,
            })
        });
        let setter_pending = wait_until(TIMEOUT, || workspace.db_state.pending_setters() == 1);
        workspace
            .server
            .pause_process_file_state_after_parse
            .store(false, Ordering::Release);
        (paused, setter_pending, parsed.join(), changed.join())
    });

    assert!(
        paused,
        "workspace parsing did not reach the synchronization point"
    );
    assert!(
        setter_pending,
        "file update did not wait for the parse snapshot"
    );
    let (host, snippet) = parsed.unwrap();
    assert_eq!(host, SOURCE);
    assert!(snippet.contains("oldValue"));
    assert!(changed.unwrap().is_ok());
    assert_eq!(
        workspace
            .get_file_content(GetFileContentParams {
                project_key,
                path: BiomePath::new(PATH),
            })
            .unwrap(),
        UPDATED_SOURCE
    );
}

#[cfg(all(feature = "module_graph", feature = "lang_js"))]
#[test]
fn module_graph_primes_lint_semantic_query() {
    const PATH: &str = "/project/file.js";
    const SOURCE: &str = "const value = 1;";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let events = Events::default();
    let event_sink = events.clone();
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::with_event_handler(Box::new(move |event| {
        event_sink.0.lock().unwrap().push(event);
    }));
    let project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap()
        .project_key;
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
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
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let setup_events = take_salsa_events(&events);
    let db = workspace.get_db();
    assert!(function_query_will_execute_count_by_name(&db, "js_semantic_model", &setup_events) > 0);
    drop(db);

    let no_unused = AnalyzerSelector::from_str("lint/correctness/noUnusedVariables").unwrap();
    workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(PATH),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![no_unused],
            skip: vec![],
            enabled_rules: vec![no_unused],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        })
        .unwrap();

    let pull_events = take_salsa_events(&events);
    let db = workspace.get_db();
    assert_eq!(
        function_query_will_execute_count_by_name(&db, "js_semantic_model", &pull_events),
        0
    );
}

#[test]
fn process_file_is_stateless_and_reports_diagnostics_for_final_output() {
    const PATH: &str = "/project/file.js";
    const STORED_SOURCE: &str = "debugger;";
    const SOURCE: &str = "debugger;\nundeclared()";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), STORED_SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::from_client(STORED_SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let no_debugger = AnalyzerSelector::from_str("lint/suspicious/noDebugger").unwrap();
    let no_undeclared =
        AnalyzerSelector::from_str("lint/correctness/noUndeclaredVariables").unwrap();
    let result = workspace
        .process_file(ProcessFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::FromClient {
                content: SOURCE.into(),
                version: 1,
            },
            categories: RuleCategoriesBuilder::default()
                .with_syntax()
                .with_lint()
                .build(),
            only: vec![no_debugger, no_undeclared],
            skip: vec![],
            enabled_rules: vec![no_debugger, no_undeclared],
            fix_file_mode: Some(FixFileMode::SafeAndUnsafeFixes),
            suppression_reason: None,
            format: true,
            write: true,
            include_code_fix: true,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
            skip_parse_errors: false,
        })
        .unwrap();

    let output = result.output.unwrap();
    assert_eq!(output, "undeclared();\n");
    assert_eq!(result.applied_fixes, 1);
    assert_eq!(result.diagnostics.len(), 1);
    let span = result.diagnostics[0].location().span.unwrap();
    assert_eq!(&output[span], "undeclared");
    assert_eq!(
        workspace
            .get_file_content(GetFileContentParams {
                project_key,
                path: BiomePath::new(PATH),
            })
            .unwrap(),
        STORED_SOURCE
    );
}

/// Closing a file removes its registered source, and closing a project removes
/// the registered sources under its root without affecting other projects.
#[test]
fn close_file_and_close_project_remove_registered_file_sources() {
    const PATH_A: &str = "/project/file_a.js";
    const PATH_B: &str = "/project/nested/file_b.js";
    const OTHER_PATH: &str = "/other/file_c.js";
    const SOURCE: &str = "let a = 1;";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH_A), SOURCE.as_bytes());
    fs.insert(Utf8PathBuf::from(PATH_B), SOURCE.as_bytes());
    fs.insert(Utf8PathBuf::from(OTHER_PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    let other_project_key = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/other"),
            open_uninitialized: true,
        })
        .unwrap()
        .project_key;

    for (key, path) in [
        (project_key, PATH_A),
        (project_key, PATH_B),
        (other_project_key, OTHER_PATH),
    ] {
        workspace
            .open_file(OpenFileParams {
                project_key: key,
                path: BiomePath::new(path),
                content: FileContent::from_client(SOURCE),
                document_file_source: None,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
    }

    assert_eq!(
        workspace.get_db().file_sources_len(),
        3,
        "opening all three files must register three file sources"
    );

    workspace
        .close_file(CloseFileParams {
            project_key,
            path: BiomePath::new(PATH_A),
        })
        .unwrap();

    assert!(
        workspace.get_db().get_file(Utf8Path::new(PATH_A)).is_none(),
        "closing a file must remove its registered source"
    );
    assert!(
        workspace.get_db().get_file(Utf8Path::new(PATH_B)).is_some(),
        "closing a file must not remove another open file"
    );
    assert_eq!(
        workspace.get_db().file_sources_len(),
        2,
        "closing a file must remove only its own source"
    );

    workspace
        .close_project(CloseProjectParams { project_key })
        .unwrap();

    assert!(
        workspace.get_db().get_file(Utf8Path::new(PATH_B)).is_none(),
        "closing a project must remove registered sources under its root"
    );
    assert!(
        workspace
            .get_db()
            .get_file(Utf8Path::new(OTHER_PATH))
            .is_some(),
        "closing a project must not remove sources outside its root"
    );
    assert_eq!(
        workspace.get_db().file_sources_len(),
        1,
        "closing a project must remove only sources under its root"
    );
}

#[test]
fn process_file_preserves_embedded_content_after_formatting() {
    const PATH: &str = "/project/file.html";
    const SOURCE: &str = "<style>#id{color:red}</style><div></div>";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
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
            path: BiomePath::new(PATH),
            content: FileContent::from_client(SOURCE),
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let result = workspace
        .process_file(ProcessFileParams {
            project_key,
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            categories: RuleCategoriesBuilder::default().with_syntax().build(),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            fix_file_mode: Some(FixFileMode::SafeFixes),
            suppression_reason: None,
            format: true,
            write: true,
            include_code_fix: false,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
            skip_parse_errors: false,
        })
        .unwrap();

    let output = result.output.unwrap();
    assert!(output.contains("#id"));
    assert!(output.contains("color: red"));
    assert!(!output.contains("<style></style>"));
    assert_eq!(
        workspace
            .get_file_content(GetFileContentParams {
                project_key,
                path: BiomePath::new(PATH),
            })
            .unwrap(),
        SOURCE
    );
}

#[test]
fn fix_file_state_reparses_updated_embedded_snippets() {
    const PATH: &str = "/project/file.html";
    const SOURCE: &str = "<script>debugger;</script>";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
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
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let server = workspace.as_workspace();
    let mut state = server
        .process_file_state_from_server(project_key, &BiomePath::new(PATH))
        .unwrap();
    #[cfg(feature = "module_graph")]
    let module_db = state.db.rc_module_db();
    let no_debugger = AnalyzerSelector::from_str("lint/suspicious/noDebugger").unwrap();
    let result = server
        .with_workspace_embedded_parse_caches(Utf8Path::new(PATH), |caches| {
            server.fix_file_state(
                FixFileParams {
                    project_key,
                    path: BiomePath::new(PATH),
                    fix_file_mode: FixFileMode::SafeAndUnsafeFixes,
                    should_format: false,
                    only: vec![no_debugger],
                    skip: vec![],
                    enabled_rules: vec![no_debugger],
                    rule_categories: RuleCategoriesBuilder::default().with_lint().build(),
                    suppression_reason: None,
                    inline_config: None,
                },
                &mut state,
                caches,
                #[cfg(feature = "module_graph")]
                module_db,
                false,
            )
        })
        .unwrap()
        .unwrap();

    assert_eq!(result.actions.len(), 1);
    assert!(
        !state
            .parsed
            .send_node()
            .into_source_text()
            .contains("debugger")
    );
    assert!(state.iter_snippets().all(|snippet| {
        !snippet
            .parsed()
            .clone()
            .embedded_syntax::<JsLanguage>()
            .text_with_trivia()
            .to_string()
            .contains("debugger")
    }));
}

#[test]
fn change_file_resumes_module_update_after_cancellation() {
    const BASE_PATH: &str = "/project/base.ts";
    const INDEX_PATH: &str = "/project/index.ts";
    const OLD_BASE: &str = "export function task(): void {}";
    const NEW_BASE: &str = "export async function task(): Promise<void> {}";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(BASE_PATH), OLD_BASE.as_bytes());
    fs.insert(
        Utf8PathBuf::from(INDEX_PATH),
        b"import { task } from './base';\ntask();",
    );
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
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
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(BASE_PATH),
            content: FileContent::FromClient {
                content: OLD_BASE.into(),
                version: 1,
            },
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let initial_diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new(INDEX_PATH),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noFloatingPromises").unwrap()],
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
    assert!(initial_diagnostics.diagnostics.is_empty());

    let params = ChangeFileParams {
        project_key,
        path: BiomePath::new(BASE_PATH),
        content: NEW_BASE.into(),
        version: 2,
        inline_config: None,
        editor_features: None,
    };
    workspace
        .server
        .cancel_change_file_after_document_update
        .store(true, Ordering::Release);

    let cancelled = salsa::Cancelled::catch(AssertUnwindSafe(|| {
        workspace.as_workspace().change_file(params.clone())
    }));
    assert!(matches!(cancelled, Err(salsa::Cancelled::PendingWrite)));

    let db = workspace.get_db();
    let module = db
        .module_for_path(Utf8Path::new(BASE_PATH))
        .expect("base module must remain registered");
    let ModuleInfoKind::Js(js_info) = module.kind(&db) else {
        panic!("base module must be JavaScript");
    };
    assert!(!format!("{:?}", js_info.raw_types).contains("Promise"));
    drop(db);

    workspace.change_file(params).unwrap();

    let db = workspace.get_db();
    let module = db
        .module_for_path(Utf8Path::new(BASE_PATH))
        .expect("base module must remain registered");
    let ModuleInfoKind::Js(js_info) = module.kind(&db) else {
        panic!("base module must be JavaScript");
    };
    assert!(format!("{:?}", js_info.raw_types).contains("Promise"));
    drop(db);

    let diagnostics = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            path: BiomePath::new(INDEX_PATH),
            only: vec![AnalyzerSelector::from_str("lint/nursery/noFloatingPromises").unwrap()],
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
    assert_eq!(diagnostics.diagnostics.len(), 1);
}

#[test]
fn owned_scan_uses_replacement_updates() {
    const PATH: &str = "/project/index.js";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), b"export const value = 1;");
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .unwrap();

    let scan = || {
        workspace
            .scan_project(ScanProjectParams {
                project_key,
                watch: false,
                force: true,
                scan_kind: ScanKind::Project,
                verbose: false,
            })
            .unwrap();
    };

    scan();
    let first = workspace.get_db().get_file(Utf8Path::new(PATH)).unwrap();
    scan();
    let second = workspace.get_db().get_file(Utf8Path::new(PATH)).unwrap();

    assert_ne!(first.as_id(), second.as_id());
}

#[test]
fn scanner_epoch_queues_setters_without_cancelling_scan() {
    const PATH: &str = "/project/package.json";
    const TIMEOUT: Duration = Duration::from_secs(5);

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), b"{}");
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    let test_state = &workspace.server.scanner_test_state;
    test_state
        .pause_file_settings_read
        .store(true, Ordering::Release);
    let initial_setter_attempts = workspace.db_state.setter_gate_attempts();

    let (scan_paused, setter_queued, pending_setters, scan_result, update_result) =
        std::thread::scope(|scope| {
            let scan_workspace = &workspace;
            let scan = scope.spawn(move || {
                scan_workspace
                    .as_workspace()
                    .scan_project(ScanProjectParams {
                        project_key,
                        watch: false,
                        force: true,
                        scan_kind: ScanKind::Project,
                        verbose: false,
                    })
            });
            let scan_paused = wait_until(TIMEOUT, || {
                test_state
                    .file_settings_read_attempts
                    .load(Ordering::Acquire)
                    >= 1
            });

            let update = scan_paused.then(|| {
                let update_workspace = &workspace;
                scope.spawn(move || {
                    update_workspace
                        .as_workspace()
                        .update_settings(UpdateSettingsParams {
                            project_key,
                            workspace_directory: Some(BiomePath::new("/project")),
                            configuration: Configuration::default(),
                            extended_configurations: vec![],
                            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
                        })
                })
            });
            let setter_queued = update.as_ref().is_some_and(|_| {
                wait_until(TIMEOUT, || {
                    workspace.db_state.setter_gate_attempts() > initial_setter_attempts
                })
            });
            let pending_setters = workspace.db_state.pending_setters();

            test_state
                .pause_file_settings_read
                .store(false, Ordering::Release);
            let scan_result = scan.join().unwrap();
            let update_result = update.map(|update| update.join().unwrap());

            (
                scan_paused,
                setter_queued,
                pending_setters,
                scan_result,
                update_result,
            )
        });

    assert!(scan_paused, "the scanner did not reach the settings read");
    assert!(
        setter_queued,
        "the settings update did not reach the setter gate"
    );
    assert_eq!(
        pending_setters, 0,
        "a setter queued behind the scanner epoch must not cancel reads"
    );
    assert!(scan_result.is_ok());
    assert!(update_result.is_some_and(|result| result.is_ok()));
    assert_eq!(
        test_state.project_scan_attempts.load(Ordering::Acquire),
        1,
        "the project scan should not restart"
    );
    assert_eq!(
        test_state.file_index_attempts.load(Ordering::Acquire),
        1,
        "the file indexing operation should not restart"
    );
    assert_eq!(
        test_state.file_commit_attempts.load(Ordering::Acquire),
        1,
        "the parsed file should be committed once"
    );
}

#[test]
fn incremental_index_retries_pending_write() {
    const PATH: &str = "/project/package.json";
    const TIMEOUT: Duration = Duration::from_secs(5);

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), b"{}");
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration::default(),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    let retained_db = workspace.get_db();
    let test_state = &workspace.server.scanner_test_state;
    test_state
        .pause_file_settings_read
        .store(true, Ordering::Release);
    let initial_index_attempts = test_state.file_index_attempts.load(Ordering::Acquire);
    let initial_settings_read_attempts = test_state
        .file_settings_read_attempts
        .load(Ordering::Acquire);

    let (setter_pending, first_attempt_paused, retry_observed, update_result, index_result) =
        std::thread::scope(|scope| {
            let update_workspace = &workspace;
            let update = scope.spawn(move || {
                update_workspace
                    .as_workspace()
                    .update_settings(UpdateSettingsParams {
                        project_key,
                        workspace_directory: Some(BiomePath::new("/project")),
                        configuration: Configuration::default(),
                        extended_configurations: vec![],
                        module_graph_resolution_kind: ModuleGraphResolutionKind::None,
                    })
            });
            let setter_pending = wait_until(TIMEOUT, || workspace.db_state.pending_setters() == 1);

            let index = setter_pending.then(|| {
                let index_workspace = workspace.as_workspace();
                scope.spawn(move || {
                    WorkspaceScannerBridge::index_file(
                        &index_workspace,
                        project_key,
                        BiomePath::new(PATH),
                        IndexTrigger::Update,
                    )
                })
            });
            let first_attempt_paused = index.as_ref().is_some_and(|_| {
                wait_until(TIMEOUT, || {
                    test_state
                        .file_settings_read_attempts
                        .load(Ordering::Acquire)
                        > initial_settings_read_attempts
                })
            });
            test_state
                .pause_file_settings_read
                .store(false, Ordering::Release);
            let retry_observed = first_attempt_paused
                && wait_until(TIMEOUT, || {
                    test_state.file_index_attempts.load(Ordering::Acquire)
                        >= initial_index_attempts + 2
                });

            drop(retained_db);
            let update_result = update.join();
            let index_result = index.map(|index| index.join());

            (
                setter_pending,
                first_attempt_paused,
                retry_observed,
                update_result,
                index_result,
            )
        });

    assert!(setter_pending, "the settings update did not become pending");
    assert!(first_attempt_paused, "incremental indexing did not start");
    assert!(retry_observed, "incremental indexing was not retried");
    assert!(matches!(update_result, Ok(Ok(_))));
    assert!(matches!(index_result, Some(Ok(Ok(_)))));
    assert!(workspace.get_db().get_file(Utf8Path::new(PATH)).is_some());
}

#[test]
fn retrying_workspace_does_not_retry_project_scan() {
    let fs = MemoryFileSystem::default();
    let (watcher_tx, _) = crossbeam::channel::unbounded();
    let (service_tx, _) = tokio::sync::watch::channel(ServiceNotification::IndexUpdated);
    let mut workspace = LocalWorkspace::new(
        Arc::new(fs),
        watcher_tx,
        service_tx,
        Arc::new(NoopQueryProvider {}),
        None,
    );
    workspace.db_state = DbState::lsp();
    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new("/project"),
            open_uninitialized: true,
        })
        .unwrap();
    workspace
        .server
        .scanner_test_state
        .cancel_first_scan_attempt
        .store(true, Ordering::Release);

    let result = salsa::Cancelled::catch(AssertUnwindSafe(|| {
        crate::workspace::RetryingWorkspace::new(workspace.as_workspace()).scan_project(
            ScanProjectParams {
                project_key,
                watch: false,
                force: true,
                scan_kind: ScanKind::Project,
                verbose: false,
            },
        )
    }));

    assert!(matches!(result, Err(salsa::Cancelled::PendingWrite)));
    assert_eq!(
        workspace
            .server
            .scanner_test_state
            .project_scan_attempts
            .load(Ordering::Acquire),
        1,
        "a project scan must not be retried from the beginning"
    );
}

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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    match workspace.get_parse(project_key, &BiomePath::new("/project/a.js")) {
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let db = workspace.get_db();
    let snippets = workspace.get_snippets(BiomePath::new("/project/file.html").as_path());
    let document = db.get_file(Utf8Path::new("/project/file.html"));

    assert!(document.is_some());
    let scripts: Vec<_> = snippets
        .iter()
        .filter(|node| {
            node.document_source_index()
                .and_then(|index| db.source_from_index(index))
                .is_some_and(|source| source.is_javascript_like())
        })
        .collect();
    let styles: Vec<_> = snippets
        .iter()
        .filter(|node| {
            node.document_source_index()
                .and_then(|index| db.source_from_index(index))
                .is_some_and(|source| source.is_css_like())
        })
        .collect();
    assert_eq!(scripts.len(), 1);
    assert_eq!(styles.len(), 1);

    let script = scripts.first().unwrap();
    let style = styles.first().unwrap();

    let script_node = script.parsed();
    assert!(
        script_node
            .unwrap_as_embedded_syntax_node()
            .into_node::<JsLanguage>()
            .text_range_with_trivia()
            .start()
            > TextSize::from(0)
    );

    let style_node = style.parsed();
    assert!(
        style_node
            .unwrap_as_embedded_syntax_node()
            .into_node::<CssLanguage>()
            .text_range_with_trivia()
            .start()
            > TextSize::from(0)
    );
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let ts_file_source = workspace.get_file_source("/project/a.ts".into(), false);
    let ts = ts_file_source.to_js_file_source().expect("JS file source");
    assert!(ts.is_typescript());
    assert!(!ts.is_jsx());
    match workspace.get_parse(project_key, &BiomePath::new("/project/a.ts")) {
        Ok(parse) => assert_eq!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }

    let js_file_source = workspace.get_file_source("/project/a.js".into(), false);
    let js = js_file_source.to_js_file_source().expect("JS file source");
    assert!(!js.is_typescript());
    assert!(js.is_jsx());
    match workspace.get_parse(project_key, &BiomePath::new("/project/a.js")) {
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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let js_file_source = workspace.get_file_source("/project/a.js".into(), false);
    let js = js_file_source.to_js_file_source().expect("JS file source");
    assert!(!js.is_typescript());
    assert!(!js.is_jsx());
    match workspace.get_parse(project_key, &BiomePath::new("/project/a.js")) {
        Ok(parse) => assert_ne!(parse.diagnostics().len(), 0),
        Err(error) => panic!("File not available: {error}"),
    }

    let jsx_file_source = workspace.get_file_source("/project/a.jsx".into(), false);
    let jsx = jsx_file_source.to_js_file_source().expect("JS file source");
    assert!(!jsx.is_typescript());
    assert!(jsx.is_jsx());
    match workspace.get_parse(project_key, &BiomePath::new("/project/a.jsx")) {
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
fn astro_attribute_expression_accepts_tsx() {
    const FILE_CONTENT: &str = r#"---
import Icon from './Icon.astro';
const total = 1;
---
<Icon count={total as number} on={(e: Event) => e} icon={<Icon />} />
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
        "Expected no diagnostics for TSX syntax in an Astro attribute expression, got: {:#?}",
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
fn stores_string_jsx_style_attributes_as_css_snippets() {
    const FILE_PATH: &str = "/project/file.jsx";
    const FILE_CONTENT: &str = r#"const Valid = <div style="color: red" />;
const Component = <Component style="color: orange" />;
const Namespaced = <div css:style="color: purple" />;
const Malformed = <. style="color: black" />;
const Expression = <div style={"color: blue"} />;
const Object = <div style={{ color: "green" }} />;
const Empty = <div style />;"#;

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
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let db = workspace.get_db();
    let snippets = workspace.get_snippets(Utf8Path::new(FILE_PATH));
    let style_snippets = snippets
        .iter()
        .filter(|snippet| {
            snippet
                .document_source_index()
                .and_then(|index| db.source_from_index(index))
                .and_then(|source| source.to_css_file_source())
                .is_some_and(|source| {
                    matches!(
                        source.as_embedding_kind(),
                        CssEmbeddingKind::HtmlStyleAttribute
                    )
                })
        })
        .collect::<Vec<_>>();

    assert_eq!(style_snippets.len(), 1);
    assert_eq!(
        style_snippets[0]
            .parsed()
            .clone()
            .embedded_syntax::<CssLanguage>()
            .text_with_trivia()
            .to_string(),
        "color: red"
    );
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

#[test]
fn go_to_definition_cursor_before_embedded_script_does_not_underflow() {
    const HTML_CONTENT: &str = "\
<div>foo</div>
<script>
const x = 1;
</script>
";
    let fs = MemoryFileSystem::default();
    fs.insert(
        Utf8PathBuf::from("/project/index.html"),
        HTML_CONTENT.as_bytes(),
    );

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
            path: BiomePath::new("/project/index.html"),
            content: FileContent::FromServer,
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    // Cursor at offset 0: inside `<div>`, before the embedded `<script>` content.
    let cursor_range = TextRange::new(TextSize::from(0), TextSize::from(0));

    // Must not panic with `attempt to subtract with overflow`.
    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new("/project/index.html"),
            cursor_range,
        })
        .unwrap();

    // There is nothing to resolve before the script, so no definition is expected.
    assert!(
        result.is_none_or(|definition| definition.matches.is_empty()),
        "cursor before an embedded script should not resolve to any definition"
    );
}

#[test]
#[cfg(feature = "js_plugin")]
fn typescript_plugin_reports_diagnostics_through_the_workspace() {
    use biome_plugin_loader::{PluginConfiguration, Plugins};

    const PLUGIN_PATH: &str = "/project/plugin.ts";
    const PLUGIN_SOURCE: &str = r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
import type { AnyJsRoot, Severity } from "@biomejs/plugin-api";

export const noTopLevelVar = defineRule({
    query: ast("JS_MODULE"),
    run(root: AnyJsRoot): void {
        for (const item of root.items) {
            if (
                item.kind === "JS_VARIABLE_STATEMENT" &&
                item.declaration?.kindToken === "var"
            ) {
                registerDiagnostic(
                    item,
                    "warning" satisfies Severity,
                    "Use let or const instead of a top-level var declaration.",
                );
            }
        }
    },
});"#;
    const FILE_PATH: &str = "/project/file.ts";
    const FILE_CONTENT: &str = "var foo: number = 1;\nexport const bar: string = `${foo}`;\n";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PLUGIN_PATH), PLUGIN_SOURCE);
    fs.insert(Utf8PathBuf::from(FILE_PATH), FILE_CONTENT);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/project");

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration: Configuration {
                plugins: Some(Plugins(vec![PluginConfiguration::Path(
                    "plugin.ts".to_string(),
                )])),
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

    let diagnostics = format!("{:?}", result.diagnostics);
    assert!(
        diagnostics.contains("top-level var declaration"),
        "Expected a diagnostic from the TypeScript plugin, got: {diagnostics}"
    );
}

#[test]
fn go_to_definition_on_vue_directive_quote_does_not_underflow() {
    const PATH: &str = "/project/file.vue";
    const SOURCE: &str = "<template><button @click=\"handler()\"></button></template>";

    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(PATH), SOURCE.as_bytes());
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
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
            path: BiomePath::new(PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    let quote = TextSize::from(SOURCE.find("\"handler").unwrap() as u32);
    let result = workspace
        .go_to_definition(GoToDefinitionParams {
            project_key,
            enabled: true,
            path: BiomePath::new(PATH),
            cursor_range: TextRange::new(quote, quote),
        })
        .unwrap();

    assert!(result.is_none_or(|definition| definition.matches.is_empty()));
}
