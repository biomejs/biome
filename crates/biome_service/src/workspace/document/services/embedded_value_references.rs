use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, AnySvelteBindingProperty, HtmlElement, HtmlRoot,
    HtmlSelfClosingElement, HtmlString, SvelteAnimateDirective, SvelteBindDirective,
    SvelteInDirective, SvelteOutDirective, SvelteTransitionDirective, SvelteUseDirective,
};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{
    AnyJsIdentifierUsage, AnyJsRoot, JsFileSource, JsReferenceIdentifier, JsStaticMemberExpression,
    JsxReferenceIdentifier,
};
use biome_rowan::{AstNode, TextRange, TokenText, WalkEvent};

#[derive(Debug, Clone, Default)]
pub struct EmbeddedValueReferences {
    /// Identifiers referenced as values in non-source snippets.
    pub references: Vec<Vec<(TextRange, TokenText)>>,
    /// Identifiers referenced only as types (e.g. `icon: IconType` in a
    /// `{#snippet}` parameter type). Tracked separately so `useImportType`
    /// can keep distinguishing value vs type usage, while the unused-* rules
    /// can treat either as a use.
    pub type_references: Vec<Vec<(TextRange, TokenText)>>,
}

#[derive(Debug)]
pub(crate) struct EmbeddedValueReferencesBuilder {
    references: Vec<(TextRange, TokenText)>,
    type_references: Vec<(TextRange, TokenText)>,
}

impl EmbeddedValueReferences {
    pub(crate) fn builder(&self) -> EmbeddedValueReferencesBuilder {
        EmbeddedValueReferencesBuilder::new()
    }

    pub(crate) fn finish(&mut self, builder: EmbeddedValueReferencesBuilder) {
        self.references.push(builder.references);
        self.type_references.push(builder.type_references);
    }
}

impl EmbeddedValueReferencesBuilder {
    fn new() -> Self {
        Self {
            references: Vec::default(),
            type_references: Vec::default(),
        }
    }

    pub(crate) fn register_reference(&mut self, range: TextRange, text: TokenText) {
        self.references.push((range, text));
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
    /// This extracts component names from Vue/Svelte templates like:
    /// - `<Component />` → tracks `Component`
    /// - `<AvatarPrimitive.Fallback>` → tracks `AvatarPrimitive`
    ///
    /// When `is_svelte` is set, also extracts references from Svelte-only
    /// constructs the parser leaves opaque or unattached: directive names
    /// (`use:action`) and `{expr}` interpolations embedded inside quoted
    /// attribute values (`style="top: {top}px"`).
    pub(crate) fn visit_html_root(&mut self, root: &HtmlRoot, is_svelte: bool) {
        for node in root.syntax().descendants() {
            // Check HtmlElement: <Component>...</Component>
            if let Some(element) = HtmlElement::cast_ref(&node) {
                self.visit_html_element(&element);
            }

            // Check HtmlSelfClosingElement: <Component />
            if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
                self.visit_html_self_closing_element(&element);
            }

            if is_svelte && let Some(string) = HtmlString::cast_ref(&node) {
                self.visit_svelte_attribute_string(&string);
            }

            // Svelte directive names that reference imported values:
            // `use:action`, `transition:fn`, `in:fn`, `out:fn`, `animate:fn`.
            // The `={initializer}` part is extracted as a snippet elsewhere; here
            // we register the directive name itself (`action` / `fn`). `bind:`,
            // `class:`, and `style:` are excluded — their property is a DOM/CSS
            // name, not a JS reference.
            if let Some(directive) = SvelteUseDirective::cast_ref(&node) {
                if let Ok(value) = directive.value() {
                    self.register_svelte_binding_property(value.property().ok());
                }
            } else if let Some(directive) = SvelteTransitionDirective::cast_ref(&node) {
                if let Ok(value) = directive.value() {
                    self.register_svelte_binding_property(value.property().ok());
                }
            } else if let Some(directive) = SvelteInDirective::cast_ref(&node) {
                if let Ok(value) = directive.value() {
                    self.register_svelte_binding_property(value.property().ok());
                }
            } else if let Some(directive) = SvelteOutDirective::cast_ref(&node) {
                if let Ok(value) = directive.value() {
                    self.register_svelte_binding_property(value.property().ok());
                }
            } else if let Some(directive) = SvelteAnimateDirective::cast_ref(&node) {
                if let Ok(value) = directive.value() {
                    self.register_svelte_binding_property(value.property().ok());
                }
            } else if let Some(directive) = SvelteBindDirective::cast_ref(&node) {
                // Shorthand `bind:open` (no `={...}`) binds the local variable
                // named by the property. With an explicit initializer
                // (`bind:value={x}`) the property is a DOM/component name and
                // the reference `x` is extracted from the initializer snippet.
                if let Ok(value) = directive.value()
                    && value.initializer().is_none()
                {
                    self.register_svelte_binding_property(value.property().ok());
                }
            }
        }
    }

