use biome_js_syntax::JsFileSource;
use biome_react_compiler_conformance::{biome_diag_keys, oxc_diag_keys};

/// Smoke test: a conditionally-called hook must be flagged identically by both
/// the Biome frontend and the OXC oracle. Proves the differential bridge works
/// end to end (parse -> convert -> compile -> diagnostics) on both sides.
#[test]
fn conditional_hook_parity() {
    let source = r#"import {useState} from 'react';

function Component(props) {
  if (props.enabled) {
    useState(0);
  }
  return <div />;
}
"#;

    let biome = biome_diag_keys(source, JsFileSource::jsx()).expect("biome conversion should succeed");
    let oxc = oxc_diag_keys(source, oxc_span::SourceType::jsx());

    assert!(
        !oxc.is_empty(),
        "oracle should report the conditional hook; got none"
    );
    assert_eq!(biome, oxc, "biome diagnostics should match the OXC oracle");
}
