use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_graphql_syntax::GraphqlEnumValueDefinition;
use biome_rowan::AstNode;
use biome_rule_options::use_graphql_naming_convention::UseGraphqlNamingConventionOptions;

declare_lint_rule! {
    /// Validates that all enum values are capitalized.
    ///
    /// By convention in GraphQL, enum values are all caps.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum MyEnum {
    ///  value
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// enum MyEnum {
    ///  VALUE
    /// }
    /// ```
    ///
    pub UseGraphqlNamingConvention {
        version: "2.0.0",
        name: "useGraphqlNamingConvention",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::GraphqlSchemaLinter("enum-values-all-caps").inspired()],
    }
}

impl Rule for UseGraphqlNamingConvention {
    type Query = Ast<GraphqlEnumValueDefinition>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseGraphqlNamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if node
            .syntax()
            .text_trimmed()
            .chars()
            .any(|c| c.is_lowercase())
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Enum values should be in all caps."
                },
            )
            .note(markup! {
                    "Change the enum value to be in all caps."
            }),
        )
    }
}
