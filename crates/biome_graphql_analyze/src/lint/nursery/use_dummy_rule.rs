use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule};
use biome_graphql_syntax::GraphqlRoot;

declare_rule! {
    /// Dummy rule
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "title": "New title",
    ///   "title": "Second title"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "title": "New title",
    ///   "secondTitle": "Second title"
    /// }
    /// ```
    pub UseDummyRule {
        version: "next",
        name: "useDummyRule",
        language: "graphql",
    }
}

impl Rule for UseDummyRule {
    type Query = Ast<GraphqlRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        None
    }
}
