use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, HtmlElement, HtmlRoot, HtmlSelfClosingElement,
};
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

    /// Visit an HTML root to track component element names as value references.
    ///
    /// This extracts component names from Vue/Svelte templates like:
    /// - `<Component />` → tracks `Component`
    /// - `<AvatarPrimitive.Fallback>` → tracks `AvatarPrimitive`
    pub(crate) fn visit_html_root(&mut self, root: &HtmlRoot) {
        for node in root.syntax().descendants() {
            // Check HtmlElement: <Component>...</Component>
            if let Some(element) = HtmlElement::cast_ref(&node) {
                self.visit_html_element(&element);
            }

            // Check HtmlSelfClosingElement: <Component />
            if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
                self.visit_html_self_closing_element(&element);
            }
        }
    }

    fn visit_html_element(&mut self, element: &HtmlElement) -> Option<()> {
        // Skip script and style tags - these are not component references
        if element.is_script_tag() || element.is_style_tag() {
            return None;
        }

        let opening = element.opening_element().ok()?;
        let name = opening.name().ok()?;

        self.track_component_reference(&name);

        Some(())
    }

    fn visit_html_self_closing_element(&mut self, element: &HtmlSelfClosingElement) -> Option<()> {
        let name = element.name().ok()?;

        self.track_component_reference(&name);

        Some(())
    }

    /// Track a component name as a value reference
    fn track_component_reference(&mut self, name: &AnyHtmlTagName) {
        match name {
            AnyHtmlTagName::HtmlComponentName(component) => {
                // Track simple component: <Component>
                if let Ok(token) = component.value_token() {
                    self.references
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlTagName::HtmlMemberName(member) => {
                // Track the base component from member expression: <Component.Member>
                if let Ok(object) = member.object() {
                    self.track_component_object(&object);
                }
            }
            AnyHtmlTagName::HtmlTagName(_) => {
                // Regular HTML tag, don't track
            }
        }
    }

    /// Track the object part of a member expression
    fn track_component_object(&mut self, object: &AnyHtmlComponentObjectName) {
        match object {
            AnyHtmlComponentObjectName::HtmlTagName(tag) => {
                // For member expressions starting with lowercase (unusual but possible)
                if let Ok(token) = tag.value_token() {
                    self.references
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlComponentObjectName::HtmlComponentName(component) => {
                // Track the component name
                if let Ok(token) = component.value_token() {
                    self.references
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlComponentObjectName::HtmlMemberName(member) => {
                // Nested member - track its object recursively
                // e.g., A.B.C - track A
                if let Ok(object) = member.object() {
                    self.track_component_object(&object);
                }
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
            if refs.values().any(|token| {
                let text = token.text();
                text == reference
            }) {
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

    #[test]
    fn tracks_html_element_names() {
        use biome_html_parser::{HtmlParseOptions, parse_html};

        let source = r#"<Component /><AvatarPrimitive.Fallback />"#;
        // Enable Vue parsing so component names are parsed correctly
        let parsed = parse_html(source, HtmlParseOptions::default().with_vue());

        println!("Diagnostics: {:?}", parsed.diagnostics());
        println!("Has errors: {}", !parsed.diagnostics().is_empty());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree());
        service.finish(builder);

        assert!(contains_reference(&service, "Component"));
        assert!(contains_reference(&service, "AvatarPrimitive"));
    }
}
