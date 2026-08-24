use crate::HtmlRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    context::RuleContext,
    declare_lint_rule,
    shared::file_input_accept::{
        AcceptValueClassification, FileInputAcceptValue, InvalidAcceptValue,
    },
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyVueDirective, HtmlString, HtmlSyntaxKind, HtmlSyntaxToken, T,
    element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::no_invalid_file_input_accept::NoInvalidFileInputAcceptOptions;

declare_lint_rule! {
    /// Disallow invalid `accept` values on file inputs.
    ///
    /// An `accept` value must contain comma-separated filename extensions, MIME types, or
    /// the wildcard MIME types `audio/*`, `image/*`, and `video/*`.
    /// Browsers ignore invalid entries, so the file picker may not filter files as intended.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input type="file" accept="image/jpg">
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input type="file" accept="image/jpeg, .jpg">
    /// ```
    ///
    pub NoInvalidFileInputAccept {
        version: "next",
        name: "noInvalidFileInputAccept",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        sources: &[RuleSource::EslintUnicorn("no-invalid-file-input-accept").same()],
    }
}

pub struct State {
    range: TextRange,
    problem: Problem,
}

enum Problem {
    Missing,
    Invalid(InvalidAcceptValue),
    NeedsNormalization(HtmlString),
}

impl Rule for NoInvalidFileInputAccept {
    type Query = Ast<AnyHtmlTagElement>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoInvalidFileInputAcceptOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.tag_name_kind() != Some(T![input])
            || !has_single_attribute(element, "type")
            || !has_single_attribute(element, "accept")
        {
            return None;
        }

        let type_attribute = element.find_attribute_by_name("type")?;
        let accept_attribute = element.find_attribute_by_name("accept")?;
        if has_trailing_dynamic_spread(element, &type_attribute)
            || has_trailing_dynamic_spread(element, &accept_attribute)
        {
            return None;
        }
        let type_string = type_attribute.as_html_attribute()?.html_string()?;
        let input_type = type_string.inner_string_text().ok()?;
        if input_type.contains('&') || !input_type.eq_ignore_ascii_case("file") {
            return None;
        }

        let accept_attribute = accept_attribute.as_html_attribute()?;
        let Some(string) = accept_attribute.html_string() else {
            return accept_attribute.initializer().is_none().then(|| State {
                range: accept_attribute.range(),
                problem: Problem::Missing,
            });
        };
        let value = string.inner_string_text().ok()?;
        if value.contains('&') {
            return None;
        }

        let range = string
            .inner_string_range()
            .ok()
            .filter(|range| !range.is_empty())
            .unwrap_or_else(|| string.range());
        let problem = match FileInputAcceptValue::new(value.text()).classify() {
            AcceptValueClassification::Valid => return None,
            AcceptValueClassification::Invalid(error) => Problem::Invalid(error),
            AcceptValueClassification::NeedsNormalization => Problem::NeedsNormalization(string),
        };

        Some(State { range, problem })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match &state.problem {
            Problem::Missing => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { "The file input "<Emphasis>"accept"</Emphasis>" attribute has no value." },
            ),
            Problem::Invalid(error) => {
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! { "The file input "<Emphasis>"accept"</Emphasis>" value is invalid." },
                )
                .note(error.explanation())
            }
            Problem::NeedsNormalization(string) => {
                let value = string.inner_string_text().ok()?;
                let replacement = FileInputAcceptValue::new(value.text()).normalized().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! { "Use "<Emphasis>{replacement.as_str()}</Emphasis>" as the file input "<Emphasis>"accept"</Emphasis>" value." },
                )
            }
        };

        Some(diagnostic.note(markup! {
            "File input accept values must contain comma-separated filename extensions or valid MIME types."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let Problem::NeedsNormalization(string) = &state.problem else {
            return None;
        };
        let value = string.inner_string_text().ok()?;
        let replacement = FileInputAcceptValue::new(value.text()).normalized().ok()?;
        let old_token = string.value_token().ok()?;
        let old_text = old_token.text_trimmed();
        let quote = old_text
            .starts_with(['\'', '"'])
            .then(|| old_text.chars().next())
            .flatten();
        if quote.is_none() && replacement.contains(char::is_whitespace) {
            return None;
        }
        let replacement_text = if let Some(quote) = quote {
            format!("{quote}{replacement}{quote}")
        } else {
            replacement
        };
        let new_token = HtmlSyntaxToken::new_detached(
            HtmlSyntaxKind::HTML_STRING_LITERAL,
            &replacement_text,
            [],
            [],
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(old_token, new_token);

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the normalized file input "<Emphasis>"accept"</Emphasis>" value." }
                .to_owned(),
            mutation,
        ))
    }
}

fn has_trailing_dynamic_spread(
    element: &AnyHtmlTagElement,
    current_attribute: &AnyHtmlAttribute,
) -> bool {
    element.attributes().into_iter().any(|attribute| {
        if attribute.range().start() < current_attribute.range().end() {
            return false;
        }
        let has_static_name = attribute.name().is_some();
        match attribute {
            AnyHtmlAttribute::HtmlSpreadAttribute(_) => true,
            AnyHtmlAttribute::AnyVueDirective(
                AnyVueDirective::VueVBindShorthandDirective(_),
            ) => !has_static_name,
            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive)) => {
                directive.is_binding() && !has_static_name
            }
            _ => false,
        }
    })
}

fn has_single_attribute(element: &AnyHtmlTagElement, name: &str) -> bool {
    element
        .attributes()
        .into_iter()
        .filter(|attribute| attribute.is_attribute_or_vue_binding(name))
        .count()
        == 1
}
