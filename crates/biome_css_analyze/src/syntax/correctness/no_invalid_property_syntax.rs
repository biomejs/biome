use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, declare_syntax_rule};
use biome_css_semantic::model::CustomProperty;
use biome_css_syntax::CssPropertyAtRule;
use biome_property_codec::PropertySyntaxResult;
use biome_rowan::AstNode;

declare_syntax_rule! {
    /// Parses the value of `syntax` in CSS custom at-rule `@property`
    ///
    /// ## Examples
    ///
    /// ```js
    /// class A {
    ///   #foo;
    ///   #foo;
    //  }
    /// ```
    pub NoInvalidPropertySyntax {
        version: "next",
        name: "noInvalidPropertySyntax",
        language: "css",
    }
}

impl Rule for NoInvalidPropertySyntax {
    type Query = Semantic<CssPropertyAtRule>;
    type State = CustomProperty;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        model
            .global_custom_variables()
            .at_properties()
            .find_map(|property| {
                if !property.syntax().is_valid() && property.range() == node.range() {
                    Some(property)
                } else {
                    None
                }
            })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.syntax() {
            PropertySyntaxResult::Missing => Some(RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                "The property definition is missing a `syntax` descriptor.",
            )),
            PropertySyntaxResult::Error(diagnostic) => Some(
                RuleDiagnostic::new(rule_category!(), diagnostic.range(), diagnostic.kind())
                    .with_advices(diagnostic.kind()),
            ),

            _ => None,
        }
    }
}
