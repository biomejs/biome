use biome_analyze::options::{JsxRuntime, PreferredQuote};
use biome_analyze::{AnalyzerAction, AnalyzerConfiguration, AnalyzerOptions};
use biome_configuration::Configuration;
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_dependency_graph::DependencyGraph;
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic};
use biome_fs::{BiomePath, FileSystem, OsFileSystem};
use biome_js_parser::{JsFileSource, JsParserOptions};
use biome_json_parser::{JsonParserOptions, ParseDiagnostic};
use biome_package::PackageJson;
use biome_project_layout::ProjectLayout;
use biome_rowan::{SyntaxKind, SyntaxNode, SyntaxSlot};
use biome_service::configuration::to_analyzer_rules;
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::Projects;
use biome_service::settings::{ServiceLanguage, Settings, WorkspaceSettingsHandle};
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

pub fn create_analyzer_options(
    input_file: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> AnalyzerOptions {
    let options = AnalyzerOptions::default().with_file_path(input_file.to_path_buf());
    // We allow a test file to configure its rule using a special
    // file with the same name as the test but with extension ".options.json"
    // that configures that specific rule.
    let mut analyzer_configuration = AnalyzerConfiguration::default()
        .with_preferred_quote(PreferredQuote::Double)
        .with_jsx_runtime(JsxRuntime::Transparent);
    let options_file = input_file.with_extension("options.json");
    let Ok(json) = std::fs::read_to_string(options_file.clone()) else {
        return options.with_configuration(analyzer_configuration);
    };
    let deserialized = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        json.as_str(),
        JsonParserOptions::default(),
        "",
    );
    if deserialized.has_errors() {
        diagnostics.extend(
            deserialized
                .into_diagnostics()
                .into_iter()
                .map(|diagnostic| {
                    diagnostic_to_string(options_file.file_stem().unwrap(), &json, diagnostic)
                })
                .collect::<Vec<_>>(),
        );
    } else {
        let configuration = deserialized.into_deserialized().unwrap_or_default();
        let mut settings = Settings::default();
        analyzer_configuration = analyzer_configuration.with_preferred_quote(
            configuration
                .javascript
                .as_ref()
                .and_then(|js| js.formatter.as_ref())
                .and_then(|f| {
                    f.quote_style.map(|quote_style| {
                        if quote_style.is_double() {
                            PreferredQuote::Double
                        } else {
                            PreferredQuote::Single
                        }
                    })
                })
                .unwrap_or_default(),
        );

        use biome_configuration::javascript::JsxRuntime::*;
        analyzer_configuration = analyzer_configuration.with_jsx_runtime(
            match configuration
                .javascript
                .as_ref()
                .and_then(|js| js.jsx_runtime)
                .unwrap_or_default()
            {
                ReactClassic => JsxRuntime::ReactClassic,
                Transparent => JsxRuntime::Transparent,
            },
        );
        analyzer_configuration = analyzer_configuration.with_globals(
            configuration
                .javascript
                .as_ref()
                .and_then(|js| {
                    js.globals
                        .as_ref()
                        .map(|globals| globals.iter().cloned().collect())
                })
                .unwrap_or_default(),
        );

        settings
            .merge_with_configuration(configuration, None, None, &[])
            .unwrap();

        analyzer_configuration =
            analyzer_configuration.with_rules(to_analyzer_rules(&settings, input_file));
    }
    options.with_configuration(analyzer_configuration)
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

    let options_file = input_file.with_extension("options.json");
    let Ok(json) = std::fs::read_to_string(options_file.clone()) else {
        return Default::default();
    };
    let deserialized = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        json.as_str(),
        JsonParserOptions::default(),
        "",
    );
    if deserialized.has_errors() {
        diagnostics.extend(
            deserialized
                .into_diagnostics()
                .into_iter()
                .map(|diagnostic| {
                    diagnostic_to_string(options_file.file_stem().unwrap(), &json, diagnostic)
                })
                .collect::<Vec<_>>(),
        );

        Default::default()
    } else {
        let configuration = deserialized.into_deserialized().unwrap_or_default();
        let mut settings = projects.get_settings(key).unwrap_or_default();
        settings
            .merge_with_configuration(configuration, None, None, &[])
            .unwrap();

        let handle = WorkspaceSettingsHandle::from(settings);
        let document_file_source = DocumentFileSource::from_path(input_file);
        handle.format_options::<L>(&input_file.into(), &document_file_source)
    }
}

