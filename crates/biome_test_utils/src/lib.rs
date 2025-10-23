#![deny(clippy::use_self)]

use std::ffi::c_int;
use std::fmt::Write;
use std::sync::{Arc, Once};

use biome_analyze::options::{JsxRuntime, PreferredQuote};
use biome_analyze::{AnalyzerAction, AnalyzerConfiguration, AnalyzerOptions};
use biome_configuration::Configuration;
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic};
use biome_fs::{BiomePath, FileSystem, OsFileSystem};
use biome_js_parser::{AnyJsRoot, JsFileSource, JsParserOptions};
use biome_js_type_info::{TypeData, TypeResolver};
use biome_json_parser::{JsonParserOptions, ParseDiagnostic};
use biome_module_graph::ModuleGraph;
use biome_package::{PackageJson, TsConfigJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::{Direction, Language, SyntaxKind, SyntaxNode, SyntaxSlot};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::projects::Projects;
use biome_service::settings::{ServiceLanguage, Settings};
use biome_string_case::StrLikeExtension;
use camino::{Utf8Path, Utf8PathBuf};
use json_comments::StripComments;
use similar::{DiffableStr, TextDiff};

mod bench_case;

pub use bench_case::BenchCase;

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
    diagnostics: &mut Vec<String>,
) -> AnalyzerOptions {
    let options = AnalyzerOptions::default().with_file_path(input_file.to_path_buf());
    // We allow a test file to configure its rule using a special
    // file with the same name as the test but with extension ".options.json"
    // that configures that specific rule.
    let analyzer_configuration = AnalyzerConfiguration::default()
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

        options.with_configuration(analyzer_configuration)
    } else {
        let configuration = deserialized.into_deserialized().unwrap_or_default();

        let mut settings = Settings::default();
        settings
            .merge_with_configuration(configuration, None)
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
        let mut settings = projects.get_root_settings(key).unwrap_or_default();
        settings
            .merge_with_configuration(configuration, None)
            .unwrap();

        let document_file_source = DocumentFileSource::from_path(
            input_file,
            settings.experimental_full_html_support_enabled(),
        );
        settings.format_options::<L>(&input_file.into(), &document_file_source)
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
    let paths = get_js_like_paths_in_dir(&dir);
    let fs = OsFileSystem::new(dir);
    let paths = get_added_paths(&fs, &paths);

    module_graph.update_graph_for_js_paths(&fs, project_layout, &paths, &[]);

    Arc::new(module_graph)
}

/// Loads and parses files from the file system to pass them to service methods.
pub fn get_added_paths<'a>(
    fs: &dyn FileSystem,
    paths: &'a [BiomePath],
) -> Vec<(&'a BiomePath, AnyJsRoot)> {
    paths
        .iter()
        .filter_map(|path| {
            let root = fs.read_file_from_path(path).ok().and_then(|content| {
                let file_source = JsFileSource::try_from(path.as_path()).unwrap_or_default();
                let parsed =
                    biome_js_parser::parse(&content, file_source, JsParserOptions::default());
                let diagnostics = parsed.diagnostics();
                assert!(
                    diagnostics.is_empty(),
                    "Unexpected diagnostics: {diagnostics:?}\nWhile parsing:\n{content}"
                );
                parsed.try_tree()
            })?;
            Some((path, root))
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

    let package_json_file = input_file.with_extension("package.json");
    if let Ok(json) = std::fs::read_to_string(&package_json_file) {
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
        let deserialized = biome_deserialize::json::deserialize_from_json_str::<TsConfigJson>(
            json.as_str(),
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            "",
        );
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
