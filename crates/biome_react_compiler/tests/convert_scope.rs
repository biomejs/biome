use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_react_compiler::{ScopeInput, convert_scope_info};

#[test]
fn converts_basic_bindings_and_references() {
    let source = "function Component(props) { return props; }";
    let source_type = JsFileSource::jsx();
    let parsed = parse(source, source_type, JsParserOptions::default());
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type));

    let scope_info = convert_scope_info(ScopeInput { model: &model })
        .expect("expected scope conversion to succeed");

    assert!(
        scope_info
            .bindings
            .iter()
            .any(|binding| binding.name == "Component")
    );
    assert!(
        scope_info
            .bindings
            .iter()
            .any(|binding| binding.name == "props")
    );
    let component = scope_info
        .bindings
        .iter()
        .find(|binding| binding.name == "Component")
        .expect("expected Component binding");
    assert_eq!(component.declaration_start, Some(9));
    assert_eq!(component.declaration_node_id, Some(10));
    assert_eq!(
        scope_info.ref_node_id_to_binding.get(&10).copied(),
        Some(component.id)
    );
    assert!(scope_info.reference_to_binding.contains_key(&35));
    assert!(scope_info.ref_node_id_to_binding.contains_key(&36));
    assert!(!scope_info.node_id_to_scope.is_empty());
    assert_eq!(
        scope_info.resolve_reference_by_node_id(36),
        scope_info.reference_to_binding.get(&35).copied()
    );
}
