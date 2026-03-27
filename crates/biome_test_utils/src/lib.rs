#![deny(clippy::use_self)]

mod bench_case;

pub use bench_case::BenchCase;
use biome_analyze::options::{JsxRuntime, PreferredQuote};
use biome_analyze::{AnalyzerAction, AnalyzerConfiguration, AnalyzerOptions, RuleCategories};
use biome_configuration::HtmlConfiguration;
use biome_configuration::analyzer::AnalyzerSelector;
use biome_configuration::{Configuration, ConfigurationPathHint};
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::AnyCssRoot;
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic};
use biome_fs::{BiomePath, FileSystem, MemoryFileSystem, OsFileSystem};
use biome_html_parser::HtmlParserOptions;
use biome_html_syntax::HtmlRoot;
use biome_js_parser::{AnyJsRoot, JsParserOptions};
use biome_js_type_info::{TypeData, TypeResolver};
use biome_json_parser::ParseDiagnostic;
use biome_module_graph::{HtmlEmbeddedContent, ModuleGraph};
use biome_package::{Manifest, PackageJson, TsConfigJson, TurboJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::{Direction, Language, SyntaxKind, SyntaxNode, SyntaxSlot};
use biome_service::Workspace;
use biome_service::WorkspaceError;
use biome_service::configuration::{LoadedConfiguration, load_configuration};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::Projects;
use biome_service::settings::{
    ModuleGraphResolutionKind, ServiceLanguage, Settings, SettingsHandle,
};
use biome_service::test_utils::setup_workspace_and_open_project;
use biome_service::workspace::{
    FileContent, OpenFileParams, PullDiagnosticsParams, ScanKind, ScanProjectParams,
    UpdateSettingsParams,
};
use biome_string_case::StrLikeExtension;
use camino::{Utf8Path, Utf8PathBuf};
use json_comments::StripComments;
use similar::{DiffableStr, TextDiff};
use std::ffi::c_int;
use std::fmt::Write;
use std::sync::{Arc, Once};

pub fn scripts_from_json(extension: &str, input_code: &str) -> Option<Vec<String>> {
    if extension == "json" || extension == "jsonc" {
        let input_code = StripComments::new(input_code.as_bytes());
        let scripts: Vec<String> = serde_json::from_reader(input_code).ok()?;
        Some(scripts)
    } else {
        None
    }
}

pub fn create_analyzer_options<L: ServiceLanguage>(
    input_file: &Utf8Path,
    working_directory: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> AnalyzerOptions {
    let options = AnalyzerOptions::default()
        .with_file_path(input_file.to_path_buf())
        .with_working_directory(working_directory);
    // We allow a test file to configure its rule using a special
    // file with the same name as the test but with extension ".options.json"
    // that configures that specific rule.
    let analyzer_configuration = AnalyzerConfiguration::default()
        .with_preferred_quote(PreferredQuote::Double)
        .with_jsx_runtime(JsxRuntime::Transparent);
    let Ok((source, loaded_configuration)) = load_configuration_for_test_file(input_file) else {
        return options.with_configuration(analyzer_configuration);
    };
    if loaded_configuration.has_errors() {
        let configuration_path = loaded_configuration.file_path.unwrap().clone();
        diagnostics.extend(
            loaded_configuration
                .diagnostics
                .into_iter()
                .map(|diagnostic| {
                    diagnostic_to_string(
                        configuration_path.file_stem().unwrap(),
                        source.as_str(),
                        diagnostic,
                    )
                })
                .collect::<Vec<_>>(),
        );

        options.with_configuration(analyzer_configuration)
    } else {
        let mut settings = Settings::default();
        settings
            .merge_with_configuration(
                loaded_configuration.configuration,
                None,
                loaded_configuration.extended_configurations,
            )
            .unwrap();

        L::resolve_analyzer_options(
            &settings,
            &L::lookup_settings(&settings.languages).linter,
            L::resolve_environment(&settings),
            &BiomePath::new(input_file),
            &DocumentFileSource::from_path(
                input_file,
                settings.experimental_full_html_support_enabled(),
            ),
            None,
        )
    }
}

pub fn load_configuration_source(
    input_file: &Utf8Path,
) -> Option<(Configuration, Vec<(Utf8PathBuf, Configuration)>)> {
    let fs = OsFileSystem::new(input_file.parent().unwrap().to_path_buf());
    let source = fs.read_file_from_path(input_file).ok()?;

    let path_hint = ConfigurationPathHint::FromUser(input_file.to_path_buf());
    let (_, loaded_configuration) = load_configuration(&fs, path_hint)
        .map(|configuration| (source, configuration))
        .ok()?;

    let LoadedConfiguration {
        configuration,
        extended_configurations,
        ..
    } = loaded_configuration;

    Some((configuration, extended_configurations))
}

/// It loads `<input_file>.options.json`
pub fn load_configuration_for_test_file(
    input_file: &Utf8Path,
) -> Result<(String, LoadedConfiguration), WorkspaceError> {
    let fs = OsFileSystem::new(input_file.parent().unwrap().to_path_buf());
    let options_file = input_file.with_extension("options.json");
    let source = fs.read_file_from_path(&options_file);
    match source {
        Ok(source) => {
            let path_hint = ConfigurationPathHint::FromUser(options_file);
            load_configuration(&fs, path_hint).map(|configuration| (source, configuration))
        }
        Err(err) => Err(err.into()),
    }
}

pub fn create_parser_options<L: ServiceLanguage>(
    input_file: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> Option<L::ParserOptions> {
    let Ok((source, loaded_configuration)) = load_configuration_for_test_file(input_file) else {
        return None;
    };

    let projects = Projects::default();
    let key = projects.insert_project(Utf8PathBuf::from(""));

    if loaded_configuration.has_errors() {
        let configuration_path = loaded_configuration.file_path.unwrap().clone();
        diagnostics.extend(
            loaded_configuration
                .diagnostics
                .into_iter()
                .map(|diagnostic| {
                    diagnostic_to_string(
                        configuration_path.file_stem().unwrap(),
                        &source,
                        diagnostic,
                    )
                })
                .collect::<Vec<_>>(),
        );

        Default::default()
    } else {
        let configuration = loaded_configuration.configuration;
        let mut settings = projects.get_root_settings(key).unwrap_or_default();
        settings
            .merge_with_configuration(
                configuration,
                None,
                loaded_configuration.extended_configurations,
            )
            .unwrap();

        let document_file_source = DocumentFileSource::from_path(
            input_file,
            settings.experimental_full_html_support_enabled(),
        );
        let handle = SettingsHandle::new(&settings, None);
        Some(handle.parse_options::<L>(&input_file.into(), &document_file_source))
    }
}

pub fn create_formatting_options<L>(
    input_file: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> L::FormatOptions
where
    L: ServiceLanguage,
{
    let projects = Projects::default();
    let key = projects.insert_project(Utf8PathBuf::from(""));

    let Ok((source, loaded_configuration)) = load_configuration_for_test_file(input_file) else {
        return Default::default();
    };
    if loaded_configuration.has_errors() {
        let configuration_path = loaded_configuration.file_path.unwrap().clone();
        diagnostics.extend(
            loaded_configuration
                .diagnostics
                .into_iter()
                .map(|diagnostic| {
                    diagnostic_to_string(
                        configuration_path.file_stem().unwrap(),
                        &source,
                        diagnostic,
                    )
                })
                .collect::<Vec<_>>(),
        );

        Default::default()
    } else {
        let configuration = loaded_configuration.configuration;
        let mut settings = projects.get_root_settings(key).unwrap_or_default();
        settings
            .merge_with_configuration(
                configuration,
                None,
                loaded_configuration.extended_configurations,
            )
            .unwrap();

        let document_file_source = DocumentFileSource::from_path(
            input_file,
            settings.experimental_full_html_support_enabled(),
        );
        let handle = SettingsHandle::new(&settings, None);
        handle.format_options::<L>(&input_file.into(), &document_file_source)
    }
}

/// Creates a module graph that is initialized for the given `input_file`.
///
/// It uses an [OsFileSystem] initialized for the directory in which the test
/// file resides and inserts all files from that directory, so that files
/// importing each other within that directory will be picked up correctly.
///
/// The `project_layout` should be initialized in advance if you want any
/// manifest files to be discovered.
pub fn module_graph_for_test_file(
    input_file: &Utf8Path,
    project_layout: &ProjectLayout,
) -> Arc<ModuleGraph> {
    let module_graph = ModuleGraph::default();

    let dir = input_file.parent().unwrap().to_path_buf();
    let fs = OsFileSystem::new(dir.clone());

    // Collect and populate JS/JSX/TS/TSX paths
    let js_paths = get_js_like_paths_in_dir(&dir);
    let js_roots = get_added_js_paths(&fs, &js_paths);
    module_graph.update_graph_for_js_paths(&fs, project_layout, &js_roots, true);

    // Collect and populate CSS paths
    let css_paths = get_css_like_paths_in_dir(&dir);
    let css_roots = get_css_added_paths(&fs, &css_paths);
    module_graph.update_graph_for_css_paths(&fs, project_layout, &css_roots, None);

    Arc::new(module_graph)
}

/// Builds a module graph for a CSS test file by scanning the directory for all
/// CSS and JS-like files, parsing them, and populating the module graph.
///
/// This enables project-domain CSS rules (e.g. `noUnusedClasses`) to work in
/// spec tests by having both sides of the cross-file reference available:
/// - CSS files provide class definitions.
/// - JS/JSX files provide `className` references.
pub fn module_graph_for_css_test_file(
    input_file: &Utf8Path,
    project_layout: &ProjectLayout,
) -> Arc<ModuleGraph> {
    let module_graph = ModuleGraph::default();

    let dir = input_file.parent().unwrap().to_path_buf();
    let fs = OsFileSystem::new(dir.clone());

    // Collect and populate CSS paths.
    let css_paths = get_css_like_paths_in_dir(Utf8Path::new(&dir));
    let css_roots = get_css_added_paths(&fs, &css_paths);
    module_graph.update_graph_for_css_paths(&fs, project_layout, &css_roots, None);

    // Also collect JS/JSX paths — they may contain `className` references.
    let js_paths = get_js_like_paths_in_dir(Utf8Path::new(&dir));
    let js_roots = get_added_js_paths(&fs, &js_paths);
    module_graph.update_graph_for_js_paths(&fs, project_layout, &js_roots, false);

    Arc::new(module_graph)
}

/// Builds a module graph for an HTML test file by opening all files in the
/// test directory through a real [`WorkspaceServer`] instance.
///
/// This mirrors production behavior exactly: the workspace server's `open_file`
/// call drives `parse_embedded_nodes`, which correctly extracts `<style>`,
/// `<script>`, and Astro frontmatter (`---...---`) blocks — including their
/// scoping semantics (Vue `<style scoped>`, Astro `<style is:global>`, etc.).
///
/// This enables project-domain HTML rules (e.g. `noUndeclaredClasses`) to work
/// in spec tests with real module graph data, identical to what the LSP/CLI
/// would compute.
pub fn module_graph_for_html_test_file(
    input_file: &Utf8Path,
    _project_layout: &ProjectLayout,
) -> Arc<ModuleGraph> {
    let dir = input_file.parent().unwrap().to_path_buf();

    // Load all files from the test directory into a MemoryFileSystem.
    let mem_fs = MemoryFileSystem::default();
    let all_files: Vec<Utf8PathBuf> = {
        let mut files = Vec::new();
        collect_all_files_in_dir(&dir, &mut files);
        files
    };
    for file_path in &all_files {
        if let Ok(content) = std::fs::read(file_path.as_std_path()) {
            mem_fs.insert(file_path.clone(), content);
        }
    }

    // Create a WorkspaceServer backed by the MemoryFileSystem.
    // The project root is the test file's directory so relative import resolution works.
    let (workspace, project_key) = setup_workspace_and_open_project(mem_fs, dir.as_str());

    // Enable experimental full HTML support so that .vue/.astro/.svelte files
    // are parsed as HTML-like (with embedded script/style extraction).
    // Also enable module graph resolution so imports are tracked.
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
            workspace_directory: None,
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .expect("can update settings");

    // Index every file through the workspace's internal indexing path.
    // This triggers parse_embedded_nodes (script/style/frontmatter extraction →
    // module graph update), identical to what the LSP/CLI scanner does.
    // We pass document_file_source explicitly with experimental_full_html_support=true
    // so that .vue/.astro/.svelte files are correctly parsed as HTML-like.
    let files_with_sources = all_files.iter().map(|file_path| {
        let biome_path = BiomePath::new(file_path.clone());
        let document_file_source = DocumentFileSource::from_well_known(file_path.as_path(), true);
        (biome_path, document_file_source)
    });
    workspace.index_files_for_test(project_key, files_with_sources);

    workspace.module_graph()
}

/// Recursively collects all file paths under `dir` into `out`.
fn collect_all_files_in_dir(dir: &Utf8Path, out: &mut Vec<Utf8PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir.as_std_path()) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(path) = Utf8PathBuf::try_from(entry.path()) else {
            continue;
        };
        if path.is_dir() {
            collect_all_files_in_dir(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn get_css_like_paths_in_dir(dir: &Utf8Path) -> Vec<BiomePath> {
    std::fs::read_dir(dir)
        .unwrap()
        .flat_map(|path| {
            let path = Utf8PathBuf::try_from(path.unwrap().path()).unwrap();
            if path.is_dir() {
                get_css_like_paths_in_dir(&path)
            } else {
                DocumentFileSource::from_well_known(&path, false)
                    .is_css_like()
                    .then(|| BiomePath::new(path))
                    .into_iter()
                    .collect()
            }
        })
        .collect()
}

/// Loads and parses files from the file system to pass them to service methods.
pub fn get_added_js_paths<'a>(
    fs: &dyn FileSystem,
    paths: &'a [BiomePath],
) -> Vec<(
    &'a BiomePath,
    AnyJsRoot,
    std::sync::Arc<biome_js_semantic::SemanticModel>,
)> {
    paths
        .iter()
        .filter_map(|path| {
            let DocumentFileSource::Js(file_source) =
                DocumentFileSource::from_path(path.as_path(), false)
            else {
                return None;
            };

            let root = fs.read_file_from_path(path).ok().and_then(|content| {
                let parsed =
                    biome_js_parser::parse(&content, file_source, JsParserOptions::default());
                let diagnostics = parsed.diagnostics();
                assert!(
                    diagnostics.is_empty(),
                    "Unexpected diagnostics: {diagnostics:?}\nWhile parsing:\n{content}"
                );
                parsed.try_tree()
            })?;

            // Build semantic model for the parsed root
            let semantic_model = biome_js_semantic::semantic_model(
                &root,
                biome_js_semantic::SemanticModelOptions::default(),
            );

            Some((path, root, std::sync::Arc::new(semantic_model)))
        })
        .collect()
}

/// Loads and parses files from the file system to pass them to service methods.
pub fn get_css_added_paths<'a>(
    fs: &dyn FileSystem,
    paths: &'a [BiomePath],
) -> Vec<(&'a BiomePath, AnyCssRoot)> {
    paths
        .iter()
        .filter_map(|path| {
            let DocumentFileSource::Css(file_source) =
                DocumentFileSource::from_path(path.as_path(), false)
            else {
                return None;
            };
            let root = fs.read_file_from_path(path).ok().map(|content| {
                let options = if file_source.is_css_modules() {
                    CssParserOptions::default().allow_css_modules()
                } else {
                    CssParserOptions::default()
                };
                let parsed = biome_css_parser::parse_css(&content, file_source, options);
                let diagnostics = parsed.diagnostics();
                assert!(
                    diagnostics.is_empty(),
                    "Unexpected diagnostics: {diagnostics:?}\nWhile parsing:\n{content}"
                );
                parsed.tree()
            })?;
            Some((path, root))
        })
        .collect()
}

/// Loads and parses files from the file system to pass them to service methods.
pub fn get_html_added_paths<'a>(
    fs: &dyn FileSystem,
    paths: &'a [BiomePath],
) -> Vec<(&'a BiomePath, HtmlRoot, Vec<HtmlEmbeddedContent>)> {
    paths
        .iter()
        .filter_map(|path| {
            let DocumentFileSource::Html(file_source) =
                DocumentFileSource::from_path(path.as_path(), false)
            else {
                return None;
            };
            let root = fs.read_file_from_path(path).ok().map(|content| {
                let parsed =
                    biome_html_parser::parse_html(&content, HtmlParserOptions::from(&file_source));
                let diagnostics = parsed.diagnostics();
                assert!(
                    diagnostics.is_empty(),
                    "Unexpected diagnostics: {diagnostics:?}\nWhile parsing:\n{content}"
                );
                parsed.tree()
            })?;
            // For test utilities, we don't parse embedded content in HTML files.
            // In real scenarios, the workspace server handles this by parsing
            // embedded blocks separately and passing them to update_graph_for_html_paths.
            Some((path, root, Vec::<HtmlEmbeddedContent>::new()))
        })
        .collect()
}

fn get_js_like_paths_in_dir(dir: &Utf8Path) -> Vec<BiomePath> {
    std::fs::read_dir(dir)
        .unwrap()
        .flat_map(|path| {
            let path = Utf8PathBuf::try_from(path.unwrap().path()).unwrap();
            if path.is_dir() {
                get_js_like_paths_in_dir(&path)
            } else {
                DocumentFileSource::from_well_known(&path, false)
                    .is_javascript_like()
                    .then(|| BiomePath::new(path))
                    .into_iter()
                    .collect()
            }
        })
        .collect()
}

pub fn project_layout_for_test_file(
    input_file: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> Arc<ProjectLayout> {
    let project_layout = ProjectLayout::default();
    let fs = OsFileSystem::new(input_file.parent().unwrap().to_path_buf());

    let package_json_file = input_file.with_extension("package.json");
    if let Ok(json) = std::fs::read_to_string(&package_json_file) {
        let deserialized = PackageJson::read_manifest(&fs, &package_json_file);
        if deserialized.has_errors() {
            diagnostics.extend(
                deserialized
                    .into_diagnostics()
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic_to_string(
                            package_json_file.file_stem().unwrap(),
                            &json,
                            diagnostic,
                        )
                    }),
            );
        } else {
            project_layout.insert_node_manifest(
                input_file
                    .parent()
                    .map(|dir_path| dir_path.to_path_buf())
                    .unwrap_or_default(),
                deserialized.into_deserialized().unwrap_or_default(),
            );
        }
    }

    let tsconfig_file = input_file.with_extension("tsconfig.json");
    if let Ok(json) = std::fs::read_to_string(&tsconfig_file) {
        let deserialized = TsConfigJson::read_manifest(&fs, &tsconfig_file);
        if deserialized.has_errors() {
            diagnostics.extend(
                deserialized
                    .into_diagnostics()
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic_to_string(tsconfig_file.file_stem().unwrap(), &json, diagnostic)
                    }),
            );
        } else {
            project_layout.insert_tsconfig(
                input_file
                    .parent()
                    .map(|dir_path| dir_path.to_path_buf())
                    .unwrap_or_default(),
                deserialized.into_deserialized().unwrap_or_default(),
            );
        }
    }

    // Try turbo.json first, then turbo.jsonc
    let turbo_json_file = input_file.with_extension("turbo.json");
    let turbo_jsonc_file = input_file.with_extension("turbo.jsonc");
    let turbo_file = if turbo_json_file.exists() {
        Some(turbo_json_file)
    } else if turbo_jsonc_file.exists() {
        Some(turbo_jsonc_file)
    } else {
        None
    };

    if let Some(turbo_file) = turbo_file
        && let Ok(json) = std::fs::read_to_string(&turbo_file)
    {
        let deserialized = TurboJson::read_manifest(&fs, &turbo_file);
        if deserialized.has_errors() {
            diagnostics.extend(
                deserialized
                    .into_diagnostics()
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic_to_string(turbo_file.file_stem().unwrap(), &json, diagnostic)
                    }),
            );
        } else {
            project_layout.insert_turbo_json(
                input_file
                    .parent()
                    .map(|dir_path| dir_path.to_path_buf())
                    .unwrap_or_default(),
                deserialized.into_deserialized().unwrap_or_default(),
            );
        }
    }

    Arc::new(project_layout)
}