/// Creates a dependency graph that is initialized for the given `input_file`.
///
/// It uses an [OsFileSystem] initialized for the directory in which the test
/// file resides and inserts all files from that directory, so that files
/// importing each other within that directory will be picked up correctly.
///
/// The `project_layout` should be initialized in advance if you want any
/// manifest files to be discovered.
pub fn dependency_graph_for_test_file(
    input_file: &Utf8Path,
    project_layout: &ProjectLayout,
) -> Arc<DependencyGraph> {
    let dependency_graph = DependencyGraph::default();

    let dir = input_file.parent().unwrap().to_path_buf();
    let paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|path| {
            let path = Utf8PathBuf::try_from(path.unwrap().path()).unwrap();
            DocumentFileSource::from_well_known(&path)
                .is_javascript_like()
                .then(|| BiomePath::new(path))
        })
        .collect();
    let fs = OsFileSystem::new(dir);

    dependency_graph.update_imports_for_js_paths(&fs, project_layout, &paths, &[], |path| {
        fs.read_file_from_path(path).ok().and_then(|content| {
            let file_source = path
                .extension()
                .and_then(|extension| JsFileSource::try_from_extension(extension).ok())
                .unwrap_or_default();
            let parsed = biome_js_parser::parse(&content, file_source, JsParserOptions::default());
            parsed.try_tree()
        })
    });

    Arc::new(dependency_graph)
}

pub fn project_layout_with_node_manifest(
    input_file: &Utf8Path,
    diagnostics: &mut Vec<String>,
) -> Arc<ProjectLayout> {
    let options_file = input_file.with_extension("package.json");
    if let Ok(json) = std::fs::read_to_string(options_file.clone()) {
        let deserialized = biome_deserialize::json::deserialize_from_json_str::<PackageJson>(
            json.as_str(),
            JsonParserOptions::default(),
            "",
        );
        if deserialized.has_errors() {
            diagnostics.extend(
                deserialized
                    .into_diagnostics()
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic_to_string(options_file.file_stem().unwrap(), &json, diagnostic)
                    })
                    .collect::<Vec<_>>(),
            );
        } else {
            let project_layout = ProjectLayout::default();
            project_layout.insert_node_manifest(
                Utf8PathBuf::new(),
                deserialized.into_deserialized().unwrap_or_default(),
            );
            return Arc::new(project_layout);
        }
    }
    Default::default()
}

pub fn diagnostic_to_string(name: &str, source: &str, diag: Error) -> String {
    let error = diag.with_file_path(name).with_file_source_code(source);
    let text = markup_to_string(biome_console::markup! {
        {PrintDiagnostic::verbose(&error)}
    });

    text
}

fn markup_to_string(markup: biome_console::Markup) -> String {
    let mut buffer = Vec::new();
    let mut write =
        biome_console::fmt::Termcolor(biome_diagnostics::termcolor::NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

// Check that all red / green nodes have correctly been released on exit
extern "C" fn check_leaks() {
    if let Some(report) = biome_rowan::check_live() {
        panic!("\n{report}")
    }
}
pub fn register_leak_checker() {
    // Import the atexit function from libc
    extern "C" {
        fn atexit(f: extern "C" fn()) -> c_int;
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
    let (_, text_edit) = action.mutation.as_text_range_and_edit().unwrap_or_default();

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

    panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}\nPrinted tree:\n{}",
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
) {
    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```{markdown_language}").unwrap();
    writeln!(snapshot, "{input_code}").unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

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
