use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::stmt_ext::CssBlockLike;
use biome_deserialize_macros::Deserializable;
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Disallow css empty blocks.
    ///
    /// This rule disallows empty block.
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
    /// Exclude comments from being treated as content inside of a block.
    ///
    /// ```json
    /// {
    ///     "noCssEmptyBlock": {
    ///         "options": {
    ///           "ignore": ["comments"]
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub NoCssEmptyBlock {
        version: "next",
        name: "noCssEmptyBlock",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-empty-block")],
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Deserializable, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoCssEmptyBlockOptions {
    pub ignore: Vec<String>,
}

impl Rule for NoCssEmptyBlock {
    type Query = Ast<CssBlockLike>;
    type State = CssBlockLike;
    type Signals = Option<Self::State>;
    type Options = Box<NoCssEmptyBlockOptions>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let options = ctx.options();
        let disallow_comments_inside_empty_block = options.ignore.iter().any(|i| i == "comments");

        if disallow_comments_inside_empty_block {
            if node.is_empty() {
                return Some(node.clone());
            }
        } else if node.is_empty() {
            let has_comments_inside_block = node.r_curly_token().ok()?.has_leading_comments()
                || node.l_curly_token().ok()?.has_trailing_comments();

            if has_comments_inside_block {
                return None;
            } else {
                return Some(node.clone());
            }
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
                    "Unexpected empty block is not allowed"
                },
            )
            .note(markup! {
                    "Consider removing the empty block or adding styles inside it."
            }),
        )
    }
}
