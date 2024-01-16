use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxChild, JsxText, TriviaPieceKind, T};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;
declare_rule! {
    /// Prevent comments from being inserted as text nodes
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a3 = <div>// comment</div>;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a4 = <div>/* comment */</div>;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a5 = <div>/** comment */</div>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = <div>{/* comment */}</div>;
    /// const a1 = <div>{/** comment */}</div>;
    /// const a2 = <div className={"cls" /* comment */}></div>;
    /// ```
    pub(crate) NoCommentText {
        version: "1.0.0",
        name: "noCommentText",
        source: RuleSource::EslintReact("jsx-no-comment-textnodes"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoCommentText {
    type Query = Ast<JsxText>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let jsx_value = n.text();
        let is_single_line_comment = jsx_value.starts_with("//");
        let is_multi_line_comment = jsx_value.starts_with("/*") && jsx_value.ends_with("*/");
        if is_single_line_comment || is_multi_line_comment {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Wrap "<Emphasis>"comments"</Emphasis>" inside children within "<Emphasis>"braces"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let normalized_comment = format!(
            "/*{}*/",
            node.text()
                .trim_start_matches("/**")
                .trim_start_matches("//")
                .trim_start_matches("/*")
                .trim_end_matches("*/")
        );

        mutation.replace_node(
            AnyJsxChild::from(node.clone()),
            AnyJsxChild::from(
                make::jsx_expression_child(
                    make::token(T!['{']).with_trailing_trivia([(
                        TriviaPieceKind::MultiLineComment,
                        normalized_comment.as_str(),
                    )]),
                    make::token(T!['}']),
                )
                .build(),
            ),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the comments with braces" }.to_owned(),
            mutation,
        })
    }
}
