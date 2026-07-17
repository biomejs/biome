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
    /// Require all descriptions to follow the same style (either block or inline) to  maintain consistency and improve readability across the schema.
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
    /// ## Options
    ///
    /// ### `style`
    ///
    /// This option will specify the description style.
    /// - `"block"`: Requires triple-quoted block descriptions (`"""..."""`)
    /// - `"inline"`: Requires single-quoted inline descriptions (`"..."`)
    ///
    /// Default `"block"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "inline"
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic,use_options
    /// enum EnumValue {
    ///   """
    ///   this is a description
    ///   """
    ///   DEFAULT
    /// }
    /// ```
    ///
    pub UseConsistentGraphqlDescriptions {
        version: "2.3.6",
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
        let style = ctx.options().style.unwrap_or_default();

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
        let style = ctx.options().style.unwrap_or_default();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected "{if style == Style::Block { Style::Inline } else { Style::Block }}" description style."
                },
            )
            .note(markup! {
                    "To stay consistent within the project, write the description "{style}" style."
            }),
        )
    }
}