pub fn diagnostic_to_string(name: &str, source: &str, diag: Error) -> String {
    let error = diag.with_file_path(name).with_file_source_code(source);
    markup_to_string(biome_console::markup! {
        {PrintDiagnostic::verbose(&error)}
    })
}

fn markup_to_string(markup: biome_console::Markup) -> String {
    let mut buffer = Vec::new();
    let mut write =
        biome_console::fmt::Termcolor(biome_diagnostics::termcolor::NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub fn dump_registered_types(content: &mut String, resolver: &dyn TypeResolver) {
    let mut registered_types = String::new();
    let mut resolver = Some(resolver);
    while let Some(current_resolver) = resolver {
        for (i, ty) in current_resolver.registered_types().iter().enumerate() {
            let level = current_resolver.level();
            registered_types.push_str(&format!("\n{level:?} TypeId({i}) => {ty}\n"));
        }

        resolver = current_resolver.fallback_resolver();
    }

    if !registered_types.is_empty() {
        content.push_str("## Registered types\n\n");

        content.push_str("```");
        content.push_str(&registered_types);
        content.push_str("```\n");
    }
}

pub fn dump_registered_module_types(content: &mut String, types: &[&TypeData]) {
    if types.is_empty() {
        return;
    }

    content.push_str("## Registered types\n\n");
    content.push_str("```");

    for (i, ty) in types.iter().enumerate() {
        content.push_str(&format!("\nModule TypeId({i}) => {ty}\n"));
    }

    content.push_str("```\n");
}

// Check that all red / green nodes have correctly been released on exit
unsafe extern "C" fn check_leaks() {
    if let Some(report) = biome_rowan::check_live() {
        panic!("\n{report}")
    }
}
pub fn register_leak_checker() {
    // Import the atexit function from libc
    unsafe extern "C" {
        fn atexit(f: unsafe extern "C" fn()) -> c_int;
    }

    // Use an atomic Once to register the check_leaks function to be called
    // when the process exits
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        countme::enable(true);
        atexit(check_leaks);
    });
}

