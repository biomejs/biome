use biome_js_syntax::{
    AnyJsIdentifierUsage, AnyJsRoot, JsReferenceIdentifier, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, TextRange, TokenText, WalkEvent};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Default)]
pub struct EmbeddedValueReferences {
    pub references: Vec<FxHashMap<TextRange, TokenText>>,
}

#[derive(Debug)]
pub(crate) struct EmbeddedValueReferencesBuilder {
    references: FxHashMap<TextRange, TokenText>,
}

impl EmbeddedValueReferences {
    pub(crate) fn builder(&self) -> EmbeddedValueReferencesBuilder {
        EmbeddedValueReferencesBuilder::new()
    }

    pub(crate) fn finish(&mut self, builder: EmbeddedValueReferencesBuilder) {
        self.references.push(builder.references);
    }
}

impl EmbeddedValueReferencesBuilder {
    fn new() -> Self {
        Self {
            references: FxHashMap::default(),
        }
    }

    /// Visit a non-source snippet to track value references
    pub(crate) fn visit_non_source_snippet(&mut self, root: &AnyJsRoot) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(reference) = JsReferenceIdentifier::cast_ref(&node) {
                        self.visit_reference_identifier(reference);
                    } else if let Some(member) = JsStaticMemberExpression::cast_ref(&node) {
                        self.visit_static_member_expression(member);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    fn visit_reference_identifier(&mut self, reference: JsReferenceIdentifier) -> Option<()> {
        let usage = AnyJsIdentifierUsage::from(reference.clone());
        if usage.is_only_type() {
            return None;
        }
        let name_token = reference.value_token().ok()?;
        self.references.insert(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }

    fn visit_static_member_expression(&mut self, member: JsStaticMemberExpression) -> Option<()> {
        let object = member.object().ok()?;
        if let Some(reference) = object.as_js_reference_identifier() {
            self.visit_reference_identifier(reference.clone())?;
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::JsFileSource;

    fn parse_js(source: &str) -> AnyJsRoot {
        let result = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
        result.tree()
    }

    fn contains_reference(service: &EmbeddedValueReferences, reference: &str) -> bool {
        for refs in service.references.iter() {
            if refs.values().any(|token| token.text() == reference) {
                return true;
            }
        }
        false
    }

    #[test]
    fn tracks_value_references() {
        let source = r#"Component; FooEnum.Foo;"#;

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_non_source_snippet(&parse_js(source));
        service.finish(builder);

        assert!(contains_reference(&service, "Component"));
        assert!(contains_reference(&service, "FooEnum"));
    }

    #[test]
    fn tracks_static_member_expressions() {
        let source = r#"FooEnum.Bar; obj.property.nested;"#;

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_non_source_snippet(&parse_js(source));
        service.finish(builder);

        assert!(contains_reference(&service, "FooEnum"));
        assert!(contains_reference(&service, "obj"));
    }

    #[test]
    fn tracks_multiple_snippets() {
        let source_1 = r#"Component; FooEnum.Foo;"#;
        let source_2 = r#"AnotherComponent; BarEnum.Bar;"#;

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_non_source_snippet(&parse_js(source_1));
        builder.visit_non_source_snippet(&parse_js(source_2));
        service.finish(builder);

        assert!(contains_reference(&service, "Component"));
        assert!(contains_reference(&service, "FooEnum"));
        assert!(contains_reference(&service, "AnotherComponent"));
        assert!(contains_reference(&service, "BarEnum"));
    }
}
