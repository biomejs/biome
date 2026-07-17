use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsName, JsCallExpression, JsNewExpression, is_in_boolean_context,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_regexp_test::UseRegexpTestOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of `RegExp.prototype.test()` over `String.prototype.match()` and `RegExp.prototype.exec()` in boolean contexts.
    ///
    /// When checking whether a string matches a regular expression, `RegExp.prototype.test()` is more appropriate
    /// than `String.prototype.match()` and `RegExp.prototype.exec()` because it returns a boolean directly.
    /// In contrast, `match()` and `exec()` return match objects or arrays, which involves unnecessary computation
    /// when only a true/false result is needed.
    ///
    /// The fix is marked as unsafe because `match()` and `exec()` can have side effects when used with
    /// [global or sticky](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex) regular expressions,
    /// since they advance the `lastIndex` property differently than `test()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if ("hello world".match(/hello/)) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (/hello/.exec("hello world")) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (/hello/.test("hello world")) {}
    /// ```
    ///
    pub UseRegexpTest {
        version: "2.4.13",
        name: "useRegexpTest",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-regexp-test").same()],
        fix_kind: FixKind::Unsafe,
    }
}

pub struct UseRegexpTestState {
    string_node: AnyJsExpression,
    regexp_node: AnyJsExpression,
}

impl Rule for UseRegexpTest {
    type Query = Ast<JsCallExpression>;
    type State = UseRegexpTestState;
    type Signals = Option<Self::State>;
    type Options = UseRegexpTestOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !is_in_boolean_context(node.syntax()).unwrap_or(false) {
            return None;
        }

        let binding = node.callee().ok()?.omit_parentheses();
        let callee = binding.as_js_static_member_expression()?;
        let call_object = &callee.object().ok()?;
        let call_name = callee
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed();

        let args = node.arguments().ok()?.args();

        if args.len() != 1 {
            return None;
        }

        let first_arg = args.first()?.ok()?;
        let first_arg_expr = first_arg.as_any_js_expression()?;

        let (string_node, regexp_node) = if call_name.text() == "match" {
            (call_object, first_arg_expr)
        } else if call_name.text() == "exec" {
            (first_arg_expr, call_object)
        } else {
            return None;
        };

        if !is_regexp(regexp_node) {
            return None;
        }

        Some(UseRegexpTestState {
            regexp_node: regexp_node.clone(),
            string_node: string_node.clone(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                <Emphasis>".match()"</Emphasis>" and "<Emphasis>".exec()"</Emphasis>" are not designed for boolean checks."
            },
        )
        .note(markup! {
            "These methods return match objects or arrays, which involves unnecessary computation when only checking for a match."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let UseRegexpTestState {
            string_node,
            regexp_node,
        } = state;

        let binding = node.callee().ok()?.omit_parentheses();
        let callee = binding.as_js_static_member_expression()?;
        let call_object = &callee.object().ok()?;
        let call_name = callee.member().ok()?;

        let args = node.arguments().ok()?.args();
        let first_arg = args.first()?.ok()?;
        let first_arg_expr = first_arg.as_any_js_expression()?;

        mutation.replace_node(call_object.clone(), regexp_node.clone());
        mutation.replace_node(
            call_name,
            AnyJsName::JsName(make::js_name(make::ident("test"))),
        );
        mutation.replace_node(first_arg_expr.clone(), string_node.clone());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use .test() instead."
            },
            mutation,
        ))
    }
}

/// Returns `true` if the expression is a regex literal (`/pattern/`) or a
/// `new RegExp(...)` / `new window.RegExp(...)` / `new globalThis.RegExp(...)`
/// constructor call.
fn is_regexp(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.as_js_regex_literal_expression().is_some()
        }
        AnyJsExpression::JsNewExpression(new_expr) => is_regexp_constructor(&new_expr),
        _ => false,
    }
}

fn is_regexp_constructor(expr: &JsNewExpression) -> bool {
    let Ok(callee) = expr.callee() else {
        return false;
    };

    match callee {
        AnyJsExpression::JsIdentifierExpression(id) => id
            .name()
            .ok()
            .and_then(|n| n.value_token().ok())
            .is_some_and(|t| t.token_text_trimmed().text() == "RegExp"),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object_is_global = member
                .object()
                .ok()
                .and_then(|obj| obj.as_js_identifier_expression()?.name().ok()?.value_token().ok())
                .is_some_and(|t| {
                    let name = t.token_text_trimmed();
                    name.text() == "window" || name.text() == "globalThis"
                });

            if !object_is_global {
                return false;
            }

            member
                .member()
                .ok()
                .and_then(|m| m.as_js_name()?.value_token().ok())
                .is_some_and(|t| t.token_text_trimmed().text() == "RegExp")
        }
        _ => false,
    }
}
