use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleBlock;
use biome_deserialize_macros::Deserializable;
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {}
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
    type Query = Ast<CssDeclarationOrRuleBlock>;
    type State = CssDeclarationOrRuleBlock;
    type Signals = Option<Self::State>;
    type Options = Box<NoCssEmptyBlockOptions>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let options = ctx.options();
        let should_alert_comments = options.ignore.iter().any(|i| i == "comments");

        if should_alert_comments {
            if node.items().into_iter().next().is_none() {
                return Some(node.clone());
            }
        } else if node.items().into_iter().next().is_none()
            && !node.r_curly_token().ok()?.has_leading_comments()
            && !node.l_curly_token().ok()?.has_trailing_comments()
        {
            return Some(node.clone());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
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
                    "This note will give you more information."
            }),
        )
    }
}
