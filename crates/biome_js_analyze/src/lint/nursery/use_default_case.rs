use std::ops::Not;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsLanguage, JsSwitchStatement};
use biome_rowan::{syntax::SyntaxTrivia, AstNode};

declare_rule! {
    /// Require default cases in switch statements.
    ///
    /// See https://eslint.org/docs/latest/rules/default-case
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case 1:
    ///         /* code */
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///     /* code */
    ///     break;
    ///
    /// default:
    ///     /* code */
    ///     break;
    /// }
    /// ```
    pub UseDefaultCase {
        version: "next",
        name: "useDefaultCase",
        recommended: false,
    }
}

impl Rule for UseDefaultCase {
    type Query = Ast<JsSwitchStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let has_case_clauses = node.cases().into_iter().len() > 0;
        let is_missing_default_case = node
            .cases()
            .into_iter()
            .find(|clause| clause.as_js_default_clause().is_some())
            .is_some()
            .not();

        let curly_token = node.r_curly_token().ok();

        let trivia_in_switch_block = if curly_token.is_some() {
            Some(curly_token.unwrap().leading_trivia())
        } else {
            None
        };

        let has_disable_comment_in_switch_block = has_disable_comment(trivia_in_switch_block);

        let has_disable_comment_in_switch_expression = node
            .discriminant()
            .ok()
            .into_iter()
            .filter(|token| token.as_js_identifier_expression().is_some())
            .find(|token| {
                let identifier_expression = token.as_js_identifier_expression().unwrap();

                if let Some(switch_identifier) = identifier_expression.name().ok() {
                    for token in switch_identifier.value_token().into_iter() {
                        if has_disable_comment(Some(token.trailing_trivia())) {
                            return true;
                        }
                    }
                }

                return false;
            })
            .is_some();

        let is_valid = is_missing_default_case
            && has_case_clauses
            && !has_disable_comment_in_switch_block
            && !has_disable_comment_in_switch_expression;

        is_valid.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Expected a default case."
            },
        ))
    }
}

fn has_disable_comment(trivia: Option<SyntaxTrivia<JsLanguage>>) -> bool {
    if trivia.is_some() {
        let comments = trivia.unwrap().pieces().filter(|token| token.is_comments());
        let last_comment = comments.last();

        return match last_comment {
            Some(comment) => comment.text().to_lowercase().contains("no default"),
            None => false,
        };
    }

    false
}
