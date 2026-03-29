use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsName, JsCallExpression, is_in_boolean_context};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_regexp_test::UseRegexpTestOptions;

use crate::JsRuleAction;
use crate::services::typed::Typed;

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
    /// ```js
    /// if ("hello world".match(/hello/)) {}
    /// ```
    ///
    /// ```js
    /// if (/hello/.exec("hello world")) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (/hello/.test("hello world")) {}
    ///
    /// const result = "hello world".match(/hello/);
    /// ```
    ///
    pub UseRegexpTest {
        version: "next",
        name: "useRegexpTest",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-regexp-test").same()],
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Unsafe,
    }
}

pub struct UseRegexpTextState {
    string_node: AnyJsExpression,
    regexp_node: AnyJsExpression,
}

impl Rule for UseRegexpTest {
    type Query = Typed<JsCallExpression>;
    type State = UseRegexpTextState;
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
        let call_name = callee.member().ok()?.as_js_name()?.to_trimmed_text();

        let args = node.arguments().ok()?.args();

        if args.len() != 1 {
            return None;
        }

        let first_arg = args.first()?.ok()?;
        let first_arg_expr = first_arg.as_any_js_expression()?;

        let (string_node, regexp_node) = if call_name == "match" {
            (call_object, first_arg_expr)
        } else if call_name == "exec" {
            (first_arg_expr, call_object)
        } else {
            return None;
        };

        let regexp_type = ctx.type_of_expression(regexp_node);

        if !regexp_type.is_regexp_literal() && !regexp_type.is_regexp_instance() {
            return None;
        }

        Some(UseRegexpTextState {
            regexp_node: regexp_node.clone(),
            string_node: string_node.clone(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let UseRegexpTextState {
            regexp_node,
            string_node,
        } = state;

        let regexp_text = regexp_node.to_trimmed_string();
        let string_text = string_node.to_trimmed_string();

        let suggestion = format!("{regexp_text}.test({string_text})");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>{suggestion}</Emphasis>" instead."
                },
            )
            .note(markup! {
                <Emphasis>"RegExp.test()"</Emphasis>" returns a boolean, which is more appropriate and efficient in a boolean context."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let UseRegexpTextState {
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

        let regexp_text = regexp_node.to_trimmed_string();
        let string_text = string_node.to_trimmed_string();

        let suggestion = format!("{regexp_text}.test({string_text})");

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace with "<Emphasis>{suggestion}</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}
