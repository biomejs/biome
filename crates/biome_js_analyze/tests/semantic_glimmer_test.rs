use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_rowan::AstNode;

/// Test that semantic model includes template references for Glimmer files
#[test]
fn test_glimmer_template_references() {
    let source = r#"
import Button from './Button';
import Card from './Card';
import Dialog from './Dialog';

export default class MyComponent {
  <template>
    <div>
      <Card>
        <Button>Click me</Button>
      </Card>
    </div>
  </template>
}
"#;

    // Parse as Glimmer (GJS)
    let file_source = JsFileSource::gjs();
    let parse_result = parse(source, file_source, JsParserOptions::default());

    let root = parse_result.tree();

    // Build semantic model - this would normally be done by the analyzer
    // but we're testing directly
    let semantic_model = semantic_model(&root, SemanticModelOptions::default());

    // Now check if bindings have the right references
    // For Button and Card, we should have at least one reference (from template)
    // For Dialog, we should have zero references

    println!("Semantic model built successfully for Glimmer file");
    println!("Total bindings: {}", semantic_model.all_bindings().count());

    for binding in semantic_model.all_bindings() {
        let name = binding.tree().syntax().text_trimmed().to_string();
        let ref_count = binding.all_references().count();
        println!("  Binding '{}' has {} references", name, ref_count);

        // Button and Card should have at least 1 reference (from template)
        if name == "Button" || name == "Card" {
            assert!(
                ref_count > 0,
                "{} should have references from template usage, but has {}",
                name,
                ref_count
            );
        }

        // Dialog should have 0 references (not used anywhere)
        if name == "Dialog" {
            assert_eq!(
                ref_count, 0,
                "Dialog should have no references, but has {}",
                ref_count
            );
        }
    }
}
