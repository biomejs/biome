use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, declare_syntax_rule};
use biome_console::markup;
use biome_css_semantic::model::CustomProperty;
use biome_css_syntax::{CssPropertyAtRule, property_syntax::PropertySyntaxResult};
use biome_rowan::AstNode;

declare_syntax_rule! {
    /// Reports invalid `syntax` descriptors in CSS `@property` rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @property --size {
    ///   syntax: "<length> |";
    ///   inherits: false;
    /// }
    /// ```
    ///
    /// A `syntax` descriptor is required.
    ///
    /// ```css,expect_diagnostic
    /// @property --size {
    ///   inherits: false;
    ///   initial-value: 0px;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @property --size {
    ///   syntax: "<length> | auto";
    ///   inherits: false;
    ///   initial-value: 0px;
    /// }
    /// ```
    pub NoInvalidPropertySyntax {
        version: "2.5.7",
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
        let model = ctx.model();
        let node = ctx.query();

        model
            .global_custom_variables()
            .at_property_by_range(node.range())
            .filter(|property| !property.syntax().is_valid())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.syntax() {
            PropertySyntaxResult::Missing => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range(),
                    "The property definition is missing a `syntax` descriptor.",
                )
                .note(markup! {
                    "Without a valid "<Emphasis>"syntax"</Emphasis>" descriptor, the browser does not register the custom property."
                })
                .note(markup! {
                    "Add a "<Emphasis>"syntax"</Emphasis>" descriptor that describes the accepted values."
                }),
            ),
            PropertySyntaxResult::Error(diagnostic) => Some(
                RuleDiagnostic::new(rule_category!(), diagnostic.range(), diagnostic.kind())
                    .with_advices(diagnostic.kind()),
            ),

            _ => None,
        }
    }
}
