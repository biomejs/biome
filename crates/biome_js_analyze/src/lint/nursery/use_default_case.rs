use std::ops::Not;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{JsLanguage, JsSwitchStatement};
use biome_rowan::{syntax::SyntaxTrivia, AstNode};
use regex::Regex;
use serde::{Deserialize, Serialize};

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
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     // no default
    /// 	case 1:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case 1:
    ///         break;
    ///     // no default
    ///     // nope
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///         /* code */
    ///         break;
    ///
    ///     default:
    ///         /* code */
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///         /* code */
    ///         break;
    ///
    ///     // no default
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    /// }
    /// ```
    pub UseDefaultCase {
        version: "next",
        name: "useDefaultCase",
        recommended: false,
    }
}

/// Options for the rule `useDefaultCase`.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseDefaultCaseOptions {
    /// Regular expression that filters the comment pattern to disable the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_pattern: Option<String>,
}

impl Rule for UseDefaultCase {
    type Query = Ast<JsSwitchStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = Box<UseDefaultCaseOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let comment_pattern = options
            .comment_pattern
            .clone()
            .unwrap_or(String::from("^no default$"))
            .replace("\\\\", "\\");

        let has_case_clauses = node.cases().into_iter().len() > 0;
        let is_missing_default_case = node
            .cases()
            .into_iter()
            .any(|clause| clause.as_js_default_clause().is_some())
            .not();

        let curly_token = node.r_curly_token().ok();
        let trivia_in_switch_block = curly_token.map(|c| c.leading_trivia());

        let has_disable_comment_in_switch_block =
            has_disable_comment(trivia_in_switch_block, &comment_pattern);

        let has_disable_comment_in_switch_expression = node
            .discriminant()
            .ok()
            .into_iter()
            .filter(|token| token.as_js_identifier_expression().is_some())
            .any(|token| {
                let identifier_expression = token.as_js_identifier_expression().unwrap();

                if let Ok(switch_identifier) = identifier_expression.name() {
                    if let Ok(token) = switch_identifier.value_token() {
                        if has_disable_comment(Some(token.trailing_trivia()), &comment_pattern) {
                            return true;
                        }
                    }
                }

                false
            });

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

fn has_disable_comment(trivia: Option<SyntaxTrivia<JsLanguage>>, comment_pattern: &str) -> bool {
    if trivia.is_some() {
        let comment_regex = Regex::new(comment_pattern).unwrap();
        let comments = trivia.unwrap().pieces().filter(|token| token.is_comments());
        let last_comment = comments.last();

        return match last_comment {
            Some(comment) => {
                let comment_text = comment
                    .text()
                    .to_lowercase()
                    .replace("//", "")
                    .replace("/*", "")
                    .replace("*/", "");

                let is_disable_comment =
                    comment_regex.captures_iter(comment_text.trim()).count() > 0;
                is_disable_comment
            }
            None => false,
        };
    }

    false
}
