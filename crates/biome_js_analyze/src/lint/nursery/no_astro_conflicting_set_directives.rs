use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxChild, JsLanguage, JsxAttribute, JsxElement,
    JsxNamespaceName, jsx_ext::AnyJsxElement,
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
    /// ```astro,expect_diagnostic
    /// <div>{items.map((item) => <span set:html={item.html}>Fallback content</span>)}</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
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

#[derive(Clone, Copy)]
enum SetDirectiveKind {
    Html,
    Text,
}

impl SetDirectiveKind {
    fn from_name(name: &JsxNamespaceName) -> Option<Self> {
        if name
            .namespace()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "set"
        {
            return None;
        }

        match name.name().ok()?.value_token().ok()?.text_trimmed() {
            "html" => Some(Self::Html),
            "text" => Some(Self::Text),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Html => "set:html",
            Self::Text => "set:text",
        }
    }
}

enum Conflict {
    Directive {
        kind: SetDirectiveKind,
        name: JsxNamespaceName,
        range: TextRange,
    },
    ChildContent {
        range: TextRange,
    },
}

pub struct State {
    conflicts: Box<[Conflict]>,
}

impl Rule for NoAstroConflictingSetDirectives {
    type Query = Ast<JsxNamespaceName>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoAstroConflictingSetDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro()
        {
            return None;
        }

        let name = ctx.query();
        SetDirectiveKind::from_name(name)?;
        let attribute = name.syntax().parent().and_then(JsxAttribute::cast)?;
        let element = attribute
            .syntax()
            .ancestors()
            .find_map(AnyJsxElement::cast)?;
        let mut conflicts = Vec::new();

        for sibling in element.attributes().iter() {
            let AnyJsxAttribute::JsxAttribute(attribute) = sibling else {
                continue;
            };
            let range = attribute.range();
            let Ok(AnyJsxAttributeName::JsxNamespaceName(sibling_name)) = attribute.name() else {
                continue;
            };
            if sibling_name.syntax() == name.syntax() {
                continue;
            }
            let Some(kind) = SetDirectiveKind::from_name(&sibling_name) else {
                continue;
            };
            conflicts.push(Conflict::Directive {
                kind,
                name: sibling_name,
                range,
            });
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
                conflicts.push(Conflict::ChildContent {
                    range: TextRange::new(first.start(), last.end()),
                });
            }
        }

        (!conflicts.is_empty()).then(|| State {
            conflicts: conflicts.into_boxed_slice(),
        })
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
        for conflict in &state.conflicts {
            if let Conflict::Directive { name, .. } = conflict {
                suppressions.suppress_node(name.syntax().clone());
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query();
        let kind = SetDirectiveKind::from_name(name)?;
        let attribute = name.syntax().parent().and_then(JsxAttribute::cast)?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attribute.range(),
            markup! {
                "The "<Emphasis>{kind.name()}</Emphasis>" directive conflicts with another content source."
            },
        );

        for conflict in &state.conflicts {
            diagnostic = match conflict {
                Conflict::Directive { kind, range, .. } => diagnostic.detail(
                    *range,
                    markup! {
                        "The "<Emphasis>{kind.name()}</Emphasis>" directive defines the element content here."
                    },
                ),
                Conflict::ChildContent { range } => diagnostic.detail(
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
