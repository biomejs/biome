use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssDashedIdentifier, AnyCssDeclarationName, CssContainerAtRule,
    CssFunctionAtRule, CssGenericProperty, CssLayerAtRule, CssMediaAtRule, CssScopeAtRule,
    CssStartingStyleAtRule, CssSupportsAtRule, ScssAtRootAtRule, ScssEachAtRule, ScssForAtRule,
    ScssIfAtRule, ScssIncludeAtRule, ScssMixinAtRule, ScssWhileAtRule, TwApplyAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unknown_property::NoUnknownPropertyOptions;
use biome_string_case::StrLikeExtension;

use crate::utils::{is_known_properties, vendor_prefixed};

declare_lint_rule! {
    /// Disallow unrecognized CSS properties.
    ///
    /// The known-property list includes standard and browser-specific properties from
    /// [known-css-properties](https://github.com/known-css/known-css-properties#source).
    /// Custom properties such as `--custom-property` and vendor-prefixed properties such as
    /// `-moz-align-self` or `-webkit-align-self` are allowed.
    ///
    /// ## SCSS limitations
    ///
    /// Property names containing SCSS interpolation are ignored because the emitted name cannot be
    /// determined statically.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   colr: blue;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   my-property: 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: green;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   fill: black;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   -moz-align-self: center;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignore`
    ///
    /// Lists additional property names to allow, without regard to letter case. Defaults to an
    /// empty list.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": [
    ///       "custom-property"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a {
    ///   custom-property: black;
    /// }
    /// ```
    ///
    pub NoUnknownProperty {
        version: "1.8.0",
        name: "noUnknownProperty",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("property-no-unknown").same(), RuleSource::EslintCss("no-invalid-properties").inspired()],
    }
}

impl Rule for NoUnknownProperty {
    type Query = Ast<CssGenericProperty>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUnknownPropertyOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let is_in_descriptor_at_rule = node.syntax().ancestors().skip(1).any(|ancestor| {
            AnyCssAtRule::can_cast(ancestor.kind())
                && !AnyDeclarationSupportingAtRule::can_cast(ancestor.kind())
        });

        if is_in_descriptor_at_rule {
            return None;
        }

        let property_name = node.name().ok()?;
        let property_name_token = declaration_name_value_token(&property_name)?;
        let property_name_lower = property_name_token.text_trimmed().to_ascii_lowercase_cow();

        let in_function_at_rule = node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|ancestor| CssFunctionAtRule::can_cast(ancestor.kind()));

        if in_function_at_rule && property_name_lower == "result" {
            return None;
        }

        if !property_name_lower.starts_with("--")
            // Ignore `composes` property.
            // See https://github.com/css-modules/css-modules/blob/master/docs/composition.md for more details.
            && property_name_lower != "composes"
            && !is_known_properties(&property_name_lower)
            && !vendor_prefixed(&property_name_lower)
            && !should_ignore(&property_name_lower, ctx.options())
        {
            return Some(property_name.range());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unknown property is not allowed."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://stylelint.io/user-guide/rules/property-no-unknown/">"CSS Specifications and browser specific properties"</Hyperlink>" for more details."
            })
           .note(markup! {
                "To resolve this issue, replace the unknown property with a valid CSS property."
            })
        )
    }
}

declare_node_union! {
    pub AnyDeclarationSupportingAtRule = TwApplyAtRule | CssContainerAtRule
                    | CssLayerAtRule
                    | CssMediaAtRule
                    | CssScopeAtRule
                    | CssStartingStyleAtRule
                    | CssSupportsAtRule
                    | CssFunctionAtRule
                    | ScssAtRootAtRule
                    | ScssEachAtRule
                    | ScssForAtRule
                    | ScssIfAtRule
                    | ScssIncludeAtRule
                    | ScssMixinAtRule
                    | ScssWhileAtRule
}

fn should_ignore(name: &str, options: &NoUnknownPropertyOptions) -> bool {
    for ignore_pattern in &options.ignore {
        if name.eq_ignore_ascii_case(ignore_pattern) {
            return true;
        }
    }
    false
}

fn declaration_name_value_token(
    name: &AnyCssDeclarationName,
) -> Option<biome_css_syntax::CssSyntaxToken> {
    match name {
        AnyCssDeclarationName::AnyCssDashedIdentifier(
            AnyCssDashedIdentifier::CssDashedIdentifier(name),
        ) => name.value_token().ok(),
        AnyCssDeclarationName::AnyCssDashedIdentifier(
            AnyCssDashedIdentifier::ScssInterpolatedDashedIdentifier(_),
        ) => None,
        AnyCssDeclarationName::CssIdentifier(name) => name.value_token().ok(),
        AnyCssDeclarationName::TwValueThemeReference(name) => {
            name.reference().ok()?.value_token().ok()
        }
        AnyCssDeclarationName::ScssInterpolatedIdentifier(_) => None,
    }
}