pub fn code_fix_to_string<L: ServiceLanguage>(source: &str, action: AnalyzerAction<L>) -> String {
    let (_, text_edit) = action.mutation.to_text_range_and_edit().unwrap_or_default();

    let output = text_edit.new_string(source);

    let diff = TextDiff::from_lines(source, &output);

    let mut diff = diff.unified_diff();
    diff.context_radius(3);

    diff.to_string()
}

/// The test runner for the analyzer is currently designed to have a
/// one-to-one mapping between test case and analyzer rules.
/// So each testing file will be run through the analyzer with only the rule
/// corresponding to the directory name. E.g., `complexity/useWhile/test.js`
/// will be analyzed with just the `complexity/useWhile` rule.
pub fn parse_test_path(file: &Utf8Path) -> (&str, &str) {
    let mut group_name = "";
    let mut rule_name = "";

    for component in file.iter().rev() {
        if component == "specs" || component == "suppression" || component == "plugin" {
            break;
        }

        rule_name = group_name;
        group_name = DiffableStr::as_str(component).unwrap_or_default();
    }

    (group_name, rule_name)
}

/// This check is used in the parser test to ensure it doesn't emit
/// bogus nodes without diagnostics, and in the analyzer tests to
/// check the syntax trees resulting from code actions are correct
pub fn has_bogus_nodes_or_empty_slots<L: biome_rowan::Language>(node: &SyntaxNode<L>) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty { .. }));
        }

        false
    })
}

