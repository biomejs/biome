use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnyHtmlContent, AnyHtmlElement, HtmlElement,
    HtmlSelfClosingElement, HtmlSyntaxToken,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, TextRange, declare_node_union};
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
enum ContentSourceKind {
    SetHtml,
    SetText,
    ChildContent,
}

#[derive(Clone, Copy)]
struct ContentSource {
    kind: ContentSourceKind,
    range: TextRange,
}

pub struct State {
    sources: Box<[ContentSource]>,
}

declare_node_union! {
    pub AnyHtmlElementLike = HtmlElement | HtmlSelfClosingElement
}

impl Rule for NoAstroConflictingSetDirectives {
    type Query = Ast<AnyHtmlElementLike>;
    type State = Box<[ContentSource]>;
    type Signals = Option<Self::State>;
    type Options = NoAstroConflictingSetDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }

        let (attributes, children) = match ctx.query() {
            AnyHtmlElementLike::HtmlElement(element) => (
                element.opening_element().ok()?.attributes(),
                Some(element.children()),
            ),
            AnyHtmlElementLike::HtmlSelfClosingElement(element) => (element.attributes(), None),
        };

        collect_conflicting_content_sources(
            attributes.iter().filter_map(content_source_from_attribute),
            || {
                children
                    .as_ref()
                    .and_then(child_content_range)
                    .map(|range| ContentSource {
                        kind: ContentSourceKind::ChildContent,
                        range,
                    })
            },
        )
        .map(|sources| State { sources })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let primary = state.sources.get(1)?;
        let mut diagnostic = match primary.kind {
            ContentSourceKind::SetHtml => RuleDiagnostic::new(
                rule_category!(),
                primary.range,
                markup! {
                    "The "<Emphasis>"set:html"</Emphasis>" directive conflicts with another content source."
                },
            ),
            ContentSourceKind::SetText => RuleDiagnostic::new(
                rule_category!(),
                primary.range,
                markup! {
                    "The "<Emphasis>"set:text"</Emphasis>" directive conflicts with another content source."
                },
            ),
            ContentSourceKind::ChildContent => RuleDiagnostic::new(
                rule_category!(),
                primary.range,
                markup! {
                    "This child content conflicts with another content source."
                },
            ),
        };

        for (index, source) in state.sources.iter().enumerate() {
            if index == 1 {
                continue;
            }

            diagnostic = match source.kind {
                ContentSourceKind::SetHtml => diagnostic.detail(
                    source.range,
                    markup! {
                        "The "<Emphasis>"set:html"</Emphasis>" directive defines the element content here."
                    },
                ),
                ContentSourceKind::SetText => diagnostic.detail(
                    source.range,
                    markup! {
                        "The "<Emphasis>"set:text"</Emphasis>" directive defines the element content here."
                    },
                ),
                ContentSourceKind::ChildContent => diagnostic.detail(
                    source.range,
                    markup! {
                        "Child content defines the element content here."
                    },
                ),
            };
        }

        Some(
            diagnostic
                .note(markup! {
                    "The "<Emphasis>"set:html"</Emphasis>" directive, the "<Emphasis>"set:text"</Emphasis>" directive, and child content are three ways to define an element's content."
                })
                .note(markup! {
                    "Choose only one content source for this element."
                }),
        )
    }
}

fn collect_conflicting_content_sources(
    mut attributes: impl Iterator<Item = ContentSource>,
    child: impl FnOnce() -> Option<ContentSource>,
) -> Option<Box<[ContentSource]>> {
    let first = attributes.next()?;

    let Some(second) = attributes.next() else {
        let second = child()?;
        return Some(Vec::from([first, second]).into_boxed_slice());
    };

    let mut sources = Vec::from([first, second]);
    sources.extend(attributes);
    if let Some(child) = child() {
        sources.push(child);
    }
    Some(sources.into_boxed_slice())
}

fn content_source_from_attribute(attribute: AnyHtmlAttribute) -> Option<ContentSource> {
    let AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroSetDirective(directive)) =
        attribute
    else {
        return None;
    };
    let value = directive.value().ok()?;
    let name = value.name().ok()?.token_text_trimmed()?;
    let kind = match name.text() {
        "html" => ContentSourceKind::SetHtml,
        "text" => ContentSourceKind::SetText,
        _ => return None,
    };

    Some(ContentSource {
        kind,
        range: directive.range(),
    })
}

fn child_content_range(children: &biome_html_syntax::HtmlElementList) -> Option<TextRange> {
    let mut ranges = children.iter().filter_map(|child| match child {
        AnyHtmlElement::AnyHtmlContent(content) => content_range(content),
        child => Some(child.range()),
    });
    let first = ranges.next()?;
    let last = ranges.next_back().unwrap_or(first);
    Some(TextRange::new(first.start(), last.end()))
}

fn content_range(content: AnyHtmlContent) -> Option<TextRange> {
    match content {
        AnyHtmlContent::HtmlContent(content) => {
            content.value_token().ok().and_then(token_content_range)
        }
        AnyHtmlContent::HtmlEmbeddedContent(content) => {
            content.value_token().ok().and_then(token_content_range)
        }
        AnyHtmlContent::AnyHtmlTextExpression(expression) => Some(expression.range()),
    }
}

fn token_content_range(token: HtmlSyntaxToken) -> Option<TextRange> {
    let text = token.token_text_trimmed().trim_token();
    (!text.is_empty()).then(|| text.source_range(token.text_range()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_html_parser::parse_html;

    #[test]
    fn child_content_range_spans_multiple_children() {
        let parsed = parse_html("<div>first<span></span>last</div>", Default::default());
        let element = parsed
            .syntax()
            .descendants()
            .find_map(HtmlElement::cast)
            .expect("parsed source should contain an element");

        assert_eq!(
            child_content_range(&element.children()),
            Some(TextRange::new(5.into(), 27.into()))
        );
    }

    #[test]
    fn skips_child_content_without_set_directives() {
        assert!(
            collect_conflicting_content_sources(std::iter::empty(), || {
                panic!("child content should not be evaluated")
            })
            .is_none()
        );
    }
}
