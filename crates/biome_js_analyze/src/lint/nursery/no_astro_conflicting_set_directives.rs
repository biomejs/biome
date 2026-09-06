use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxChild, JsLanguage, JsxAttribute, JsxElement,
    JsxName, jsx_ext::AnyJsxElement,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_astro_conflicting_set_directives::NoAstroConflictingSetDirectivesOptions;

declare_lint_rule! {
    /// Disallow conflicting content sources on Astro elements.
    ///
    /// The `set:html` and `set:text` directives replace an element's child content.
    /// Combining either directive with another content source makes it unclear which
    /// content should be rendered.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,ignore
    /// <div>{items.map((item) => <span set:html={item.html}>Fallback content</span>)}</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// <div>{items.map((item) => <span set:html={item.html} />)}</div>
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro template directives](https://docs.astro.build/en/reference/directives-reference/#sethtml)
    pub NoAstroConflictingSetDirectives {
        version: "next",
        name: "noAstroConflictingSetDirectives",
        language: "jsx",
        severity: Severity::Error,
        recommended: true,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-conflict-set-directives").same()],
    }
}

fn set_directive_name(name: &JsxName) -> Option<&'static str> {
    match name.value_token().ok()?.text_trimmed() {
        "set:html" => Some("set:html"),
        "set:text" => Some("set:text"),
        _ => None,
    }
}

pub enum RuleState {
    SetDirective(JsxName),
    ChildContent(TextRange),
}

impl Rule for NoAstroConflictingSetDirectives {
    type Query = Ast<JsxName>;
    type State = Box<[RuleState]>;
    type Signals = Option<Self::State>;
    type Options = NoAstroConflictingSetDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_template()
        {
            return None;
        }

        let name = ctx.query();
        set_directive_name(name)?;
        let attribute = name.syntax().parent().and_then(JsxAttribute::cast)?;
        let element = attribute
            .syntax()
            .ancestors()
            .find_map(AnyJsxElement::cast)?;
        let mut conflicting_sources = Vec::new();

        for sibling in element.attributes().iter() {
            let AnyJsxAttribute::JsxAttribute(attribute) = sibling else {
                continue;
            };
            let Ok(AnyJsxAttributeName::JsxName(sibling_name)) = attribute.name() else {
                continue;
            };
            if sibling_name.syntax() == name.syntax() {
                continue;
            }
            if set_directive_name(&sibling_name).is_none() {
                continue;
            }
            conflicting_sources.push(RuleState::SetDirective(sibling_name));
        }

        if let AnyJsxElement::JsxOpeningElement(opening_element) = element {
            let element = opening_element
                .syntax()
                .parent()
                .and_then(JsxElement::cast)?;
            let mut child_ranges = element.children().iter().filter_map(|child| match child {
                AnyJsxChild::JsxText(text) => {
                    let token = text.value_token().ok()?;
                    let text = token.token_text_trimmed().trim_token();
                    (!text.is_empty()).then(|| text.source_range(token.text_range()))
                }
                AnyJsxChild::JsxExpressionChild(expression) if expression.expression().is_none() => {
                    None
                }
                child => Some(child.range()),
            });

            if let Some(first) = child_ranges.next() {
                let last = child_ranges.next_back().unwrap_or(first);
                conflicting_sources.push(RuleState::ChildContent(TextRange::new(
                    first.start(),
                    last.end(),
                )));
            }
        }

        (!conflicting_sources.is_empty()).then(|| conflicting_sources.into_boxed_slice())
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .syntax()
            .parent()
            .and_then(JsxAttribute::cast)
            .map(|attribute| attribute.range())
    }

    fn suppressed_nodes(
        _ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        for source in state.iter() {
            if let RuleState::SetDirective(name) = source {
                suppressions.suppress_node(name.syntax().clone());
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query();
        let directive_name = set_directive_name(name)?;
        let attribute = name.syntax().parent().and_then(JsxAttribute::cast)?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attribute.range(),
            markup! {
                "The "<Emphasis>{directive_name}</Emphasis>" directive conflicts with another content source."
            },
        );

        for source in state.iter() {
            diagnostic = match source {
                RuleState::SetDirective(name) => {
                    let directive_name = set_directive_name(name)?;
                    let attribute = name.syntax().parent().and_then(JsxAttribute::cast)?;
                    diagnostic.detail(
                        attribute.range(),
                        markup! {
                            "The "<Emphasis>{directive_name}</Emphasis>" directive defines the element content here."
                        },
                    )
                }
                RuleState::ChildContent(range) => diagnostic.detail(
                    *range,
                    markup! {
                        "Child content defines the element content here."
                    },
                ),
            };
        }

        Some(diagnostic.note(markup! {
            "Choose only one content source for this element."
        }))
    }
}
