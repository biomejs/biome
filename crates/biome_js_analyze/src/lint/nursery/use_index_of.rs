mod callback_arrow_function_match;
mod callback_function_match;
mod extract_parameter_name;
mod find_index_comparable_expression;
mod types;

use crate::JsRuleAction;
use crate::lint::nursery::use_index_of::{
    callback_arrow_function_match::callback_arrow_function_match,
    callback_function_match::callback_function_match, types::JsSyntaxMatchPair,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression,
    JsCallExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxToken};

declare_lint_rule! {
    /// Prefer `Array#{indexOf,lastIndexOf}()` over `Array#{findIndex,findLastIndex}()` when looking for the index of an item.
    ///
    /// `Array#findIndex()` and `Array#findLastIndex()` are intended for more complex needs.
    /// If you are just looking for the index where the given item is present, then the code can be simplified to use Array#indexOf() or Array#lastIndexOf().
    /// This applies to any search with a literal, a variable, or any expression that doesn't have any explicit side effects.
    /// However, if the expression you are looking for relies on an item related to the function (its arguments, the function self, etc.), the case is still valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => x === 'foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => 'foo' === x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => {
    ///      return x === 'foo';
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(x => x === 'foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(x => 'foo' === x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(x => {
    ///      return x === 'foo';
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(function(x) {
    ///      return x === 'foo';
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const index = foo.indexOf('foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => x == undefined);
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => x !== 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex((x, index) => x === index);
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => (x === 'foo') && isValid());
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => y === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => y.x === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => {
    ///     const bar = getBar();
    ///     return x === bar;
    /// });
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(function(x) {
    ///     const bar = getBar();
    ///     return x === bar;
    /// });
    /// ```
    ///
    /// ```js
    /// const index = foo.lastIndexOf('foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => x == undefined);
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => x !== 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex((x, index) => x === index);
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => (x === 'foo') && isValid());
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => y === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => y.x === 'foo');
    /// ```
    ///
    pub UseIndexOf {
        version: "2.0.0",
        name: "useIndexOf",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintUnicorn("prefer-array-index-of")],
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseIndexOf {
    type Query = Ast<JsCallExpression>;
    type State = JsSyntaxMatchPair;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let member_expression = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        let member_name = member_expression.member_name()?;

        if !matches!(member_name.text(), "findIndex" | "findLastIndex") {
            return None;
        }

        let member_name_token = member_expression.syntax().last_token()?;
        let callback_function = call.arguments().ok()?.args().first()?.ok()?;
        match callback_function {
            AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsFunctionExpression(function)) => {
                callback_function_match(&function, member_name_token)
            }
            AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsArrowFunctionExpression(
                function,
            )) => callback_arrow_function_match(&function, member_name_token),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Prefer `Array#{indexOf,lastIndexOf}()` over `Array#{findIndex,findLastIndex}()` when looking for the index of an item."
                },
            )
                .note(markup! {
                "If the expression you are looking for relies on an item related to the function (its arguments, the function self, etc.), the case is still valid. This rule is fixable, unless the search expression has side effects."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, matched_expression: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let mut mutation = ctx.root().begin();
        let JsSyntaxMatchPair {
            matching_array_element,
            member_name,
        } = matched_expression;

        let old_member_name = member_name.text_trimmed();
        let new_member_name = match old_member_name {
            "findIndex" => "indexOf",
            "findLastIndex" => "lastIndexOf",
            _ => return None,
        };

        mutation.replace_token_discard_trivia(
            member_name.clone(),
            SyntaxToken::new_detached(member_name.kind(), new_member_name, [], []),
        );

        let arguments = call.arguments().ok()?;
        let old_arg = arguments.args().first()?.ok()?;

        mutation.replace_element_discard_trivia(
            old_arg.into_syntax().into(),
            matching_array_element.clone().into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
              "Replace `Array."{old_member_name} "()` with `Array." {new_member_name}"()`"
            },
            mutation,
        ))
    }
}
