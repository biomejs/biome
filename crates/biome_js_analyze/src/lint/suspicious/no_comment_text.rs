use crate::JsRuleAction;
use crate::utils::batch::JsBatchMutation;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxChild, JsSyntaxKind, JsSyntaxToken, JsxText, T};
use biome_rowan::{BatchMutationExt, TextRange, TextSize, TriviaPieceKind};
use biome_rule_options::no_comment_text::NoCommentTextOptions;
use std::borrow::Cow;
use std::ops::Range;

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
        sources: &[RuleSource::EslintReact("jsx-no-comment-textnodes").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoCommentText {
    type Query = Ast<JsxText>;
    type State = Range<usize>;
    type Signals = Option<Self::State>;
    type Options = NoCommentTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let jsx_value = node.value_token().ok()?;
        let jsx_value = jsx_value.text();
        let bytes = jsx_value.as_bytes();
        let mut bytes_iter = jsx_value.bytes().enumerate();
        while let Some((index, byte)) = bytes_iter.next() {
            if byte != b'/' {
                continue;
            }
            match bytes_iter.next()? {
                (_, b'/')
                    // Ignore `://` (`https://`, ...)
                    if (index == 0 || bytes.get(index - 1) != Some(&b':')) => {
                        let end = bytes_iter
                            .find(|(_, c)| c == &b'\n')
                            .map_or(bytes.len(), |(index, _)| index);
                        return Some(index..end);
                    }
                (_, b'*') => {
                    let mut end = 0;
                    while let Some((_, byte)) = bytes_iter.next() {
                        if byte != b'*' {
                            continue;
                        }
                        let Some((index, b'/')) = bytes_iter.next() else {
                            continue;
                        };
                        end = index + 1;
                        break;
                    }
                    if end > 0 {
                        return Some(index..end);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let node_range_start = ctx.query().value_token().ok()?.text_range().start();
        Some(RuleDiagnostic::new(
            rule_category!(),
            TextRange::new(
                node_range_start + TextSize::from(range.start as u32),
                node_range_start + TextSize::from(range.end as u32),
            ),
            markup! {
                "Wrap "<Emphasis>"comments"</Emphasis>" inside children within "<Emphasis>"braces"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, range: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let jsx_value = node.value_token().ok()?;
        let jsx_value = jsx_value.text();
        let before_comment = &jsx_value[..range.start];
        let after_comment = &jsx_value[range.end..];
        // Block comments are kept verbatim, while `// text` is turned into `/* text */`.
        let comment: Cow<str> = if jsx_value.as_bytes()[range.start + 1] == b'*' {
            Cow::Borrowed(&jsx_value[range.start..range.end])
        } else {
            let comment_text = jsx_value[range.start + 2..range.end].trim();
            Cow::Owned(format!("/* {comment_text} */"))
        };
        let comment_kind = if comment.contains(['\n', '\r']) {
            TriviaPieceKind::MultiLineComment
        } else {
            TriviaPieceKind::SingleLineComment
        };
        // Emit a real JSX expression container whose `{` token carries the
        // comment as trivia, so that the fixed tree no longer contains a
        // `JsxText` matching this rule. Re-emitting the braces as plain JSX
        // text would make the fix loop of `check --write` wrap its own output
        // in braces over and over again.
        let comment_child = AnyJsxChild::JsxExpressionChild(
            make::jsx_expression_child(
                make::token(T!['{']).with_trailing_trivia([(comment_kind, comment.as_ref())]),
                make::token(T!['}']),
            )
            .build(),
        );
        let mut new_children = Vec::with_capacity(3);
        if !before_comment.is_empty() {
            new_children.push(AnyJsxChild::JsxText(make::jsx_text(
                JsSyntaxToken::new_detached(JsSyntaxKind::JSX_TEXT, before_comment, [], []),
            )));
        }
        new_children.push(comment_child);
        if !after_comment.is_empty() {
            new_children.push(AnyJsxChild::JsxText(make::jsx_text(
                JsSyntaxToken::new_detached(JsSyntaxKind::JSX_TEXT, after_comment, [], []),
            )));
        }
        let mut mutation = ctx.root().begin();
        if !mutation
            .add_jsx_elements_replacing_element(&AnyJsxChild::from(node.clone()), new_children)
        {
            return None;
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the comments with braces" }.to_owned(),
            mutation,
        ))
    }
}