    /// Registers the identifier of a Svelte directive property
    /// (`use:action` → `action`) as a value reference.
    fn register_svelte_binding_property(
        &mut self,
        property: Option<AnySvelteBindingProperty>,
    ) -> Option<()> {
        let name = property?;
        let token = match name {
            AnySvelteBindingProperty::SvelteName(name) => name.ident_token().ok()?,
            // A member like `transition:ns.fade` or a literal isn't a plain
            // imported binding we can attribute to a single name; skip.
            AnySvelteBindingProperty::SvelteMemberProperty(_)
            | AnySvelteBindingProperty::SvelteLiteral(_) => return None,
        };
        self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    /// Extracts `{expr}` interpolations embedded inside a quoted Svelte
    /// attribute value (`style="top: {top}px"`, `class="a {cls} b"`). The
    /// parser stores the whole value as one opaque string token, so the
    /// interpolations are invisible to the AST; we scan the raw text for
    /// balanced `{...}` groups, parse each as JS, and register the references.
    fn visit_svelte_attribute_string(&mut self, string: &HtmlString) -> Option<()> {
        let token = string.value_token().ok()?;
        let raw = token.text_trimmed();
        // Strip surrounding quotes so a quote char doesn't open a string state.
        let inner = raw
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| raw.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(raw);

        for expr in svelte_interpolations(inner) {
            // Ranges from this parse are throwaway: `is_used_as_value` only
            // compares token text, never positions.
            let parsed =
                biome_js_parser::parse(expr, JsFileSource::ts(), JsParserOptions::default());
            self.visit_non_source_snippet(&parsed.tree());
        }
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
        if usage.is_only_type() {
            // Type-only usage (`icon: IconType`): record separately so it
            // counts as a use for the unused-* rules without confusing
            // `useImportType` (which must still see it as "not a value").
            self.register_type_reference(
                name_token.text_trimmed_range(),
                name_token.token_text_trimmed(),
            );
            return Some(());
        }
        self.register_reference(
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

/// Scans a Svelte attribute value for `{...}` interpolations and returns the
/// inner expression text of each. Brace matching is balanced and aware of JS
/// string and template literals, so a `}` inside a string (`{ok ? 'a}b' : c}`)
/// or nested object (`{ {x: 1} }`) does not end the interpolation early.
fn svelte_interpolations(input: &str) -> Vec<&str> {
    let bytes = input.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'{' {
            i += 1;
            continue;
        }
        // Start of an interpolation. Scan to the matching closing brace.
        let start = i + 1;
        let mut depth = 1usize;
        let mut j = start;
        // String/template-literal state inside the expression.
        let mut quote: Option<u8> = None;
        while j < bytes.len() {
            let c = bytes[j];
            if let Some(q) = quote {
                if c == b'\\' {
                    j += 2;
                    continue;
                }
                if c == q {
                    quote = None;
                }
                j += 1;
                continue;
            }
            match c {
                b'\'' | b'"' | b'`' => quote = Some(c),
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        if depth == 0 {
            let expr = input[start..j].trim();
            if !expr.is_empty() {
                out.push(expr);
            }
            i = j + 1;
        } else {
            // Unbalanced — stop scanning, the rest isn't a valid interpolation.
            break;
        }
    }
    out
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
        builder.visit_html_root(&parsed.tree(), false);
        service.finish(builder);

        assert!(contains_reference(&service, "Component"));
        assert!(contains_reference(&service, "AvatarPrimitive"));
    }

    #[test]
    fn extracts_svelte_attribute_string_interpolations() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        let source = r#"<div style="top: {top}px; left: {left}px" class="a {cls} b"></div>"#;
        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedValueReferences::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree(), true);
        service.finish(builder);

        assert!(contains_reference(&service, "top"));
        assert!(contains_reference(&service, "left"));
        assert!(contains_reference(&service, "cls"));
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
            .references
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
        builder.visit_html_root(&parsed.tree(), true);
        service.finish(builder);

        assert!(contains_reference(&service, "inView"));
        assert!(contains_reference(&service, "fade"));
        assert!(contains_reference(&service, "fly"));
        assert!(contains_reference(&service, "flip"));
    }

    #[test]
    fn svelte_interpolations_is_brace_and_string_aware() {
        assert_eq!(svelte_interpolations("top: {top}px"), vec!["top"]);
        assert_eq!(svelte_interpolations("a {x} b {y} c"), vec!["x", "y"]);
        // `}` inside a string must not end the interpolation early.
        assert_eq!(
            svelte_interpolations("{ ok ? 'a}b' : c }"),
            vec!["ok ? 'a}b' : c"]
        );
        // Nested braces (object literal).
        assert_eq!(svelte_interpolations("{ {x: 1} }"), vec!["{x: 1}"]);
        // No interpolation.
        assert!(svelte_interpolations("plain text").is_empty());
    }
}
