use crate::JsRuleAction;
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
use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken, JsxString, jsx_ext::AnyJsxElement};
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
    /// ```jsx,expect_diagnostic
    /// <input type="file" accept="image/jpg" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input type="file" accept="image/jpeg, .jpg" />
    /// ```
    ///
    pub NoInvalidFileInputAccept {
        version: "next",
        name: "noInvalidFileInputAccept",
        language: "jsx",
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
    NeedsNormalization(JsxString),
}

impl Rule for NoInvalidFileInputAccept {
    type Query = Ast<AnyJsxElement>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoInvalidFileInputAcceptOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.is_custom_component()
            || element.name_value_token().ok()?.text_trimmed() != "input"
            || !has_single_attribute(element, "type")
            || !has_single_attribute(element, "accept")
        {
            return None;
        }

        let type_attribute = element.find_attribute_by_name("type")?;
        let accept_attribute = element.find_attribute_by_name("accept")?;
        if element.has_trailing_spread_prop(&type_attribute)
            || element.has_trailing_spread_prop(&accept_attribute)
        {
            return None;
        }
        let input_type = type_attribute
            .initializer()?
            .value()
            .ok()?
            .as_jsx_string()?
            .inner_string_text()
            .ok()?;
        if input_type.contains('&') || !input_type.eq_ignore_ascii_case("file") {
            return None;
        }

        let Some(initializer) = accept_attribute.initializer() else {
            return Some(State {
                range: accept_attribute.range(),
                problem: Problem::Missing,
            });
        };
        let string = initializer.value().ok()?.as_jsx_string()?.clone();
        let value = string.inner_string_text().ok()?;
        if value.contains('&') {
            return None;
        }

        let range = string.range();
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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let Problem::NeedsNormalization(string) = &state.problem else {
            return None;
        };
        let value = string.inner_string_text().ok()?;
        let replacement = FileInputAcceptValue::new(value.text()).normalized().ok()?;
        let old_token = string.value_token().ok()?;
        let quote = if old_token.text_trimmed().starts_with('\'') {
            '\''
        } else {
            '"'
        };
        let replacement_text = format!("{quote}{replacement}{quote}");
        let new_token = JsSyntaxToken::new_detached(
            JsSyntaxKind::JSX_STRING_LITERAL,
            &replacement_text,
            [],
            [],
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(old_token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the normalized file input "<Emphasis>"accept"</Emphasis>" value." }
                .to_owned(),
            mutation,
        ))
    }
}

fn has_single_attribute(element: &AnyJsxElement, name: &str) -> bool {
    element
        .attributes()
        .into_iter()
        .filter_map(|attribute| attribute.as_jsx_attribute().cloned())
        .filter_map(|attribute| attribute.name().ok())
        .filter_map(|name| name.name().ok())
        .filter(|token| token.text_trimmed() == name)
        .count()
        == 1
}
