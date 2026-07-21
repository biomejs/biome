use std::slice;
use std::sync::Arc;

use biome_analyze::{
    AnalysisFilter, AnalyzerOptions, AnalyzerPlugin, ControlFlow, Never, RuleFilter, profiling,
};
use biome_fs::MemoryFileSystem;
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_languages::JsFileSource;
use biome_plugin_loader::AnalyzerGritPlugin;
use camino::Utf8Path;

/// Rule timings must be attributed to each plugin individually
/// (`plugin/<name>`) in `--profile-rules` output, instead of being aggregated
/// under a single `plugin/plugin` label.
/// See <https://github.com/biomejs/biome/issues/10795>.
///
/// This lives in its own integration test binary because the profiler is
/// process-global state.
#[test]
fn profiles_are_split_per_plugin() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/noAssign.grit".into(),
        r#"`Object.assign($args)` where {
    register_diagnostic(span = $args, message = "Prefer object spread")
}"#,
    );
    fs.insert(
        "/noConsoleLog.grit".into(),
        r#"`console.log($msg)` where {
    register_diagnostic(span = $msg, message = "No console.log")
}"#,
    );

    let plugins: Vec<_> = ["/noAssign.grit", "/noConsoleLog.grit"]
        .iter()
        .map(|path| {
            let plugin = AnalyzerGritPlugin::load(&fs, Utf8Path::new(path), None)
                .expect("Couldn't load plugin");
            Arc::new(Box::new(plugin) as Box<dyn AnalyzerPlugin>)
        })
        .collect();

    let source = "const a = Object.assign({}, b);\nconsole.log(a);\n";
    let parsed = parse(
        source,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );
    let root = parsed.tree();

    // Enable at least one rule so that the analyzer phases run.
    let rule_filter = RuleFilter::Rule("nursery", "noCommonJs");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default().with_file_path("/test.js");
    let semantic_model = semantic_model(&root, SemanticModelOptions::default());
    let services = JsAnalyzerServices::default()
        .with_source_type(JsFileSource::js_module())
        .with_semantic_model(&semantic_model);

    profiling::reset();
    profiling::enable();
    biome_js_analyze::analyze(&root, filter, &options, &plugins, services, |_| {
        ControlFlow::<Never>::Continue(())
    });
    profiling::disable();

    let labels: Vec<String> = profiling::drain_sorted_by_total(true)
        .into_iter()
        .map(|profile| profile.label.to_string())
        .collect();

    assert!(
        labels.contains(&"plugin/noAssign".to_string()),
        "expected a profile for plugin/noAssign, got: {labels:?}"
    );
    assert!(
        labels.contains(&"plugin/noConsoleLog".to_string()),
        "expected a profile for plugin/noConsoleLog, got: {labels:?}"
    );
    assert!(
        !labels.contains(&"plugin/plugin".to_string()),
        "plugin timings should not be aggregated under plugin/plugin: {labels:?}"
    );
}
