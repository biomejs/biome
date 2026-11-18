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

use crate::utils::{
    AnyPropertyStatement, is_known_properties, nested_property_name, vendor_prefixed,
};

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
    /// SCSS lets you write properties that share a prefix inside a block named after that prefix,
    /// such as `border: { radius: 4px; }`. These nested properties are checked by the full name
    /// Sass emits for them, so `radius` here is checked as `border-radius`. If the name of a
    /// surrounding block contains interpolation, the properties inside that block are ignored.
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
    type State = NoUnknownPropertyState;
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
        let nested_name = match node
            .syntax()
            .grand_parent()
            .and_then(AnyPropertyStatement::cast)
            .and_then(|statement| statement.group())
        {
            Some(group) => Some(nested_property_name(
                property_name_token.text_trimmed(),
                group,
            )?),
            None => None,
        };
        let property_name_lower = nested_name
            .as_deref()
            .unwrap_or(property_name_token.text_trimmed())
            .to_ascii_lowercase_cow();

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
            return Some(NoUnknownPropertyState {
                range: property_name.range(),
                nested_name,
            });
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "Unknown property is not allowed."
            },
        );
        if let Some(nested_name) = &state.nested_name {
            diagnostic = diagnostic.note(markup! {
                "Sass emits this nested property as "<Emphasis>{nested_name}</Emphasis>"."
            });
        }
        Some(
            diagnostic
            .note(markup! {
                "Browsers ignore properties they do not know, so the declaration has no effect. An unknown property is often a typo."
            })
           .note(markup! {
                "Use a known CSS property instead."
            })
            .note(markup! {
                "See "<Hyperlink href="https://stylelint.io/user-guide/rules/property-no-unknown/">"CSS Specifications and browser specific properties"</Hyperlink>" for more details."
            })
        )
    }
}

pub struct NoUnknownPropertyState {
    range: TextRange,
    /// The name Sass emits for a property nested in an SCSS group, such as
    /// `border-radius` for `radius` in `border: { radius: 4px; }`, or `None`
    /// outside a group.
    nested_name: Option<String>,
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
        AnyCssDeclarationName::ScssInterpolatedIdentifier(_)
        | AnyCssDeclarationName::CssMetavariable(_) => None,
    }
}
