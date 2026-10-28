use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsBinaryExpression, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsIdentifierExpression, JsNullLiteralExpression, JsSyntaxKind,
    JsUnaryExpression, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_null::NoNullOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow the use of the `null` literal.
    ///
    /// JavaScript has two values that mean "no value": `null` and `undefined`.
    /// Code that uses both has to check for both, and it's easy to forget one of them.
    /// Using only `undefined` keeps missing values consistent across a codebase.
    ///
    /// Strict comparisons such as `foo === null` are allowed, because code often needs to check for `null` values that come from other libraries or from APIs such as `JSON.parse()`.
    ///
    /// Some common APIs require `null` as an argument and are allowed:
    /// - `Object.create(null)`, which creates an object with no prototype.
    /// - `useRef(null)` and `React.useRef(null)`, the usual way to create a React ref for an element.
    /// - `parent.insertBefore(node, null)`, which adds `node` at the end of `parent`.
    ///
    /// The fix replaces `null` with `undefined`.
    /// It isn't applied automatically, because other code may check for `null` with `===`, and TypeScript types that include `null` may not accept `undefined`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (bar == null) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function getUser() {
    ///     return null;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo(null);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo;
    /// if (bar == undefined) {}
    /// if (bar === null) {}
    /// const dictionary = Object.create(null);
    /// const ref = useRef(null);
    /// parent.insertBefore(node, null);
    /// ```
    ///
    /// ## See Also
    ///
    /// - To remove `undefined` that is assigned to a variable when it is declared, see [`noUselessUndefinedInitialization`](https://biomejs.dev/linter/rules/no-useless-undefined-initialization/).
    ///
    pub NoNull {
        version: "2.6.4",
        name: "noNull",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-null").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoNull {
    type Query = Ast<JsNullLiteralExpression>;
    type State = NullUsage;
    type Signals = Option<Self::State>;
    type Options = NoNullOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = AnyJsExpression::AnyJsLiteralExpression(ctx.query().clone().into());
        let outer = node.outer_expression()?;

        if let Some(binary) = outer.parent::<JsBinaryExpression>() {
            let operator = binary.operator_token().ok()?;
            if STRICT_EQUALITY_OPERATORS.contains(operator.kind()) {
                return None;
            }
        }

        if is_allowed_argument(&outer) {
            return None;
        }

        // `delete undefined` is an error in strict mode code, so there is nothing to replace
        // the operand with.
        let is_delete_operand = outer.parent::<JsUnaryExpression>().is_some_and(|unary| {
            unary
                .operator_token()
                .is_ok_and(|operator| operator.kind() == T![delete])
        });

        Some(if is_delete_operand {
            NullUsage::DeleteOperand
        } else {
            NullUsage::Value
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Unexpected use of "<Emphasis>"null"</Emphasis>"."
            },
        )
        .note(markup! {
            "Using both "<Emphasis>"null"</Emphasis>" and "<Emphasis>"undefined"</Emphasis>" for missing values means code has to check for both."
        });
        Some(match state {
            NullUsage::DeleteOperand => diagnostic,
            NullUsage::Value => diagnostic.note(markup! {
                "Use "<Emphasis>"undefined"</Emphasis>" instead."
            }),
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if *state == NullUsage::DeleteOperand {
            return None;
        }

        let node = AnyJsExpression::AnyJsLiteralExpression(ctx.query().clone().into());
        let undefined = AnyJsExpression::JsIdentifierExpression(make::js_identifier_expression(
            make::js_reference_identifier(make::ident("undefined")),
        ));

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node, undefined);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>"undefined"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Where the reported `null` appears.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullUsage {
    /// The operand of a `delete` expression, such as `delete null`.
    DeleteOperand,
    /// Any other position.
    Value,
}

const STRICT_EQUALITY_OPERATORS: TokenSet<JsSyntaxKind> = token_set![T![===], T![!==]];

/// Returns `true` if `argument` is a `null` argument that one of these calls requires:
///
/// - `Object.create(null)` and `Object.create(null, properties)`
/// - `useRef(null)` and `React.useRef(null)`
/// - `parent.insertBefore(node, null)`
///
/// Optional calls such as `useRef?.(null)` don't match. Neither do calls with spread
/// arguments, because the position of each argument is unknown.
fn is_allowed_argument(argument: &AnyJsExpression) -> bool {
    let Some(call) = argument
        .parent::<JsCallArgumentList>()
        .and_then(|list| list.parent::<JsCallArguments>())
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
    else {
        return false;
    };
    if call.is_optional() {
        return false;
    }
    let Ok(arguments) = call.arguments() else {
        return false;
    };
    let arguments = arguments.args();
    let count = arguments.len();
    if count > 2
        || arguments
            .iter()
            .any(|argument| !matches!(argument, Ok(AnyJsCallArgument::AnyJsExpression(_))))
    {
        return false;
    }
    let is_argument_at = |index: usize| {
        arguments
            .iter()
            .nth(index)
            .and_then(Result::ok)
            .is_some_and(|candidate| candidate.syntax() == argument.syntax())
    };

    match call.callee().map(AnyJsExpression::omit_parentheses) {
        // `useRef(null)`
        Ok(AnyJsExpression::JsIdentifierExpression(callee)) => {
            count == 1 && is_identifier(&callee, "useRef")
        }
        Ok(AnyJsExpression::JsStaticMemberExpression(callee)) => {
            let Some(method) = callee
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
            else {
                return false;
            };
            let object = callee.object().map(AnyJsExpression::omit_parentheses);
            let object = match &object {
                Ok(AnyJsExpression::JsIdentifierExpression(object)) => Some(object),
                _ => None,
            };
            match method.text_trimmed() {
                // `parent.insertBefore(node, null)` and `parent?.insertBefore(node, null)`
                "insertBefore" => count == 2 && is_argument_at(1),
                // `Object.create(null)` and `Object.create(null, properties)`
                "create" => {
                    !callee.is_optional()
                        && object.is_some_and(|object| is_identifier(object, "Object"))
                        && is_argument_at(0)
                }
                // `React.useRef(null)`
                "useRef" => {
                    !callee.is_optional()
                        && count == 1
                        && object.is_some_and(|object| is_identifier(object, "React"))
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn is_identifier(expression: &JsIdentifierExpression, name: &str) -> bool {
    expression
        .name()
        .and_then(|reference| reference.value_token())
        .is_ok_and(|token| token.text_trimmed() == name)
}
