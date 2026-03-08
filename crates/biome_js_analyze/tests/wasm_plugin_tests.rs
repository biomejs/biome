//! Analyzer pipeline integration tests for the booleanNaming WASM plugin.
//!
//! These tests load the compiled `.wasm` plugin fixture, run it through the
//! full `biome_js_analyze::analyze` pipeline, and verify diagnostics.

use biome_analyze::{AnalysisFilter, AnalyzerPluginSlice, ControlFlow, Never, RuleFilter};
use biome_diagnostics::advice::CodeSuggestionAdvice;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_plugin_loader::AnalyzerWasmPlugin;
use biome_test_utils::{create_analyzer_options, diagnostic_to_string};
use camino::Utf8Path;
use std::path::Path;
use std::slice;
use std::sync::Arc;

fn fixture_path(name: &str) -> camino::Utf8PathBuf {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../biome_plugin_loader/tests/fixtures")
        .join(name);
    camino::Utf8PathBuf::try_from(p).unwrap()
}

/// Run the analyzer with a WASM plugin and return (diagnostics, code_fixes).
fn run_wasm_plugin(wasm_name: &str, source: &str) -> (Vec<String>, Vec<String>) {
    let plugin_path = fixture_path(wasm_name);
    let loaded = AnalyzerWasmPlugin::load(plugin_path.as_ref(), wasm_name, None)
        .unwrap_or_else(|e| panic!("Failed to load {wasm_name}: {e:?}"));

    let source_type = JsFileSource::js_module();
    let parsed = parse(source, source_type, JsParserOptions::default());
    let root = parsed.tree();

    // Enable at least one rule so the PhaseRunner is activated (needed for
    // suppression comment parsing).
    let rule_filter = RuleFilter::Rule("nursery", "noCommonJs");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let plugin_arcs: Vec<Arc<dyn biome_analyze::AnalyzerPlugin>> =
        loaded.into_iter().map(|p| Arc::new(p) as _).collect();
    let plugins: AnalyzerPluginSlice = &plugin_arcs;
    let mut diagnostics = Vec::new();
    let input_file = Utf8Path::new("test.js");
    let mut diag_options = Vec::new();
    let working_directory = input_file.parent().unwrap_or(input_file);
    let options = create_analyzer_options::<JsLanguage>(input_file, working_directory, &mut diag_options);
    let services = JsAnalyzerServices::from((Default::default(), Default::default(), source_type));

    let mut code_fixes = Vec::new();

    let (_, errors) =
        biome_js_analyze::analyze(&root, filter, &options, plugins, services, |event| {
            if let Some(mut diag) = event.diagnostic() {
                for action in event.actions() {
                    if !action.is_suppression() {
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                }
                diagnostics.push(diagnostic_to_string("test.js", source, diag.into()));
                return ControlFlow::Continue(());
            }

            for action in event.actions() {
                if !action.category.matches("quickfix.suppressRule") {
                    code_fixes.push(biome_test_utils::code_fix_to_string(source, action));
                }
            }

            ControlFlow::<Never>::Continue(())
        });

    for error in errors {
        diagnostics.push(diagnostic_to_string("test.js", source, error));
    }

    (diagnostics, code_fixes)
}

// ---------------------------------------------------------------
// booleanNaming: detection
// ---------------------------------------------------------------

#[test]
fn wasm_plugin_boolean_naming_detects_bad_names() {
    let source = "const enabled = true;\nconst isReady = false;\nconst count = 42;\n";
    let (diags, _fixes) = run_wasm_plugin("boolean_naming.wasm", source);

    // Should detect 1 diagnostic for `enabled = true` (bad name).
    // `isReady = false` has a good prefix. `count = 42` is not boolean.
    assert_eq!(
        diags.len(),
        1,
        "Expected 1 diagnostic for 'enabled = true', got {}:\n{}",
        diags.len(),
        diags.join("\n"),
    );
    assert!(
        diags[0].contains("enabled"),
        "Diagnostic should mention 'enabled':\n{}",
        diags[0],
    );
}

#[test]
fn wasm_plugin_boolean_naming_detects_comparison() {
    let source = "const equal = a === b;\nconst isSmaller = x < y;\n";
    let (diags, _fixes) = run_wasm_plugin("boolean_naming.wasm", source);

    // Only `equal = a === b` should trigger; `isSmaller` has a valid prefix.
    assert_eq!(
        diags.len(),
        1,
        "Expected 1 diagnostic for 'equal = a === b', got {}:\n{}",
        diags.len(),
        diags.join("\n"),
    );
}

#[test]
fn wasm_plugin_boolean_naming_no_false_positives() {
    let source = "const isActive = true;\nconst hasPermission = false;\nconst count = 42;\nconst name = \"hello\";\n";
    let (diags, _fixes) = run_wasm_plugin("boolean_naming.wasm", source);

    assert!(
        diags.is_empty(),
        "Expected no diagnostics for valid names, got {}:\n{}",
        diags.len(),
        diags.join("\n"),
    );
}

// ---------------------------------------------------------------
// Suppression comments
// ---------------------------------------------------------------

#[test]
fn wasm_plugin_suppression_comment_does_not_crash() {
    // Verify that a suppression comment targeting a plugin rule doesn't crash
    // the analyzer. Full suppression support for plugin rules may be wired in
    // a future change.
    let source = "// biome-ignore plugin/booleanNaming: reason\nconst enabled = true;\n";
    let (diags, _fixes) = run_wasm_plugin("boolean_naming.wasm", source);

    // The analyzer should not crash. If we get here without panicking, the test
    // passes. Suppression may or may not be effective depending on wiring.
    let _ = &diags;
}