/// This function analyzes the parsing result of a file and panic with a
/// detailed message if it contains any error-level diagnostic, bogus nodes,
/// empty list slots or missing required children
pub fn assert_errors_are_absent<L: ServiceLanguage>(
    program: &SyntaxNode<L>,
    diagnostics: &[ParseDiagnostic],
    path: &Utf8Path,
) {
    let debug_tree = format!("{program:?}");
    let has_missing_children = debug_tree.contains("missing (required)");

    if diagnostics.is_empty() && !has_bogus_nodes_or_empty_slots(program) && !has_missing_children {
        return;
    }

    let mut buffer = Buffer::no_color();
    for diagnostic in diagnostics {
        let error = diagnostic
            .clone()
            .with_file_path(path.as_str())
            .with_file_source_code(program.to_string());
        Formatter::new(&mut Termcolor(&mut buffer))
            .write_markup(markup! {
                {PrintDiagnostic::verbose(&error)}
            })
            .unwrap();
    }

    panic!(
        "There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}\nPrinted tree:\n{}",
        path,
        std::str::from_utf8(buffer.as_slice()).unwrap(),
        &program,
        &program.to_string()
    );
}

pub fn write_analyzer_snapshot(
    snapshot: &mut String,
    input_code: &str,
    diagnostics: &[String],
    code_fixes: &[String],
    markdown_language: &str,
    parser_diagnostics: usize,
) {
    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```{markdown_language}").unwrap();
    writeln!(snapshot, "{input_code}").unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    if parser_diagnostics > 0 {
        writeln!(
            snapshot,
            "_Note: The parser emitted {parser_diagnostics} diagnostics which are not shown here._"
        )
        .unwrap();
        writeln!(snapshot).unwrap();
    }

    if !diagnostics.is_empty() {
        writeln!(snapshot, "# Diagnostics").unwrap();
        for diagnostic in diagnostics {
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot, "{diagnostic}").unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }

    if !code_fixes.is_empty() {
        writeln!(snapshot, "# Actions").unwrap();
        for action in code_fixes {
            writeln!(snapshot, "```diff").unwrap();
            writeln!(snapshot, "{action}").unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }
}

