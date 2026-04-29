use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_react_compiler::{CompileInput, compile_program, default_lint_options};

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
        println!("CASE {name}");
        match result {
            Ok(output) => {
                println!("diagnostics: {}", output.diagnostics.len());
                for diagnostic in output.diagnostics {
                    println!("{diagnostic}");
                }
            }
            Err(error) => println!("ERR {error}"),
        }
    }
}
