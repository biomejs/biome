use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssPseudoClass, CssPseudoClassFunctionCompoundSelector,
    CssPseudoClassFunctionCompoundSelectorList, CssPseudoClassFunctionIdentifier,
    CssPseudoClassFunctionNth, CssPseudoClassFunctionRelativeSelectorList,
    CssPseudoClassFunctionSelector, CssPseudoClassFunctionSelectorList,
    CssPseudoClassFunctionValueList, CssPseudoClassIdentifier, CssPseudoClassSelector,
};
use biome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Disallow unknown pseudo-class selectors.
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
    pub NoUnknownPseudoClassSelector {
        version: "1.8.0",
        name: "noUnknownPseudoClassSelector",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("selector-pseudo-class-no-unknown")],
    }
}

declare_node_union! {
    pub AnyWithoutBogusPseudoSelector = CssPseudoClassFunctionCompoundSelector|CssPseudoClassFunctionCompoundSelectorList|CssPseudoClassFunctionIdentifier
                                        |CssPseudoClassFunctionNth|CssPseudoClassFunctionRelativeSelectorList|CssPseudoClassFunctionSelector
                                        |CssPseudoClassFunctionSelectorList|CssPseudoClassFunctionValueList|CssPseudoClassIdentifier
}

impl Rule for NoUnknownPseudoClassSelector {
    type Query = Ast<CssPseudoClassSelector>;
    type State = AnyCssPseudoClass;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let pseudo_class = node.class().ok()?;
        let (pseudo_selector_name, pseudo_selector_range) = match &pseudo_class {
            biome_css_syntax::AnyCssPseudoClass::CssBogusPseudoClass(_) => None,
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(
                selector,
            ) => {
                let name = selector.name().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(
                selector_list,
            ) => {
                let name = selector_list.name().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(ident) => {
                let name = ident.name_token().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionNth(func_nth) => {
                let name = func_nth.name().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(
                selector_list,
            ) => {
                let name = selector_list.name_token().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionSelector(selector) => {
                let name = selector.name().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(
                selector_list,
            ) => {
                let name = selector_list.name().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassFunctionValueList(
                func_value_list,
            ) => {
                let name = func_value_list.name_token().ok()?;
                Some((name.to_string(), name.text_range()))
            }
            biome_css_syntax::AnyCssPseudoClass::CssPseudoClassIdentifier(ident) => {
                let name = ident.name().ok()?;
                Some((name.to_string(), name.range()))
            }
        }?;

        Some(pseudo_class)
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
