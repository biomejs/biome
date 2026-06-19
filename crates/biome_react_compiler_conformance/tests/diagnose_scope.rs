//! Diagnostic (DEV ONLY): dump the Biome vs OXC `ScopeInfo` for a minimal
//! context-variable repro, so the local/context-classification divergence can
//! be inspected field by field.
//!
//! Run with:
//!   cargo test -p biome_react_compiler_conformance --test diagnose_scope -- --ignored --nocapture

use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use react_compiler_ast::scope::ScopeInfo;

const SOURCE: &str = r#"function Component() {
  let x = 0;
  const update = () => {
    x = 1;
  };
  return update(x);
}
"#;

fn dump(label: &str, info: &ScopeInfo) {
    println!("\n===== {label} =====");
    println!("program_scope: {:?}", info.program_scope);
    println!("scopes:");
    for s in &info.scopes {
        println!(
            "  {:?} parent={:?} kind={:?} bindings={:?}",
            s.id, s.parent, s.kind, s.bindings
        );
    }
    println!("bindings:");
    for b in &info.bindings {
        println!(
            "  {:?} name={:?} kind={:?} scope={:?} decl_start={:?} decl_node_id={:?}",
            b.id, b.name, b.kind, b.scope, b.declaration_start, b.declaration_node_id
        );
    }
    println!("ref_node_id_to_binding: {:?}", info.ref_node_id_to_binding);
    println!("node_id_to_scope: {:?}", info.node_id_to_scope);
}

#[test]
#[ignore = "dev-only scope diagnostic"]
fn diagnose_scope() {
    println!("SOURCE:\n{SOURCE}");

    // --- Biome side ---
    let source_type = JsFileSource::jsx();
    let parsed = parse(SOURCE, source_type, JsParserOptions::default());
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));
    let biome = biome_react_compiler::convert_scope_info(biome_react_compiler::ScopeInput {
        model: &model,
    })
    .expect("biome scope conversion");
    dump("BIOME", &biome);

    // --- OXC side ---
    let allocator = oxc_allocator::Allocator::default();
    let oxc_parsed =
        oxc_parser::Parser::new(&allocator, SOURCE, oxc_span::SourceType::jsx()).parse();
    let semantic = oxc_semantic::SemanticBuilder::new()
        .build(&oxc_parsed.program)
        .semantic;
    let oxc = react_compiler_oxc::convert_scope::convert_scope_info(&semantic, &oxc_parsed.program);
    dump("OXC", &oxc);
}
