use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlDescription;
use biome_rowan::AstNode;
use biome_rule_options::use_description_style::{Style, UseDescriptionStyleOptions};

declare_lint_rule! {
    /// Require all descriptions to follow the same style (either block or inline)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum EnumValue {
    ///   "this is a description"
    ///   DEFAULT
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// enum EnumValue {
    ///   """
    ///   this is a description
    ///   """
    ///   DEFAULT
    /// }
    /// ```
    ///
    pub UseDescriptionStyle {
        version: "next",
        name: "useDescriptionStyle",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("description-style").same()],
        recommended: false,
    }
}

impl Rule for UseDescriptionStyle {
    type Query = Ast<GraphqlDescription>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseDescriptionStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let style = ctx.options().style;

        let value = node.graphql_string_value().ok()?;

        if style == Style::Block && !value.is_block() {
            return Some(());
        }

        if style == Style::Inline && value.is_block() {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        let style = ctx.options().style;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected "{if style == Style::Block { Style::Inline } else { Style::Block }}" description style."
                },
            )
            .note(markup! {
                    "Expected description to have "{style}" style."
            }),
        )
    }
}
