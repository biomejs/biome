use biome_analyze::{
    Ast, QueryMatch, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlContent, AnyHtmlElement, HtmlElement, HtmlOpeningElement,
    element_ext::AnyHtmlTagElement, inner_string_text,
};
use biome_rowan::AstNode;
use biome_rule_options::no_ambiguous_anchor_text::NoAmbiguousAnchorTextOptions;
use biome_string_case::StrOnlyExtension;

use crate::a11y::is_hidden_from_screen_reader;

declare_lint_rule! {
    /// Disallow ambiguous anchor descriptions.
    ///
    /// Enforces `<a>` values are not exact matches for the phrases "click here", "here", "link", "a link", or "learn more".
    /// Screen readers announce tags as links/interactive, but rely on values for context.
    /// Ambiguous anchor descriptions do not provide sufficient context for users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <a>learn more</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <a>documentation</a>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `words`
    ///
    /// The words option allows users to modify the strings that can be checked for in the anchor text. Useful for specifying other words in other languages.
    ///
    /// Default `["click here", "here", "link", "a link", "learn more"]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "words": ["click this"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```html,expect_diagnostic,use_options
    /// <a>click this</a>
    /// ```
    ///
    pub NoAmbiguousAnchorText {
        version: "2.3.10",
        name: "noAmbiguousAnchorText",
        language: "html",
        recommended: false,
        sources: &[RuleSource::EslintJsxA11y("anchor-ambiguous-text").same()],
    }
}

impl Rule for NoAmbiguousAnchorText {
    type Query = Ast<HtmlOpeningElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAmbiguousAnchorTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let words = ctx.options().words();

        let name = binding.name().ok()?;
        let name_text = name.token_text_trimmed()?;
        if name_text != "a" {
            return None;
        }

        let parent = HtmlElement::cast(binding.syntax().parent()?)?;
        let text = get_accessible_child_text(&parent);

        if words.contains(&text) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let parent = node.syntax().parent()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                parent.text_range(),
                markup! {
                    "No ambiguous anchor descriptions allowed."
                },
            )
            .note(markup! {
                "Ambiguous anchor descriptions do not provide sufficient context for screen reader users. Provide more context to these users."
            }),
        )
    }
}

fn get_aria_label(node: &AnyHtmlTagElement) -> Option<String> {
    let attribute = node.attributes().find_by_name("aria-label")?;
    let initializer = attribute.initializer()?;
    let value = initializer.value().ok()?;
    let html_string = value.as_html_string()?;
    let text = html_string.inner_string_text().ok()?;

    Some(text.to_string())
}

fn get_img_alt(node: &AnyHtmlTagElement) -> Option<String> {
    let name = node.name().ok()?;
    let name_text = name.token_text_trimmed()?;
    if name_text != "img" {
        return None;
    }

    let attribute = node.attributes().find_by_name("alt")?;
    let initializer = attribute.initializer()?;
    let value = initializer.value().ok()?;
    let html_string = value.as_html_string()?;
    let text = html_string.inner_string_text().ok()?;

    Some(text.to_string())
}

fn standardize_space_and_case(input: &str) -> String {
    input
        .chars()
        .filter(|c| !matches!(c, ',' | '.' | '?' | '¿' | '!' | '‽' | '¡' | ';' | ':'))
        .collect::<String>()
        .to_lowercase_cow()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_accessible_text(node: &AnyHtmlTagElement) -> Option<String> {
    if is_hidden_from_screen_reader(node) {
        return Some(String::new());
    }

    if let Some(aria_label) = get_aria_label(node) {
        return Some(standardize_space_and_case(&aria_label));
    }

    if let Some(alt) = get_img_alt(node) {
        return Some(standardize_space_and_case(&alt));
    }

    None
}

fn get_accessible_child_text(node: &HtmlElement) -> String {
    if let Ok(opening) = node.opening_element() {
        let any_jsx_element: AnyHtmlTagElement = opening.clone().into();
        if let Some(accessible_text) = get_accessible_text(&any_jsx_element) {
            return accessible_text;
        }
    };

    let raw_child_text = node
        .children()
        .into_iter()
        .map(|child| match child {
            AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlContent(content)) => {
                if let Ok(value_token) = content.value_token() {
                    inner_string_text(&value_token).to_string()
                } else {
                    String::new()
                }
            }
            AnyHtmlElement::HtmlElement(element) => get_accessible_child_text(&element),
            AnyHtmlElement::HtmlSelfClosingElement(element) => {
                let any_jsx_element: AnyHtmlTagElement = element.clone().into();
                get_accessible_text(&any_jsx_element).unwrap_or_default()
            }
            _ => String::new(),
        })
        .collect::<Vec<String>>()
        .join(" ");

    standardize_space_and_case(&raw_child_text)
}
