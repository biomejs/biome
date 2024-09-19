use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{global_identifier, AnyJsExpression, JsNewOrCallExpression};
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce passing a message value when creating a built-in error.
    ///
    /// This rule enforces a message value to be passed in when creating an instance of a built-in `Error` object,
    /// which leads to more readable and debuggable code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// throw Error();
    /// ```
    /// ```js,expect_diagnostic
    /// throw Error('');
    /// ```
    /// ```js,expect_diagnostic
    /// throw new TypeError();
    /// ```
    /// ```js,expect_diagnostic
    /// const error = new AggregateError(errors);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// throw Error('Unexpected property.');
    /// ```
    /// ```js
    /// throw new TypeError('Array expected.');
    /// ```
    /// ```js
    /// const error = new AggregateError(errors, 'Promises rejected.');
    /// ```
    pub UseErrorMessage {
        version: "1.8.0",
        name: "useErrorMessage",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("error-message")],
        recommended: false,
    }
}

impl Rule for UseErrorMessage {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseErrorMessageRule;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let (reference, name) = global_identifier(&callee.omit_parentheses())?;
        let name_text = name.text();

        if BUILTIN_ERRORS.binary_search(&name_text).is_err()
            || ctx.model().binding(&reference).is_some()
        {
            return None;
        }

        let argument_position = if name_text == "AggregateError" { 1 } else { 0 };
        let arguments = node.arguments()?;

        let has_spread = arguments
            .args()
            .into_iter()
            .take(argument_position + 1)
            .map_while(|arg| arg.ok())
            .any(|arg| arg.as_js_spread().is_some());

        if has_spread {
            return None;
        }

        let Some(arg) = arguments
            .args()
            .into_iter()
            .nth(argument_position)
            .and_then(|a| a.ok())
        else {
            return Some(UseErrorMessageRule::MissingMessage);
        };

        match arg.as_any_js_expression()? {
            AnyJsExpression::AnyJsLiteralExpression(literal) => {
                let Some(string_literal) = literal.as_js_string_literal_expression() else {
                    return Some(UseErrorMessageRule::NotString);
                };

                let text = string_literal.inner_string_text().ok()?;
                if text.trim().is_empty() {
                    return Some(UseErrorMessageRule::EmptyString);
                }

                None
            }
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.elements().into_iter().count() == 0 {
                    return Some(UseErrorMessageRule::EmptyString);
                }

                None
            }
            AnyJsExpression::JsArrayExpression(_) | AnyJsExpression::JsObjectExpression(_) => {
                Some(UseErrorMessageRule::NotString)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query().arguments()?;

        let message = match state {
            UseErrorMessageRule::MissingMessage => "Provide an error message for the error.",
            UseErrorMessageRule::EmptyString => "Error message should not be an empty string.",
            UseErrorMessageRule::NotString => "Error message should be a string.",
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { {message} },
            )
            .note(markup! {
                "Providing meaningful error messages leads to more readable and debuggable code."
            }),
        )
    }
}

/// Sorted array of builtins errors requiring an error message.
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error
const BUILTIN_ERRORS: &[&str] = &[
    "AggregateError",
    "Error",
    "EvalError",
    "InternalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
];

pub enum UseErrorMessageRule {
    MissingMessage,
    EmptyString,
    NotString,
}

#[test]
fn test_order() {
    for items in BUILTIN_ERRORS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
