use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_graphql_syntax::GraphqlDirective;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Require specifying the reason argument when using `@deprecated` directive
    ///
    /// This rule checks the parameter of `@deprecated` directive for the use of reason argument,
    /// suggesting user to add it in case the argument is missing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// query {
    ///   member @deprecated
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   member @deprecated(reason: "Why?")
    /// }
    /// ```
    pub UseDeprecatedReason {
        version: "1.9.0",
        name: "useDeprecatedReason",
        language: "graphql",
        sources: &[RuleSource::EslintGraphql("require-deprecation-reason")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
    }
}

impl Rule for UseDeprecatedReason {
    type Query = Ast<GraphqlDirective>;
    type State = GraphqlDirective;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let name = node.name().ok()?;
        if name.to_trimmed_string() != "deprecated" {
            return None;
        }
        // Fire diagnostic if the directive does not have any arguments
        let Some(arguments) = node.arguments() else {
            return Some(node.clone());
        };
        let arguments = arguments.arguments();
        let has_reason = arguments.into_iter().any(|argument| {
            argument
                .name()
                .is_ok_and(|name| name.to_trimmed_string() == "reason")
        });
        if has_reason {
            None
        } else {
            Some(node.clone())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "The directive `@deprecated` should have a `reason` argument."
                },
            )
            .note(markup! {
                    "Add a `reason` argument to the directive."
            }),
        )
    }
}
