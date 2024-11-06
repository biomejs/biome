use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, AnyJsCallArgument, AnyJsExpression, JsNewOrCallExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::{services::semantic::Semantic, JsRuleAction};

declare_lint_rule! {
    /// Disallow Array constructors.
    ///
    /// Use of the Array constructor to construct a new array is generally discouraged in favor of array literal notation because of the single-argument pitfall and because the Array global may be redefined.
    /// The exception is when the Array constructor intentionally creates sparse arrays of a specified size by giving the constructor a single numeric argument.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = new Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array(...args);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const xs = Array(65000);
    /// ```
    ///
    /// ```js
    /// const xs = [0, 1, 2];
    /// ```
    ///
    pub UseArrayLiterals {
        version: "1.7.2",
        name: "useArrayLiterals",
        language: "js",
        sources: &[RuleSource::Eslint("no-array-constructor"), RuleSource::EslintTypeScript("no-array-constructor")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseArrayLiterals {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&callee)?;
        if name.text() != "Array" || ctx.model().binding(&reference).is_some() {
            return None;
        }
        if callee.syntax() != reference.syntax()
            && !reference
                .value_token()
                .is_ok_and(|name| matches!(name.text_trimmed(), "globalThis" | "window" | "Array"))
        {
            return None;
        }
        let Some(arguments) = node.arguments() else {
            return if matches!(node, JsNewOrCallExpression::JsNewExpression(_)) {
                // Report `new Array`
                Some(())
            } else {
                // ignore `Array`
                None
            };
        };
        let [arg1, arg2] = arguments.get_arguments_by_index([0, 1]);
        if arg1.is_some() && arg2.is_none() && !matches!(arg1?, AnyJsCallArgument::JsSpread(_)) {
            // Ignore `Array(length)`
            return None;
        }
        // Report `Array()`, `Array(x, y)`, and `Array(...xs)`
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use an array literal instead of the "<Emphasis>"Array"</Emphasis>" constructor."
                },
            )
            .note(markup! {
                "The "<Emphasis>"Array"</Emphasis>" constructor is misleading because it can be used to preallocate an array of a given length or to create an array with a given list of elements."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if node
            .syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT)
        {
            // Ignore useless expression statements.
            // This avoids issues with missing semicolons.
            return None;
        }
        let mut mutation = ctx.root().begin();
        let new_node = if let Some(args) = node.arguments() {
            let l_paren_trailing_trivia = args.l_paren_token().ok()?.trailing_trivia().pieces();
            let r_paren_leading_trivia = args.r_paren_token().ok()?.leading_trivia().pieces();
            let args = args.args();
            let items = args
                .elements()
                .flat_map(|item| item.into_node())
                .map(|item| item.into())
                .collect::<Vec<_>>();
            let separators = args.separators().flatten().collect::<Vec<_>>();
            make::js_array_expression(
                make::token(T!['[']).append_trivia_pieces(l_paren_trailing_trivia),
                make::js_array_element_list(items, separators),
                make::token(T![']']).prepend_trivia_pieces(r_paren_leading_trivia),
            )
        } else {
            // `new Array` -> `[]`
            make::js_array_expression(
                make::token(T!['[']),
                make::js_array_element_list([], []),
                make::token(T![']']),
            )
        };
        mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_node.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an array literal." }.to_owned(),
            mutation,
        ))
    }
}
