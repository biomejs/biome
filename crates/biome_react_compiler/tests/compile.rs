use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_react_compiler::{CompileInput, compile_program, default_lint_options};

#[test]
fn compiler_boundary_reports_without_panicking() {
    let source = r#"function Component(props) {
    const value = props.value;
    return <div id="x">{value}</div>;
}"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));

    let _ = compile_program(CompileInput {
        root: &parsed.tree(),
        model: &model,
        source,
        source_type,
        options: default_lint_options(source),
    });
}

#[test]
fn compiler_boundary_reports_lint_events_as_diagnostics() {
    let source = r#"import {useState} from 'react';

function Component(props) {
    if (props.enabled) {
        useState(0);
    }

    return <div />;
}"#;
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));

    let output = compile_program(CompileInput {
        root: &parsed.tree(),
        model: &model,
        source,
        source_type,
        options: default_lint_options(source),
    })
    .expect("program should compile far enough to return lint diagnostics");

    assert!(output.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .to_string()
            .contains("Hooks must always be called in a consistent order")
    }));
}
