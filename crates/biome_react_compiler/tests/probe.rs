use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_languages::JsFileSource;
use biome_react_compiler::{
    CompileInput, ReactCompilerError, compile_program, default_lint_options,
};

#[test]
fn probe_diagnostics() {
    let cases = [
        (
            "non_literal_deps",
            r#"import {useMemo} from 'react';

function App({text, hasDeps}) {
  const resolvedText = useMemo(() => text.toUpperCase(), hasDeps ? null : [text]);
  return resolvedText;
}
"#,
        ),
        (
            "void_use_memo",
            r#"import {useMemo} from 'react';

function Component(props) {
  const value = useMemo(() => {
    props.onChange();
  }, []);
  return <div>{value}</div>;
}
"#,
        ),
        (
            "conditional_hook",
            r#"import {useState} from 'react';

function Component(props) {
  if (props.enabled) {
    useState(0);
  }
  return <div />;
}
"#,
        ),
        (
            "prop_mutation",
            r#"function Component(props) {
  props.value = true;
  return <div>{props.value}</div>;
}
"#,
        ),
        (
            "set_state_render",
            r#"import {useState} from 'react';

function Component() {
  const [value, setValue] = useState(0);
  setValue(1);
  return <div>{value}</div>;
}
"#,
        ),
    ];

    for (name, source) in cases {
        let source_type = JsFileSource::jsx();
        let parsed = parse(source, source_type, JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));
        let result = compile_program(CompileInput {
            root: &parsed.tree(),
            model: &model,
            source,
            source_type,
            options: default_lint_options(source),
        });
        let output = result.unwrap_or_else(|error| panic!("{name}: unexpected error: {error}"));
        assert!(
            !output.diagnostics.is_empty(),
            "{name}: expected at least one diagnostic"
        );
    }
}

/// Diagnostic ranges must be UTF-8 byte offsets into the source, even though
/// the compiler internally reports Babel-style UTF-16 positions. Non-ASCII
/// text before the offending code makes the two encodings diverge, so the
/// flagged snippet must be identical to the one from an all-ASCII source.
#[test]
fn diagnostic_ranges_are_byte_offsets_in_non_ascii_sources() {
    fn flagged_snippets(source: &str) -> Vec<&str> {
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
        .expect("compilation should succeed");
        let snippets: Vec<&str> = output
            .diagnostics
            .iter()
            .filter_map(|diagnostic| match diagnostic {
                ReactCompilerError::CompilerDiagnostic {
                    range: Some(range), ..
                } => Some(&source[*range]),
                _ => None,
            })
            .collect();
        assert!(!snippets.is_empty(), "expected a diagnostic with a range");
        snippets
    }

    let body = r#"
function Component(props) {
  props.value = true;
  return <div>{props.value}</div>;
}
"#;
    let ascii = format!("const banner = \"sparkling hello world\";\n{body}");
    let non_ascii = format!("const banner = \"✨🎉 héllo wörld ✨\";\n{body}");
    assert_eq!(flagged_snippets(&ascii), flagged_snippets(&non_ascii));
}
