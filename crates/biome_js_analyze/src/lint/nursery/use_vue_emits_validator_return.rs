use crate::services::vue::VueEmit;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsExpression, AnyJsFunctionBody, JsClassDeclaration,
    JsFunctionDeclaration, JsLanguage, JsReturnStatement,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::use_vue_emits_validator_return::UseVueEmitsValidatorReturnOptions;

declare_lint_rule! {
    /// Require the validator function of a Vue event to return a true value when the event is valid.
    ///
    /// A component can check the values it passes along with an event by giving the event a
    /// validator function in the `emits` option or in `defineEmits()`. Vue calls the validator
    /// each time the event is emitted. When the validator doesn't return a true value, Vue prints
    /// a warning in development mode.
    ///
    /// A validator that never returns a value, or that only returns values such as `false`, `0`,
    /// `""`, `null` or `undefined`, makes Vue print that warning every time the event is emitted,
    /// even when its values are correct. This is usually a mistake, such as a forgotten `return`.
    ///
    /// The rule reports a validator when none of its `return` statements can return a true value.
    /// A `return` statement inside another function written in the validator doesn't count.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   emits: {
    ///     submit(payload) {
    ///       payload.email.includes("@");
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineEmits({
    ///   close: () => false,
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// defineEmits({
    ///   submit(payload) {
    ///     return payload.email.includes("@");
    ///   },
    ///   select(id) {
    ///     if (typeof id !== "number") {
    ///       return false;
    ///     }
    ///     return true;
    ///   },
    ///   close: null,
    /// });
    /// </script>
    /// ```
    ///
    pub UseVueEmitsValidatorReturn {
        version: "next",
        name: "useVueEmitsValidatorReturn",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("return-in-emits-validator").same()],
    }
}

impl Rule for UseVueEmitsValidatorReturn {
    type Query = VueEmit;
    type State = ValidatorReturns;
    type Signals = Option<Self::State>;
    type Options = UseVueEmitsValidatorReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let range = ctx.query().validator_range()?;
        let root = ctx.root();
        if !root.syntax().text_range_with_trivia().contains_range(range) {
            return None;
        }
        let function = AnyFunctionLike::cast(root.syntax().covering_element(range).into_node()?)?;
        let body = match function.body().ok()? {
            AnyJsFunctionBody::AnyJsExpression(expression) => {
                return is_falsy(&expression).then_some(ValidatorReturns::OnlyFalsyValues);
            }
            AnyJsFunctionBody::JsFunctionBody(body) => body,
        };

        let mut returns_value = false;
        let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(argument) =
                JsReturnStatement::cast(node).and_then(|statement| statement.argument())
            {
                if !is_falsy(&argument) {
                    return None;
                }
                returns_value = true;
            }
            preorder.skip_subtree();
        }
        Some(if returns_value {
            ValidatorReturns::OnlyFalsyValues
        } else {
            ValidatorReturns::NoValue
        })
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().validator_range()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let emit = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                emit.validator_range()?,
                markup! {
                    "The validator of the event "<Emphasis>{emit.name()}</Emphasis>" never returns "{state}"."
                },
            )
            .note(markup! {
                "Vue prints a warning in development mode each time the event is emitted, because the validator never accepts its values."
            })
            .note(markup! {
                "Return "<Emphasis>"true"</Emphasis>" when the values of the event are valid, and "<Emphasis>"false"</Emphasis>" otherwise."
            }),
        )
    }
}

/// What the `return` statements of an event validator return, when none of
/// them can return a true value.
#[derive(Clone, Copy, Debug)]
pub enum ValidatorReturns {
    /// The validator has no `return` statement with a value.
    NoValue,
    /// Every `return` statement with a value returns a value that is always
    /// false, such as `false`, `0` or `null`.
    OnlyFalsyValues,
}

impl biome_console::fmt::Display for ValidatorReturns {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::NoValue => "a value",
            Self::OnlyFalsyValues => "a true value",
        })
    }
}

/// The nodes the search for `return` statements stops at: the statements
/// themselves, and the nodes whose `return` statements belong to a nested
/// function.
///
/// An expression can only hold a `return` statement inside a function or
/// class written in it, so the search never needs to look inside one.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsReturnStatement::KIND_SET
    .union(AnyJsExpression::KIND_SET)
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsClassDeclaration::KIND_SET);

/// Returns `true` when `expression` always evaluates to a false value:
/// `false`, `0`, `0n`, `""`, `null`, `undefined` or `NaN`.
fn is_falsy(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    if let Some(number) = expression
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression())
    {
        // Zero can be written in several ways, such as `0.0`, `0x0` or `0e0`.
        return number.as_number().is_some_and(|value| value == 0.0);
    }
    if let Some(value) = expression.as_static_value() {
        return value.is_falsy();
    }
    expression
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok())
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == "NaN")
}
