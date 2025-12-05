use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsxChild, JsxElement, JsxOpeningElement, JsxSelfClosingElement, inner_string_text,
    jsx_ext::AnyJsxElement,
};
use biome_rowan::AstNode;
use biome_rule_options::no_ambiguous_anchor_text::NoAmbiguousAnchorTextOptions;
use biome_string_case::StrOnlyExtension;

use crate::a11y::is_hidden_from_screen_reader;

declare_lint_rule! {
    /// Disallow ambiguous anchor descriptions.
    ///
    /// Enforces <a> values are not exact matches for the phrases "click here", "here", "link", "a link", or "learn more".
    /// Screen readers announce tags as links/interactive, but rely on values for context.
    /// Ambiguous anchor descriptions do not provide sufficient context for users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const Invalid = () => <a>learn more</a>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const Valid = () => <a>documentation</a>;
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
    /// ```jsx,expect_diagnostic,use_options
    /// const Invalid = () => <a>click this</a>;
    /// ```
    ///
    pub NoAmbiguousAnchorText {
        version: "next",
        name: "noAmbiguousAnchorText",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoAmbiguousAnchorText {
    type Query = Ast<JsxOpeningElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAmbiguousAnchorTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let words = ctx.options().words();

        let name = binding.name().ok()?;
        let jsx_name = name.as_jsx_name()?;
        let value_token = jsx_name.value_token().ok()?;
        if value_token.to_string() != "a" {
            return None;
        }

        let parent = JsxElement::cast(binding.syntax().parent()?)?;
        let text = get_accessible_child_text(&parent);

        if words.contains(&text) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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

fn get_aria_label(node: &AnyJsxElement) -> Option<String> {
    let attribute = node.attributes().find_by_name("aria-label")?;
    let initializer = attribute.initializer()?;
    let value = initializer.value().ok()?;
    let jsx_string = value.as_jsx_string()?;
    let value_token = jsx_string.value_token().ok()?;

    Some(value_token.text_trimmed().to_string())
}

fn get_img_alt(node: &AnyJsxElement) -> Option<String> {
    let name = node.name().ok()?;
    let jsx_name = name.as_jsx_name()?;
    let value_token = jsx_name.value_token().ok()?;
    if value_token.to_string() != "img" {
        return None;
    }

    let attribute = node.attributes().find_by_name("alt")?;
    let initializer = attribute.initializer()?;
    let value = initializer.value().ok()?;
    let jsx_string = value.as_jsx_string()?;
    let value_token = jsx_string.value_token().ok()?;

    Some(value_token.text_trimmed().to_string())
}

fn standardize_space_and_case(input: String) -> String {
    input
        .chars()
        .filter(|c| !matches!(c, ',' | '.' | '?' | '¿' | '!' | '‽' | '¡' | ';' | ':'))
        .collect::<String>()
        .to_lowercase_cow()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_self_closing_accessible_text(node: &JsxSelfClosingElement) -> String {
    let any_jsx_element: AnyJsxElement = node.clone().into();
    if is_hidden_from_screen_reader(&any_jsx_element) {
        return String::new();
    }

    if let Some(aria_label) = get_aria_label(&any_jsx_element) {
        return standardize_space_and_case(aria_label);
    }

    if let Some(alt) = get_img_alt(&any_jsx_element) {
        return standardize_space_and_case(alt);
    }

    String::new()
}

fn get_accessible_child_text(node: &JsxElement) -> String {
    if let Ok(opening) = node.opening_element() {
        let any_jsx_element: AnyJsxElement = opening.clone().into();
        if is_hidden_from_screen_reader(&any_jsx_element) {
            return String::new();
        }

        if let Some(aria_label) = get_aria_label(&any_jsx_element) {
            return standardize_space_and_case(aria_label);
        }

        if let Some(alt) = get_img_alt(&any_jsx_element) {
            return standardize_space_and_case(alt);
        }
    };

    let raw_child_text = node
        .children()
        .into_iter()
        .map(|child| match child {
            AnyJsxChild::JsxText(element) => {
                if let Ok(value_token) = element.value_token() {
                    inner_string_text(&value_token).to_string()
                } else {
                    String::new()
                }
            }
            AnyJsxChild::JsxElement(element) => get_accessible_child_text(&element),
            AnyJsxChild::JsxSelfClosingElement(element) => {
                get_self_closing_accessible_text(&element)
            }
            _ => String::new(),
        })
        .collect::<Vec<String>>()
        .join(" ");

    standardize_space_and_case(raw_child_text)
}
