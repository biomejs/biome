use biome_diagnostics::Diagnostic;
use biome_grit_parser::parse_grit;
use biome_grit_patterns::{
    GritQuery, GritQueryEffect, GritTargetFile, GritTargetLanguage, JsTargetLanguage, Message,
    OutputFile,
};
use biome_test_utils::register_leak_checker;
use camino::Utf8Path;
use grit_util::Range;
use std::{fs::read_to_string, path::Path};

tests_macros::gen_tests! {"tests/specs/**/*.grit", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let query_path = Path::new(input);

    let (test_name, target_lang_ext) = parse_test_path(query_path);
    if target_lang_ext == "specs" {
        panic!("the test file must be placed in the specs/<target-lang-ext>/ directory");
    }

    // We have one exception: The `specs/error/` directory
    // is for testing error diagnostics.
    if target_lang_ext == "error" {
        run_error_test(query_path, test_name);
        return;
    }

    let Some(target_lang) = GritTargetLanguage::from_extension(target_lang_ext) else {
        panic!("the test file must be placed in the specs/<target-lang-ext>/ directory, unrecognized extension: {target_lang_ext}");
    };

    let query = {
        let query = read_to_string(query_path)
            .unwrap_or_else(|err| panic!("cannot read query from {query_path:?}: {err:?}"));

        let parse_grit_result = parse_grit(&query);
        if !parse_grit_result.diagnostics().is_empty() {
            panic!(
                "cannot parse query from {query_path:?}:\n{:?}",
                parse_grit_result.diagnostics()
            );
        }

        GritQuery::from_node(
            parse_grit_result.tree(),
            None,
            target_lang.clone(),
            Vec::new(),
        )
        .unwrap_or_else(|err| panic!("cannot compile query from {query_path:?}: {err:?}"))
    };

    let target_file = {
        let target_path = format!("tests/specs/{target_lang_ext}/{test_name}.{target_lang_ext}");
        let target_path = Utf8Path::new(&target_path);
        let target_code = read_to_string(target_path)
            .unwrap_or_else(|err| panic!("failed to read code from {target_path:?}: {err:?}"));

        GritTargetFile::parse(&target_code, target_path.into(), target_lang)
    };

    let result = query
        .execute(target_file)
        .unwrap_or_else(|err| panic!("cannot execute query from {query_path:?}: {err:?}"));
    let snapshot_result = SnapshotResult::from_query_effects(result.effects);

    let snapshot = if result.logs.is_empty() {
        format!("{snapshot_result:#?}")
    } else {
        let logs = result
            .logs
            .iter()
            .map(|log| {
                format!(
                    "Message: {}Syntax: {}",
                    log.message,
                    log.syntax_tree.as_deref().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{snapshot_result:#?}\n\n## Logs\n\n{logs}")
    };

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => query_path.parent().unwrap(),
    }, {
        insta::assert_snapshot!(test_name, snapshot, test_name);
    });
}

fn run_error_test(query_path: &Path, test_name: &str) {
    let snapshot_result = {
        let query = read_to_string(query_path)
            .unwrap_or_else(|err| panic!("cannot read query from {query_path:?}: {err:?}"));

        let parse_grit_result = parse_grit(&query);
        if parse_grit_result.diagnostics().is_empty() {
            match GritQuery::from_node(
                parse_grit_result.tree(),
                None,
                GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
                Vec::new(),
            ) {
                Ok(_) => panic!("an error was expected when compiling query from {query_path:?}"),
                Err(error) => ErrorSnapshotResult {
                    diagnostics: vec![Box::new(error)],
                },
            }
        } else {
            ErrorSnapshotResult {
                diagnostics: parse_grit_result
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| {
                        let boxed: Box<dyn Diagnostic> = Box::new(diagnostic.clone());
                        boxed
                    })
                    .collect(),
            }
        }
    };

    let snapshot = format!("{snapshot_result:#?}");

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => query_path.parent().unwrap(),
    }, {
        insta::assert_snapshot!(test_name, snapshot, test_name);
    });
}

/// Tests should be in a `specs/<target-lang-extension>` directory, and each
/// test should have a `.grit` file and a matching `.<target-lang-extension>`
/// file.
///
/// Returns a `("<test-name>", "<target-lang-extension>")` tuple.
fn parse_test_path(file: &Path) -> (&str, &str) {
    let test_name = file.file_stem().unwrap();

    let target_lang_extension = file.parent().unwrap();
    let target_lang_extension = target_lang_extension.file_name().unwrap();

    (
        test_name.to_str().unwrap(),
        target_lang_extension.to_str().unwrap(),
    )
}

#[derive(Debug, Default)]
struct SnapshotResult {
    messages: Vec<Message>,
    matched_ranges: Vec<String>,
    rewritten_files: Vec<OutputFile>,
    created_files: Vec<OutputFile>,
}

impl SnapshotResult {
    fn from_query_effects(results: Vec<GritQueryEffect>) -> Self {
        let mut snapshot_result = Self::default();
        for result in results {
            match result {
                GritQueryEffect::Match(m) => {
                    snapshot_result.messages.extend(m.messages);
                    snapshot_result
                        .matched_ranges
                        .extend(m.ranges.into_iter().map(format_range));
                }
                GritQueryEffect::Rewrite(rewrite) => {
                    snapshot_result.messages.extend(rewrite.original.messages);
                    snapshot_result
                        .matched_ranges
                        .extend(rewrite.original.ranges.into_iter().map(format_range));
                    snapshot_result.rewritten_files.push(rewrite.rewritten);
                }
                GritQueryEffect::CreateFile(create_file) => {
                    if let Some(ranges) = create_file.ranges {
                        snapshot_result
                            .matched_ranges
                            .extend(ranges.into_iter().map(format_range));
                    }
                    snapshot_result.created_files.push(create_file.rewritten);
                }
            }
        }

        snapshot_result
    }
}

fn format_range(range: Range) -> String {
    format!(
        "{}:{}-{}:{}",
        range.start.line, range.start.column, range.end.line, range.end.column
    )
}

#[derive(Debug, Default)]
struct ErrorSnapshotResult {
    #[expect(unused)]
    diagnostics: Vec<Box<dyn Diagnostic>>,
}
