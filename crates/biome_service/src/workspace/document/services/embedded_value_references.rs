use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, AnySvelteBindingProperty, AnySvelteDirective,
    HtmlElement, HtmlFileSource, HtmlRoot, HtmlSelfClosingElement,
};
use biome_js_syntax::{
    AnyJsIdentifierUsage, AnyJsRoot, JsReferenceIdentifier, JsStaticMemberExpression,
    JsxReferenceIdentifier,
};
use biome_rowan::{AstNode, TextRange, TokenText, WalkEvent};

#[derive(Debug, Clone, Default)]
pub struct EmbeddedValueReferences {
    /// Identifiers referenced as values.
    pub value_references: Vec<Vec<(TextRange, TokenText)>>,
    /// Identifiers referenced only in type position (e.g. `icon: IconType`). Constructs that create a type and a value e.g. `class` aren't tracked.
    pub type_references: Vec<Vec<(TextRange, TokenText)>>,
}

#[derive(Debug)]
pub(crate) struct EmbeddedValueReferencesBuilder {
    value_references: Vec<(TextRange, TokenText)>,
    type_references: Vec<(TextRange, TokenText)>,
}

impl EmbeddedValueReferences {
    pub(crate) fn builder(&self) -> EmbeddedValueReferencesBuilder {
        EmbeddedValueReferencesBuilder::new()
    }

    pub(crate) fn finish(&mut self, builder: EmbeddedValueReferencesBuilder) {
        self.value_references.push(builder.value_references);
        self.type_references.push(builder.type_references);
    }
}

impl EmbeddedValueReferencesBuilder {
    fn new() -> Self {
        Self {
            value_references: Vec::default(),
            type_references: Vec::default(),
        }
    }

    pub(crate) fn register_reference(&mut self, range: TextRange, text: TokenText) {
        self.value_references.push((range, text));
    }

    pub(crate) fn register_type_reference(&mut self, range: TextRange, text: TokenText) {
        self.type_references.push((range, text));
    }

    /// Visit a non-source snippet to track value references
    pub(crate) fn visit_non_source_snippet(&mut self, root: &AnyJsRoot) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(reference) = JsxReferenceIdentifier::cast_ref(&node) {
                        self.visit_jsx_reference_identifier(reference);
                    } else if let Some(reference) = JsReferenceIdentifier::cast_ref(&node) {
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
    /// This extracts component names from Vue/Svelte/Astro templates like:
    /// - `<Component />` → tracks `Component`
    /// - `<AvatarPrimitive.Fallback>` → tracks `AvatarPrimitive`
    ///
    /// For Svelte, it also tracks directive names that read a local binding,
    /// such as `use:action` or the `bind:open` shorthand. Interpolations like
    /// `style="top: {top}px"` are parsed as embedded JavaScript snippets, so
    /// their references are collected through the normal snippet path.
    pub(crate) fn visit_html_root(&mut self, root: &HtmlRoot, file_source: &HtmlFileSource) {
        let is_svelte = file_source.is_svelte();
        for node in root.syntax().descendants() {
            // Check HtmlElement: <Component>...</Component>
            if let Some(element) = HtmlElement::cast_ref(&node) {
                self.visit_html_element(&element);
            }
            // Check HtmlSelfClosingElement: <Component />
            if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
                self.visit_html_self_closing_element(&element);
            }
            if is_svelte && let Some(directive) = AnySvelteDirective::cast_ref(&node) {
                self.register_svelte_directive_reference(&directive);
            }
        }
    }

    /// Registers the directive name as a value reference when it resolves to a
    /// local binding. Which directives qualify:
    ///
    /// - `use:inView` → tracks `inView` (an action function)
    /// - `transition:fade`, `in:fly`, `out:fly`, `animate:flip` → tracks the
    ///   transition/animation function
    /// - `bind:open` (shorthand, no `={...}`) → tracks `open` (reads the
    ///   local variable of the same name)
    /// - `bind:value={expr}` → skipped; the expression is a snippet handled
    ///   elsewhere, and the directive name `value` is an HTML attribute, not a
    ///   binding reference
    /// - `style:color`, `class:active` → skipped; these name CSS
    ///   properties/classes, not JS bindings
    fn register_svelte_directive_reference(
        &mut self,
        directive: &AnySvelteDirective,
    ) -> Option<()> {
        let value = match directive {
            AnySvelteDirective::SvelteUseDirective(d) => d.value().ok()?,
            AnySvelteDirective::SvelteTransitionDirective(d) => d.value().ok()?,
            AnySvelteDirective::SvelteInDirective(d) => d.value().ok()?,
            AnySvelteDirective::SvelteOutDirective(d) => d.value().ok()?,
            AnySvelteDirective::SvelteAnimateDirective(d) => d.value().ok()?,
            AnySvelteDirective::SvelteBindDirective(d) => {
                let value = d.value().ok()?;
                if value.initializer().is_some() {
                    return None;
                }
                value
            }
            AnySvelteDirective::SvelteStyleDirective(_)
            | AnySvelteDirective::SvelteClassDirective(_) => return None,
        };
        self.register_svelte_binding_property(value.property().ok())
    }

