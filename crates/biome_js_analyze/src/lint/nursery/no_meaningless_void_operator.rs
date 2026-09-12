use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsExpressionStatement, JsUnaryExpression, T,
};
use biome_module_graph::type_inference::TypeInferenceClassification;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_meaningless_void_operator::NoMeaninglessVoidOperatorOptions;

use crate::{JsRuleAction, services::typed::Typed};

declare_lint_rule! {
    /// Disallow `void` when it does not discard a call's return value or a thenable.
    ///
    /// Using `void` communicates that a value is deliberately ignored. Applying it to
    /// a call that already returns `void` or `undefined` obscures that intent and can
    /// hide changes to the called API. Non-call operands are also reported, except
    /// for thenables and the common `void 0` idiom.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-call.ts
    /// declare function log(): void;
    /// void log();
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-value.js
    /// void 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// declare function value(): number;
    /// void value();
    /// void Promise.resolve();
    /// void 0;
    /// ```
    pub NoMeaninglessVoidOperator {
        version: "next",
        name: "noMeaninglessVoidOperator",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-meaningless-void-operator").same()],
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum MeaninglessVoid {
    Call,
    NonCall,
}

impl Rule for NoMeaninglessVoidOperator {
    type Query = Typed<JsUnaryExpression>;
    type State = MeaninglessVoid;
    type Signals = Option<Self::State>;
    type Options = NoMeaninglessVoidOperatorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_void().ok()? {
            return None;
        }
        let argument = node.argument().ok()?;
        let inner = argument.inner_expression()?;
        let mut type_argument = argument.clone().omit_parentheses();
        while let AnyJsExpression::TsSatisfiesExpression(node) = type_argument {
            type_argument = node.expression().ok()?.omit_parentheses();
        }
        if matches!(inner, AnyJsExpression::JsCallExpression(_)) {
            // Classification preserves overload uncertainty that normalization can erase.
            if ctx.classify_expression_as_promise(&type_argument)
                != TypeInferenceClassification::NoMatch
            {
                return None;
            }
            return (ctx
                .type_of_expression(&type_argument)?
                .is_void_like()
                == Some(true))
            .then_some(MeaninglessVoid::Call);
        }

        if let AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) = inner
            && number.as_number()? == 0.0
        {
            return None;
        }

        while let AnyJsExpression::TsNonNullAssertionExpression(node) = type_argument {
            type_argument = node.expression().ok()?.omit_parentheses();
        }
        match ctx.classify_expression_as_promise(&type_argument) {
            TypeInferenceClassification::Match | TypeInferenceClassification::Indeterminate => {
                return None;
            }
            TypeInferenceClassification::NoMatch => {}
        }
        match ctx.classify_expression_as_thenable(&type_argument) {
            TypeInferenceClassification::Match | TypeInferenceClassification::Indeterminate => None,
            TypeInferenceClassification::NoMatch => Some(MeaninglessVoid::NonCall),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! { "This "<Emphasis>"void"</Emphasis>" operator is unnecessary." },
        ).note(match state {
            MeaninglessVoid::Call => "This call already returns void or undefined.",
            MeaninglessVoid::NonCall => "This operand is neither a function call nor a thenable.",
        }).note(markup! {
            "Use "<Emphasis>"void"</Emphasis>" to explicitly discard a call's return value or a thenable."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if matches!(state, MeaninglessVoid::NonCall)
            && node.parent::<JsExpressionStatement>().is_none()
        {
            return None;
        }
        let operator = node.operator_token().ok()?;
        // Parentheses must not turn an ASI boundary into a call on the preceding value.
        if node.parent::<JsExpressionStatement>().is_some()
            && let Some(previous) = operator.prev_token()
            && !matches!(previous.kind(), T![;] | T!['{'])
        {
            return None;
        }
        let argument = node.argument().ok()?;
        let trailing = argument.syntax().last_token()?.trailing_trivia();
        let replacement = make::js_parenthesized_expression(
            make::token(T!['('])
                .with_leading_trivia_pieces(operator.leading_trivia().pieces())
                .with_trailing_trivia_pieces(
                    operator
                        .trailing_trivia()
                        .pieces()
                        .skip_while(|piece| piece.is_whitespace())
                        .collect::<Vec<_>>(),
                ),
            argument.with_trailing_trivia_pieces([])?,
            make::token(T![')']).with_trailing_trivia_pieces(trailing.pieces()),
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            AnyJsExpression::from(node.clone()),
            AnyJsExpression::from(replacement),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"void"</Emphasis>" operator." }.to_owned(),
            mutation,
        ))
    }
}