pub fn write_transformation_snapshot(
    snapshot: &mut String,
    input_code: &str,
    transformations: &[String],
    extension: &str,
) {
    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```{extension}").unwrap();
    writeln!(snapshot, "{input_code}").unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    if !transformations.is_empty() {
        writeln!(snapshot, "# Transformations").unwrap();
        for transformation in transformations {
            writeln!(snapshot, "```{extension}").unwrap();
            writeln!(snapshot, "{transformation}").unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }
}

pub enum CheckActionType {
    Suppression,
    Lint,
}

impl CheckActionType {
    pub const fn is_suppression(&self) -> bool {
        matches!(self, Self::Suppression)
    }
}

/// Validator to run in our parser's spec tests to make sure no excess data
/// is collected in the EOF token.
pub fn validate_eof_token<L: Language>(syntax: SyntaxNode<L>) {
    let last_token = syntax.last_token().expect("no tokens parsed");
    assert_eq!(
        last_token.kind(),
        L::Kind::EOF,
        "the syntax tree's last token must be an EOF token"
    );
    assert!(
        last_token.token_text_trimmed().is_empty(),
        "the EOF token may not contain any data except trailing whitespace, but found \"{}\"",
        last_token.token_text_trimmed()
    );
}

/// Asserts whether test files containing comments:
/// - `should not generate diagnostics` emit no diagnostics
/// - `should generate diagnostics` emit diagnostics
///
/// Additionally it checks that valid test files contain
/// comment enforcing no diagnostics.
///
/// ## Examples
///
/// `valid.js` file
/// ```js
/// /** should not generate diagnostics */
/// ```
/// `valid.yml` file
/// ```yaml
/// # should not generate diagnostics
/// ```
///
/// `in+valid.js` file
/// ```js
/// /** should generate diagnostics */
/// ```
///
pub fn assert_diagnostics_expectation_comment<L: Language>(
    file_path: &Utf8Path,
    syntax: &SyntaxNode<L>,
    diagnostics: Vec<String>,
) {
    let no_diagnostics_comment_text = "should not generate diagnostics";
    let diagnostics_comment_text = "should generate diagnostics";

    let is_valid_test_file = match file_path.extension().unwrap_or_default() {
        // Excluded files types which cannot contain comment in the source code
        "snap" | "json" | "jsonc" | "svelte" | "vue" | "astro" | "html" => false,
        _ => {
            let name = file_path.file_name().unwrap().to_ascii_lowercase_cow();
            // We can't know all the valid file names, but this should catch most common cases.
            name.contains("valid") && !name.contains("invalid")
        }
    };

    enum Diagnostics {
        ShouldGenerateDiagnostics,
        ShouldNotGenerateDiagnostics,
    }

    let diagnostic_comment = syntax.preorder_tokens(Direction::Next).find_map(|token| {
        for piece in token.leading_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                let text = comment.text();

                if text.contains(no_diagnostics_comment_text) {
                    return Some(Diagnostics::ShouldNotGenerateDiagnostics);
                }

                if text.contains(diagnostics_comment_text) {
                    return Some(Diagnostics::ShouldGenerateDiagnostics);
                }
            }
        }

        None
    });

    let has_diagnostics = !diagnostics.is_empty();
    match diagnostic_comment {
        Some(Diagnostics::ShouldNotGenerateDiagnostics) => {
            if has_diagnostics {
                panic!(
                    "This test should not generate diagnostics\nFile: {file_path}\n\nDiagnostics: {}",
                    diagnostics.join("\n")
                );
            }
        }
        Some(Diagnostics::ShouldGenerateDiagnostics) => {
            if !has_diagnostics {
                panic!("This test should generate diagnostics\nFile: {file_path}",);
            }
        }
        None => {
            if is_valid_test_file {
                panic!(
                    "Valid test files should contain comment `{no_diagnostics_comment_text}`\nFile: {file_path}",
                );
            }
        }
    }
}

