use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxChild, JsSyntaxKind, JsSyntaxToken, JsxText};
use biome_rowan::{AstNode, BatchMutationExt};
use regex::Regex;
use std::sync::LazyLock;

static COMMENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//.*|/\*[\s\S]*?\*/").unwrap());

declare_lint_rule! {
    /// Prevent comments from being inserted as text nodes
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div>// comment</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>/* comment */</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>/** comment */</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>text /* comment */</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>/* comment */ text</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>
    ///     text
    ///     // comment
    /// </div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>
    ///     // comment
    ///    text
    /// </div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>
    ///     /* comment */
    ///     text
    /// </div>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///    <div>{/* comment */}</div>;
    ///    <div>{/** comment */}</div>;
    ///    <div className={"cls" /* comment */}></div>;
    ///    <div>text {/* comment */}</div>;
    ///    <div>{/* comment */} text</div>;
    /// </>
    /// ```
    pub NoCommentText {
        version: "1.0.0",
        name: "noCommentText",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-comment-textnodes")],
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
        if !COMMENT_REGEX.is_match(&jsx_value) {
            return None;
        }

        Some(())
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
        let node_text = node.text().to_string();

        // Replace the comments with JSX comments
        let new_jsx_value = COMMENT_REGEX.replace_all(&node_text, |caps: &regex::Captures| {
            let comment = caps[0].trim();
            match comment {
                c if c.starts_with("//") => format!("{{/* {} */}}", c[2..].trim()),
                c if c.starts_with("/**") => format!("{{/** {} */}}", &c[3..c.len() - 2].trim()),
                c => format!("{{/* {} */}}", &c[2..c.len() - 2].trim()),
            }
        });

        // Create a new JSX text node with the new value
        let new_jsx_text = AnyJsxChild::JsxText(make::jsx_text(JsSyntaxToken::new_detached(
            JsSyntaxKind::JSX_TEXT,
            &new_jsx_value,
            [],
            [],
        )));

        mutation.replace_node(AnyJsxChild::from(node.clone()), new_jsx_text);

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Wrap the comments with braces" }.to_owned(),
            mutation,
        ))
    }
}