    /// Extracts the identifier from a directive's binding property, if it is a
    /// plain name. Only `SvelteName` maps to a local variable:
    ///
    /// - `bind:open` → property is `SvelteName("open")` → tracks `open`
    /// - `bind:a.b` → property is `SvelteMemberProperty` → skipped (not a
    ///   simple local binding)
    /// - `bind:"literal"` → property is `SvelteLiteral` → skipped
    fn register_svelte_binding_property(
        &mut self,
        property: Option<AnySvelteBindingProperty>,
    ) -> Option<()> {
        let token = match property? {
            AnySvelteBindingProperty::SvelteName(name) => name.ident_token().ok()?,
            AnySvelteBindingProperty::SvelteMemberProperty(_)
            | AnySvelteBindingProperty::SvelteLiteral(_) => return None,
        };
        self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
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
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
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
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlComponentObjectName::HtmlComponentName(component) => {
                // Track the component name
                if let Ok(token) = component.value_token() {
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
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

    fn visit_jsx_reference_identifier(&mut self, reference: JsxReferenceIdentifier) -> Option<()> {
        let name_token = reference.value_token().ok()?;
        self.register_reference(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }

    fn visit_reference_identifier(&mut self, reference: JsReferenceIdentifier) -> Option<()> {
        let usage = AnyJsIdentifierUsage::from(reference.clone());
        let name_token = reference.value_token().ok()?;
        // Classify by how the reference is used here, not what it declares: a
        // name in type position (`x: Foo`) goes to the type list, as a value
        // (`new Foo()`) to the value list. Used both ways, it lands in both.
        if usage.is_only_type() {
            self.register_type_reference(
                name_token.text_trimmed_range(),
                name_token.token_text_trimmed(),
            );
        } else {
            self.register_reference(
                name_token.text_trimmed_range(),
                name_token.token_text_trimmed(),
            );
        }
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
    use biome_languages::JsFileSource;

    fn parse_js(source: &str) -> AnyJsRoot {
        let result = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
        result.tree()
    }

    fn contains_reference(service: &EmbeddedValueReferences, reference: &str) -> bool {
        for refs in service.value_references.iter() {
            if refs.iter().any(|(_, token)| {
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
        use biome_html_parser::{HtmlParserOptions, parse_html};

        let source = r#"<Component /><AvatarPrimitive.Fallback />"#;
        // Enable Vue parsing so component names are parsed correctly
        let parsed = parse_html(source, HtmlParserOptions::default().with_vue());

        println!("Diagnostics: {:?}", parsed.diagnostics());
        println!("Has errors: {}", !parsed.diagnostics().is_empty());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree(), &HtmlFileSource::vue());
        service.finish(builder);

        assert!(contains_reference(&service, "Component"));
        assert!(contains_reference(&service, "AvatarPrimitive"));
    }

    #[test]
    fn tracks_type_only_references_separately() {
        // `IconType` is used only as a type; it must land in type_references,
        // not references, so useImportType still treats it as type-only while
        // the unused-* rules see it as used.
        let source = r#"const x: IconType = foo;"#;
        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_non_source_snippet(&parse_js(source));
        service.finish(builder);

        let in_value = service
            .value_references
            .iter()
            .any(|r| r.iter().any(|(_, t)| t.text() == "IconType"));
        let in_type = service
            .type_references
            .iter()
            .any(|r| r.iter().any(|(_, t)| t.text() == "IconType"));
        assert!(!in_value, "IconType should not be a value reference");
        assert!(in_type, "IconType should be a type reference");
        // `foo` is a value reference.
        assert!(contains_reference(&service, "foo"));
    }

    #[test]
    fn extracts_svelte_directive_names() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        let source = r#"<div use:inView transition:fade in:fly out:fly animate:flip></div>"#;
        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree(), &HtmlFileSource::svelte());
        service.finish(builder);

        assert!(contains_reference(&service, "inView"));
        assert!(contains_reference(&service, "fade"));
        assert!(contains_reference(&service, "fly"));
        assert!(contains_reference(&service, "flip"));
    }

    #[test]
    fn extracts_svelte_bind_shorthand() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        // `bind:open` without `={...}` is shorthand for `bind:open={open}` — the
        // local variable `open` must be tracked as a value reference.
        let source = r#"<Modal bind:open />"#;
        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree(), &HtmlFileSource::svelte());
        service.finish(builder);

        assert!(contains_reference(&service, "open"));
    }

    #[test]
    fn ignores_svelte_bind_with_initializer() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        // `bind:value={expr}` has an explicit initializer; the directive name
        // itself is not a variable reference (the expression is handled separately).
        let source = r#"<input bind:value={myVal} />"#;
        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree(), &HtmlFileSource::svelte());
        service.finish(builder);

        assert!(!contains_reference(&service, "value"));
    }
}