/// Asserts diagnostic expectations by scanning the raw file content for comments.
///
/// This is the content-based equivalent of [`assert_diagnostics_expectation_comment`],
/// used for HTML-ish files (Vue, Svelte, Astro, HTML) where we don't have access
/// to the parsed syntax tree from the workspace.
///
/// Looks for both HTML comments (`<!-- should not generate diagnostics -->`) and
/// JS comments (`// should not generate diagnostics`) in the raw text.
pub fn assert_diagnostics_expectation_from_content(
    file_path: &Utf8Path,
    content: &str,
    diagnostics: Vec<String>,
) {
    let no_diagnostics_comment_text = "should not generate diagnostics";
    let diagnostics_comment_text = "should generate diagnostics";

    let is_valid_test_file = {
        let name = file_path.file_name().unwrap().to_ascii_lowercase_cow();
        name.contains("valid") && !name.contains("invalid")
    };

    enum Diagnostics {
        ShouldGenerateDiagnostics,
        ShouldNotGenerateDiagnostics,
    }

    // Search the raw content for the expectation comment
    let diagnostic_comment = if content.contains(no_diagnostics_comment_text) {
        Some(Diagnostics::ShouldNotGenerateDiagnostics)
    } else if content.contains(diagnostics_comment_text) {
        Some(Diagnostics::ShouldGenerateDiagnostics)
    } else {
        None
    };

    let has_diagnostics = !diagnostics.is_empty();
    match diagnostic_comment {
        Some(Diagnostics::ShouldNotGenerateDiagnostics) => {
            if has_diagnostics {
                panic!(
                    "This test should not generate diagnostics\nFile: {file_path}\n\nDiagnostics: {}",
                    diagnostics.join("\n")
                );
            }
        }
        Some(Diagnostics::ShouldGenerateDiagnostics) => {
            if !has_diagnostics {
                panic!("This test should generate diagnostics\nFile: {file_path}");
            }
        }
        None => {
            if is_valid_test_file {
                panic!(
                    "Valid test files should contain comment `{no_diagnostics_comment_text}`\nFile: {file_path}",
                );
            }
        }
    }
}

