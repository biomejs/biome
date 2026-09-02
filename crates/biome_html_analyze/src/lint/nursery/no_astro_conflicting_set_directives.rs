use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnyHtmlContent, AnyHtmlElement, AnyHtmlTextExpression,
    AstroSetDirective, HtmlElement, HtmlLanguage, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
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
    /// <div set:html={content}>Fallback content</div>
    /// ```
    ///
    /// ```astro,expect_diagnostic
    /// <div set:html={html} set:text={text}></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <div set:html={content}></div>
    /// <div>Fallback content</div>
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro template directives](https://docs.astro.build/en/reference/directives-reference/#sethtml)
    pub NoAstroConflictingSetDirectives {
        version: "next",
        name: "noAstroConflictingSetDirectives",
        language: "html",
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
    fn from_directive(directive: &AstroSetDirective) -> Option<Self> {
        let value = directive.value().ok()?;
        let name = value.name().ok()?.token_text_trimmed()?;

        match name.text() {
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
        directive: AstroSetDirective,
    },
    ChildContent {
        range: TextRange,
    },
}

pub struct State {
    conflicts: Box<[Conflict]>,
}

fn is_js_trivia_only(mut source: &str) -> bool {
    loop {
        source = source.trim_start_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
        if source.is_empty() {
            return true;
        }

        if let Some(rest) = source.strip_prefix("//") {
            source = rest
                .find(['\n', '\r', '\u{2028}', '\u{2029}'])
                .map_or("", |index| &rest[index..]);
        } else if let Some(rest) = source.strip_prefix("/*") {
            source = rest.find("*/").map_or("", |index| &rest[index + 2..]);
        } else {
            return false;
        }
    }
}

fn meaningful_child_range(content: &AnyHtmlContent) -> Option<TextRange> {
    match content {
        AnyHtmlContent::HtmlContent(content) => {
            let token = content.value_token().ok()?;
            let text = token.token_text_trimmed().trim_token();
            (!text.is_empty()).then(|| text.source_range(token.text_range()))
        }
        AnyHtmlContent::HtmlEmbeddedContent(content) => {
            let token = content.value_token().ok()?;
            let text = token.token_text_trimmed().trim_token();
            (!text.is_empty()).then(|| text.source_range(token.text_range()))
        }
        AnyHtmlContent::AnyHtmlTextExpression(
            AnyHtmlTextExpression::HtmlSingleTextExpression(expression),
        ) => {
            let token = expression.expression()?.html_literal_token().ok()?;
            (!is_js_trivia_only(token.text_trimmed())).then(|| expression.range())
        }
        AnyHtmlContent::AnyHtmlTextExpression(expression) => Some(expression.range()),
    }
}

impl Rule for NoAstroConflictingSetDirectives {
    type Query = Ast<AstroSetDirective>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoAstroConflictingSetDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }

        let directive = ctx.query();
        SetDirectiveKind::from_directive(directive)?;

        let element = directive
            .syntax()
            .ancestors()
            .find_map(AnyHtmlTagElement::cast)?;
        let mut conflicts = Vec::new();

        for attribute in element.attributes().iter() {
            let AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroSetDirective(
                sibling,
            )) = attribute
            else {
                continue;
            };
            if sibling.syntax() == directive.syntax() {
                continue;
            }
            let Some(kind) = SetDirectiveKind::from_directive(&sibling) else {
                continue;
            };
            conflicts.push(Conflict::Directive {
                kind,
                directive: sibling,
            });
        }

        if let AnyHtmlTagElement::HtmlOpeningElement(opening_element) = element {
            let element = opening_element
                .syntax()
                .parent()
                .and_then(HtmlElement::cast)?;
            let mut child_ranges = element.children().iter().filter_map(|child| match child {
                AnyHtmlElement::AnyHtmlContent(content) => meaningful_child_range(&content),
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
        Some(ctx.query().range())
    }

    fn suppressed_nodes(
        _ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<HtmlLanguage>,
    ) {
        for conflict in &state.conflicts {
            if let Conflict::Directive { directive, .. } = conflict {
                suppressions.suppress_node(directive.syntax().clone());
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        let kind = SetDirectiveKind::from_directive(directive)?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            directive.range(),
            markup! {
                "The "<Emphasis>{kind.name()}</Emphasis>" directive conflicts with another content source."
            },
        );

        for conflict in &state.conflicts {
            diagnostic = match conflict {
                Conflict::Directive { kind, directive } => diagnostic.detail(
                    directive.range(),
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
