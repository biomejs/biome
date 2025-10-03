use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlDescription;
use biome_rowan::AstNode;
use biome_rule_options::use_consistent_graphql_descriptions::{
    Style, UseConsistentGraphqlDescriptionsOptions,
};

declare_lint_rule! {
    /// Require all descriptions to follow the same style (either block or inline)
    ///
    /// ## Examples
    ///
    /// ### style: `block`
    ///
    /// #### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum EnumValue {
    ///   "this is a description"
    ///   DEFAULT
    /// }
    /// ```
    ///
    /// #### Valid
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
    /// ### style: `inline`
    ///
    /// #### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum EnumValue {
    ///   """
    ///   this is a description
    ///   """
    ///   DEFAULT
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```graphql
    /// enum EnumValue {
    ///   "this is a description"
    ///   DEFAULT
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### style
    ///
    /// Use the `style` option to specify the required description style:
    /// - `"block"` (default): Requires triple-quoted block descriptions (`"""..."""`)
    /// - `"inline"`: Requires single-quoted inline descriptions (`"..."`)
    ///
    pub UseConsistentGraphqlDescriptions {
        version: "next",
        name: "useConsistentGraphqlDescriptions",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("description-style").same()],
        recommended: false,
    }
}

impl Rule for UseConsistentGraphqlDescriptions {
    type Query = Ast<GraphqlDescription>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentGraphqlDescriptionsOptions;

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