/// Runs lint analysis on a test file using the Workspace API.
///
/// This uses `WorkspaceServer` internally, which properly handles embedded
/// languages (e.g., JS inside Vue/Svelte/Astro/HTML files) through the full
/// HTML parser pipeline with correct offset mapping.
///
/// Returns a snapshot string in the standard analyzer snapshot format
/// (`# Input` / `# Diagnostics`).
pub fn analyze_with_workspace(
    input_file: &Utf8Path,
    input_code: String,
    group: &str,
    rule: &str,
) -> String {
    let document_file_source = DocumentFileSource::from_well_known(input_file, true);

    if document_file_source == DocumentFileSource::Unknown {
        panic!(
            "Invalid document file source: {:?}. Make sure the document is supported by Biome.",
            input_file
        );
    };
    let file_name = input_file.file_name().unwrap();
    let project_root = Utf8PathBuf::from("/test-project");
    let virtual_file_path = project_root.join(file_name);

    // Set up in-memory filesystem
    let fs = MemoryFileSystem::default();
    fs.insert(virtual_file_path.clone(), input_code.as_bytes());
    let mut files_to_index = vec![virtual_file_path.clone()];

    // Insert sidecar files if they exist on disk
    files_to_index.extend(insert_sidecar_files(&fs, input_file, &project_root));

    // Create workspace and open project
    let (workspace, project_key) = setup_workspace_and_open_project(fs, project_root.as_str());

    // Build configuration: enable full HTML support + merge .options.json if present
    let config = build_test_configuration(input_file);

    // Push configuration into the workspace
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: config,
            workspace_directory: Some(BiomePath::new(&project_root)),
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .expect("failed to update settings");

    // Scan project so package.json/tsconfig.json are discovered
    workspace
        .scan_project(ScanProjectParams {
            project_key,
            watch: false,
            force: false,
            scan_kind: ScanKind::NoScanner,
            verbose: false,
        })
        .expect("failed to scan project");

    workspace.index_files_for_test(
        project_key,
        files_to_index.into_iter().map(|path| {
            let document_file_source = DocumentFileSource::from_well_known(path.as_path(), true);
            (BiomePath::new(path), document_file_source)
        }),
    );

    // Open file
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(&virtual_file_path),
            content: FileContent::FromClient {
                content: input_code.clone(),
                version: 0,
            },
            document_file_source: Some(document_file_source),
            persist_node_cache: false,
            inline_config: None,
        })
        .expect("failed to open file");

    // Build rule selector
    let rule_selector = format!("{group}/{rule}")
        .parse::<AnalyzerSelector>()
        .unwrap_or_else(|err| panic!("failed to parse rule selector {group}/{rule}: {err}"));

    // Pull diagnostics with code actions embedded
    let result = workspace
        .pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: BiomePath::new(&virtual_file_path),
            categories: RuleCategories::default(),
            only: vec![rule_selector],
            skip: vec![],
            enabled_rules: vec![],
            pull_code_actions: true,
            inline_config: None,
        })
        .expect("failed to pull diagnostics");

    // Convert serde::Diagnostics to rendered strings
    let diagnostics: Vec<String> = result
        .diagnostics
        .into_iter()
        .map(|diag| {
            let error = Error::from(diag);
            diagnostic_to_string(file_name, &input_code, error)
        })
        .collect();

    let extension = input_file.extension().unwrap_or_default();

    let mut snapshot = String::new();
    write_analyzer_snapshot(
        &mut snapshot,
        &input_code,
        diagnostics.as_slice(),
        &[],
        extension,
        0,
    );

    assert_diagnostics_expectation_from_content(input_file, &input_code, diagnostics);

    snapshot
}

