use crate::services::semantic::SemanticServices;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, declare_syntax_rule};
use biome_css_semantic::model::CssPropertyAtRule;
use biome_property_codec::PropertySyntaxResult;

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
    type Query = SemanticServices;
    type State = CssPropertyAtRule;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();

        model
            .global_custom_variables()
            .at_properties()
            .filter_map(|property| {
                if !property.syntax().is_valid() {
                    Some(property)
                } else {
                    None
                }
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.syntax() {
            PropertySyntaxResult::Missing => Some(RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                "The property definition is missing a `syntax` descriptor.",
            )),
            PropertySyntaxResult::Error(diagnostic) => Some(RuleDiagnostic::new(
                rule_category!(),
                diagnostic.range(),
                diagnostic.kind(),
            )),

            _ => None,
        }
    }
}
