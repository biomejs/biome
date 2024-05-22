use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_deserialize_macros::Deserializable;
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Disallow CSS empty blocks.
    ///
    /// By default, it will allow empty blocks with comments inside.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .b {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media print { a {} }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: red;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   /* foo */
    /// }
    /// ```
    ///
    /// ```css
    /// @media print { a { color: pink; } }
    /// ```
    ///
    /// ## Options
    ///
    /// If false, exclude comments from being treated as content inside of a block.
    ///
    /// ```json
    /// {
    ///     "noEmptyBlock": {
    ///         "options": {
    ///           "allowComments": false
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub NoEmptyBlock {
        version: "next",
        name: "noEmptyBlock",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-empty-block")],
    }
}

#[derive(Debug, Clone, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoEmptyBlockOptions {
    pub allow_comments: bool,
}

impl Default for NoEmptyBlockOptions {
    fn default() -> Self {
        Self {
            allow_comments: true,
        }
    }
}

impl Rule for NoEmptyBlock {
    type Query = Ast<CssBlockLike>;
    type State = CssBlockLike;
    type Signals = Option<Self::State>;
    type Options = NoEmptyBlockOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let options = ctx.options();
        let allow_comments_inside_empty_block = options.allow_comments;
        if allow_comments_inside_empty_block {
            let has_comments_inside_block = node.r_curly_token().ok()?.has_leading_comments()
                || node.l_curly_token().ok()?.has_trailing_comments();

            if !node.is_empty() || has_comments_inside_block {
                return None;
            } else {
                return Some(node.clone());
            }
        } else if node.is_empty() {
            return Some(node.clone());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Empty blocks aren't allowed."
                },
            )
            .note(markup! {
                    "Consider removing the empty block or adding styles inside it."
            }),
        )
    }
}