/// Builds a `Configuration` for workspace-based tests.
///
/// Enables full HTML support and merges in `.options.json` if present.
fn build_test_configuration(input_file: &Utf8Path) -> Configuration {
    let html_full_support = HtmlConfiguration {
        experimental_full_support_enabled: Some(biome_configuration::bool::Bool(true)),
        ..Default::default()
    };
    let mut config = Configuration {
        html: Some(html_full_support),
        ..Default::default()
    };

    // Load and merge .options.json if present
    let options_path = input_file.with_extension("options.json");
    if let Ok(options_content) = std::fs::read_to_string(&options_path) {
        match serde_json::from_str::<Configuration>(&options_content) {
            Ok(options_config) => {
                // Preserve our HTML full support setting, merge everything else
                let html_setting = config.html.clone();
                config = options_config;
                if config.html.is_none() {
                    config.html = html_setting;
                } else if let Some(ref mut html) = config.html
                    && html.experimental_full_support_enabled.is_none()
                {
                    html.experimental_full_support_enabled =
                        html_setting.and_then(|h| h.experimental_full_support_enabled);
                }
            }
            Err(err) => {
                panic!("failed to parse {options_path}: {err}");
            }
        }
    }

    config
}

/// Inserts sidecar files (package.json, tsconfig.json, etc.) and peer source
/// files into the `MemoryFileSystem` for workspace-based tests.
fn insert_sidecar_files(
    fs: &MemoryFileSystem,
    input_file: &Utf8Path,
    project_root: &Utf8Path,
) -> Vec<Utf8PathBuf> {
    let mut inserted_files = Vec::new();

    // Insert package.json sidecar
    let package_json_sidecar = input_file.with_extension("package.json");
    if let Ok(content) = std::fs::read_to_string(&package_json_sidecar) {
        let target_path = project_root.join("package.json");
        fs.insert(target_path.clone(), content.as_bytes());
        inserted_files.push(target_path);
    }

    // Insert tsconfig.json sidecar
    let tsconfig_sidecar = input_file.with_extension("tsconfig.json");
    if let Ok(content) = std::fs::read_to_string(&tsconfig_sidecar) {
        let target_path = project_root.join("tsconfig.json");
        fs.insert(target_path.clone(), content.as_bytes());
        inserted_files.push(target_path);
    }

    // Insert turbo.json sidecar
    let turbo_json_sidecar = input_file.with_extension("turbo.json");
    let turbo_jsonc_sidecar = input_file.with_extension("turbo.jsonc");
    if let Ok(content) = std::fs::read_to_string(&turbo_json_sidecar) {
        let target_path = project_root.join("turbo.json");
        fs.insert(target_path.clone(), content.as_bytes());
        inserted_files.push(target_path);
    } else if let Ok(content) = std::fs::read_to_string(&turbo_jsonc_sidecar) {
        let target_path = project_root.join("turbo.jsonc");
        fs.insert(target_path.clone(), content.as_bytes());
        inserted_files.push(target_path);
    }

    // Insert additional source files from the same directory for module graph rules
    let Some(parent_dir) = input_file.parent() else {
        return inserted_files;
    };
    let Ok(entries) = std::fs::read_dir(parent_dir) else {
        return inserted_files;
    };
    for entry in entries.flatten() {
        let Ok(path) = Utf8PathBuf::try_from(entry.path()) else {
            continue;
        };
        // Skip the main test file (already inserted), sidecars, and snapshots
        if path == input_file {
            continue;
        }
        let ext = path.extension().unwrap_or_default();
        if matches!(
            ext,
            "js" | "mjs"
                | "cjs"
                | "jsx"
                | "css"
                | "ts"
                | "mts"
                | "cts"
                | "tsx"
                | "vue"
                | "svelte"
                | "astro"
                | "html"
        ) && let Ok(content) = std::fs::read_to_string(&path)
        {
            let target_name = path.file_name().unwrap();
            let target_path = project_root.join(target_name);
            fs.insert(target_path.clone(), content.as_bytes());
            inserted_files.push(target_path);
        }
    }

    inserted_files
}
