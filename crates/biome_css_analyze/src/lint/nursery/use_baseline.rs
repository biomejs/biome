use crate::keywords::NAMED_COLORS;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_css_syntax::{
    AnyCssAtRule, CssAtRule, CssFunction, CssGenericProperty, CssPseudoClassIdentifier,
    CssPseudoElementIdentifier, CssQueryFeaturePlain, CssSupportsAtRule, CssSupportsNotCondition,
};
use biome_rowan::{AstNode, TextRange, TokenText, declare_node_union};
use biome_rule_options::use_baseline::{AvailabilityNamed, AvailabilityTarget, UseBaselineOptions};
use biome_string_case::StrLikeExtension;
use std::collections::HashMap;

use crate::baseline_data::{
    BASELINE_AT_RULES, BASELINE_FUNCTIONS, BASELINE_MEDIA_CONDITIONS, BASELINE_PROPERTIES,
    BASELINE_SELECTORS, BaselineTier, BaselineYear, find_baseline, find_property_value_baseline,
};

declare_lint_rule! {
    /// Disallow CSS properties, values, at-rules, functions, and selectors that are not part of the configured Baseline.
    ///
    /// [Baseline](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
    /// tracks the availability of web platform features across core browsers.
    /// This rule helps you avoid features that aren't supported in the browsers you need to target.
    ///
    /// Features are categorized into three tiers:
    /// - **Limited**: Not yet available in all core browsers.
    /// - **Newly available**: Available in all core browsers for less than 30 months.
    /// - **Widely available**: Available in all core browsers for at least 30 months.
    ///
    /// By default, the rule warns on anything that is not Baseline **widely available**.
    ///
    /// Code inside `@supports` blocks is exempt: if you feature-detect a capability before
    /// using it, the rule does not flag it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   backdrop-filter: blur(4px);
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { width: abs(20% - 100px); }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media (inverted-colors: inverted) { a { color: red; } }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// details::details-content { background: red; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { color: red; }
    /// ```
    ///
    /// ```css
    /// /* @supports exempts feature-detected code */
    /// @supports (backdrop-filter: blur(4px)) {
    ///   a { backdrop-filter: blur(4px); }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `available`
    ///
    /// Specifies the minimum Baseline availability tier to accept. Defaults to `"widely"`.
    ///
    /// - `"widely"`: Only accept features that are Baseline widely available (default).
    /// - `"newly"`: Accept features that are at least Baseline newly available.
    /// - A year number (e.g. `2023`): Accept features that became newly available in that year or earlier.
    ///
    /// Default: `"widely"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "available": "newly"
    ///   }
    /// }
    /// ```
    ///
    /// With `"newly"`, a property that is newly (but not yet widely) available doesn't trigger the rule:
    ///
    /// ```css,use_options
    /// a { backdrop-filter: blur(4px); }
    /// ```
    ///
    /// But a limited property still fails:
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { accent-color: red; }
    /// ```
    ///
    /// ### `allowProperties`
    ///
    /// A list of CSS property names to exclude from checking (case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowProperties": ["backdrop-filter"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// a { backdrop-filter: blur(4px); }
    /// ```
    ///
    /// ### `allowAtRules`
    ///
    /// A list of CSS at-rule names to exclude from checking (without `@`, case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowAtRules": ["view-transition"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// @view-transition { navigation: auto; }
    /// ```
    ///
    /// ### `allowFunctions`
    ///
    /// A list of CSS value function names to exclude from checking (case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowFunctions": ["abs"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// a { width: abs(20% - 100px); }
    /// ```
    ///
    /// ### `allowMediaConditions`
    ///
    /// A list of CSS media query condition names to exclude from checking (case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowMediaConditions": ["inverted-colors"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// @media (inverted-colors: inverted) { a { color: red; } }
    /// ```
    ///
    /// ### `allowPropertyValues`
    ///
    /// A list of CSS property value pairs to exclude from checking, in `"property:value"` format
    /// (case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowPropertyValues": ["clip-path:fill-box"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// a { clip-path: fill-box; }
    /// ```
    ///
    /// ### `allowSelectors`
    ///
    /// A list of CSS pseudo-class or pseudo-element names to exclude from checking
    /// (without `:` or `::`, case-insensitive).
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowSelectors": ["has"]
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// h1:has(+ h2) { margin: 0; }
    /// ```
    ///
    pub UseBaseline {
        version: "next",
        name: "useBaseline",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Eslint("css/use-baseline").inspired()],
    }
}

impl Rule for UseBaseline {
    type Query = Ast<AnyBaselineCheckable>;
    type State = UseBaselineState;
    type Signals = Option<Self::State>;
    type Options = UseBaselineOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let node = ctx.query();

        match node {
            AnyBaselineCheckable::CssGenericProperty(prop) => check_property(prop, options),
            AnyBaselineCheckable::CssFunction(func) => check_function(func, options),
            AnyBaselineCheckable::CssPseudoClassIdentifier(pseudo) => check_pseudo(
                pseudo.name().ok()?.value_token().ok()?.token_text_trimmed(),
                pseudo.syntax().text_trimmed_range(),
                "pseudo-class",
                options,
            ),
            AnyBaselineCheckable::CssPseudoElementIdentifier(pseudo) => check_pseudo(
                pseudo.name().ok()?.value_token().ok()?.token_text_trimmed(),
                pseudo.syntax().text_trimmed_range(),
                "pseudo-element",
                options,
            ),
            AnyBaselineCheckable::CssQueryFeaturePlain(feature) => {
                check_media_condition(feature, options)
            }
            AnyBaselineCheckable::CssAtRule(at_rule) => check_at_rule(at_rule, options),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "CSS "<Emphasis>{state.category}</Emphasis>" "<Emphasis>{state.feature_name.message()}</Emphasis>" isn't part of the chosen Baseline."
                },
            )
            .note(markup! {
                "Using a feature that isn't part of the Baseline can lead to unexpected behavior in older browsers."
            })
            .note(markup! {
                "Either remove the feature, or use the "<Emphasis>"@supports"</Emphasis>" at-rule to gate the feature behind a browser-specific support condition."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility">"MDN Baseline"</Hyperlink>" for more information."
            })
            .note(state.feature_name.can_i_use())
        )
    }
}

declare_node_union! {
    pub AnyBaselineCheckable =
        CssGenericProperty
        | CssFunction
        | CssPseudoClassIdentifier
        | CssPseudoElementIdentifier
        | CssQueryFeaturePlain
        | CssAtRule
}

pub struct UseBaselineState {
    /// Human-readable name of the feature that failed the check.
    pub feature_name: FeatureName,
    /// Category label for the diagnostic message.
    pub category: &'static str,
    /// Source range to underline.
    pub range: TextRange,
}

pub enum FeatureName {
    Token(TokenText),
    String(&'static str),
    PropertyValue(TokenText, TokenText),
}

impl From<TokenText> for FeatureName {
    fn from(name: TokenText) -> Self {
        FeatureName::Token(name)
    }
}

impl From<&'static str> for FeatureName {
    fn from(name: &'static str) -> Self {
        FeatureName::String(name)
    }
}

impl From<(TokenText, TokenText)> for FeatureName {
    fn from(pair: (TokenText, TokenText)) -> Self {
        FeatureName::PropertyValue(pair.0, pair.1)
    }
}

impl FeatureName {
    fn message(&self) -> String {
        match self {
            FeatureName::Token(name) => name.text().to_string(),
            FeatureName::PropertyValue(key, value) => format!("{}: {}", key.text(), value.text()),
            FeatureName::String(name) => name.to_string(),
        }
    }

    fn can_i_use(&self) -> MarkupBuf {
        match self {
            FeatureName::Token(name) => {
                let href = format!("https://caniuse.com/?search={}", name.text());
                markup! {
                    "Check "<Hyperlink href={href}>"caniuse.com"</Hyperlink>" for more information about the feature "{name.text()}"."
                }
                .to_owned()
            }
            FeatureName::PropertyValue(key, value) => {
                let value_href = format!("https://caniuse.com/?search={}", value.text());
                let key_href = format!("https://caniuse.com/?search={}", key.text());

                markup! {
                    "Check "<Hyperlink href={value_href}>"caniuse.com"</Hyperlink>" for more information about the value "{value.text()}" for the property "{key.text()}"."
                    "Check "<Hyperlink href={key_href}>"caniuse.com"</Hyperlink>" for more information about the property "{key.text()}"."
                }.to_owned()
            }
            FeatureName::String(name) => {
                let href = format!("https://caniuse.com/?search={}", name);

                markup! {
                    "Check "<Hyperlink href={href}>"caniuse.com"</Hyperlink>" for more information about the feature "{name.to_string()}"."
                }.to_owned()
            }
        }
    }
}

/// Return `true` if the feature's status passes the configured availability
/// requirement.
fn is_acceptable(options: &UseBaselineOptions, tier: BaselineTier, year: BaselineYear) -> bool {
    match &options.available {
        AvailabilityTarget::Named(AvailabilityNamed::Widely) => {
            matches!(tier, BaselineTier::Widely)
        }
        AvailabilityTarget::Named(AvailabilityNamed::Newly) => {
            !matches!(tier, BaselineTier::Limited)
        }
        AvailabilityTarget::Year(cutoff) => match tier {
            BaselineTier::Widely => true,
            BaselineTier::Newly => {
                // Passes only if the year is known and on or before the cutoff.
                matches!(year, BaselineYear::Known(y) if y <= *cutoff)
            }
            BaselineTier::Limited => false,
        },
    }
}

/// Check whether a syntax node is inside a positive (non-negated) `@supports`
/// condition. If so, the property/value is feature-gated and should not be
/// flagged.
fn is_inside_positive_supports(syntax: &biome_css_syntax::CssSyntaxNode) -> bool {
    for anc in syntax.ancestors().skip(1) {
        // If we encounter a `not` condition on the way up, it means we're
        // inside `@supports not (...)` — not suppressed.
        if CssSupportsNotCondition::can_cast(anc.kind()) {
            return false;
        }
        // If we reach the top-level @supports at-rule (via its parent CssAtRule),
        // we are inside a positive @supports.
        if CssSupportsAtRule::can_cast(anc.kind()) {
            return true;
        }
    }
    false
}

fn in_allow_list(name: &TokenText, list: &[String]) -> bool {
    list.iter()
        .any(|item| item.eq_ignore_ascii_case(&name.to_ascii_lowercase_cow()))
}

fn is_allowed_property_value(
    key: &TokenText,
    value: &TokenText,
    list: &HashMap<String, String>,
) -> bool {
    let this_value = list.get(key.to_ascii_lowercase_cow().as_ref());
    this_value.is_some_and(|this_value| {
        value
            .to_ascii_lowercase_cow()
            .eq_ignore_ascii_case(this_value)
    })
}

fn is_named_color(name: &str) -> bool {
    NAMED_COLORS.binary_search(&name).is_ok()
}

fn check_property(
    prop: &CssGenericProperty,
    options: &UseBaselineOptions,
) -> Option<UseBaselineState> {
    use biome_css_syntax::AnyCssValue;

    // If inside a positive @supports, the feature is gated — skip
    if is_inside_positive_supports(prop.syntax()) {
        return None;
    }

    // Check the property name itself
    let name_node = prop.name().ok()?;
    let prop_name = name_node.as_css_identifier()?.value_token().ok()?;
    let prop_text = prop_name.token_text_trimmed();

    // Skip custom properties (--foo)
    if prop_text.text().starts_with("--") {
        return None;
    }

    // Skip if in allow list
    if in_allow_list(&prop_text, options.allow_properties.as_slice()) {
        return None;
    }

    // Check property name baseline
    if let Some(status) = find_baseline(
        BASELINE_PROPERTIES,
        &prop_text.text().to_ascii_lowercase_cow(),
    ) && !is_acceptable(options, status.tier, status.year)
    {
        return Some(UseBaselineState {
            feature_name: prop_text.into(),
            category: "property",
            range: prop_name.text_trimmed_range(),
        });
    }
    // If not in our data, assume it's fine (unknown = not checked)

    // Check property values (identifier keywords only)
    for component in prop.value() {
        if let Some(value) = component.as_any_css_value()
            && let AnyCssValue::CssIdentifier(ident) = value
        {
            let tok = ident.value_token().ok()?;
            let val_text = tok.token_text_trimmed();

            // Skip named colors
            if is_named_color(&val_text.to_ascii_lowercase_cow()) {
                continue;
            }

            // Skip if in property value allow list
            if is_allowed_property_value(&prop_text, &val_text, &options.allow_property_values) {
                continue;
            }

            if let Some(status) = find_property_value_baseline(
                &prop_text.to_ascii_lowercase_cow(),
                &val_text.to_ascii_lowercase_cow(),
            ) && !is_acceptable(options, status.tier, status.year)
            {
                return Some(UseBaselineState {
                    feature_name: (prop_text, val_text).into(),
                    category: "property value",
                    range: tok.text_trimmed_range(),
                });
            }
        }
    }

    None
}

fn check_function(func: &CssFunction, options: &UseBaselineOptions) -> Option<UseBaselineState> {
    let name_node = func.name().ok()?;
    let name_ident = name_node.as_css_identifier()?;
    let name_token = name_ident.value_token().ok()?;
    let name = name_token.token_text_trimmed();

    // Skip custom functions (--foo())
    if name.starts_with("--") {
        return None;
    }

    if in_allow_list(&name, &options.allow_functions) {
        return None;
    }

    if let Some(status) = find_baseline(BASELINE_FUNCTIONS, &name.to_ascii_lowercase_cow())
        && !is_acceptable(options, status.tier, status.year)
    {
        return Some(UseBaselineState {
            feature_name: name.into(),
            category: "function",
            range: name_token.text_trimmed_range(),
        });
    }
    None
}

fn check_pseudo(
    name: TokenText,
    range: TextRange,
    category: &'static str,
    options: &UseBaselineOptions,
) -> Option<UseBaselineState> {
    if in_allow_list(&name, &options.allow_selectors) {
        return None;
    }

    if let Some(status) = find_baseline(BASELINE_SELECTORS, &name.to_ascii_lowercase_cow())
        && !is_acceptable(options, status.tier, status.year)
    {
        return Some(UseBaselineState {
            feature_name: name.into(),
            category,
            range,
        });
    }
    None
}

fn check_media_condition(
    feature: &CssQueryFeaturePlain,
    options: &UseBaselineOptions,
) -> Option<UseBaselineState> {
    let name_node = feature.name().ok()?;
    let tok = name_node.value_token().ok()?;
    let name = tok.token_text_trimmed();

    if in_allow_list(&name, &options.allow_media_conditions) {
        return None;
    }

    if let Some(status) = find_baseline(BASELINE_MEDIA_CONDITIONS, &name.to_ascii_lowercase_cow())
        && !is_acceptable(options, status.tier, status.year)
    {
        return Some(UseBaselineState {
            feature_name: name.into(),
            category: "media condition",
            range: tok.text_trimmed_range(),
        });
    }
    None
}

fn at_rule_name(rule: &AnyCssAtRule) -> Option<&'static str> {
    match rule {
        AnyCssAtRule::CssCharsetAtRule(_) => Some("charset"),
        AnyCssAtRule::CssColorProfileAtRule(_) => Some("color-profile"),
        AnyCssAtRule::CssContainerAtRule(_) => Some("container"),
        AnyCssAtRule::CssCounterStyleAtRule(_) => Some("counter-style"),
        AnyCssAtRule::CssDocumentAtRule(_) => Some("document"),
        AnyCssAtRule::CssFontFaceAtRule(_) => Some("font-face"),
        AnyCssAtRule::CssFontFeatureValuesAtRule(_) => Some("font-feature-values"),
        AnyCssAtRule::CssFontPaletteValuesAtRule(_) => Some("font-palette-values"),
        AnyCssAtRule::CssImportAtRule(_) => Some("import"),
        AnyCssAtRule::CssKeyframesAtRule(_) => Some("keyframes"),
        AnyCssAtRule::CssLayerAtRule(_) => Some("layer"),
        AnyCssAtRule::CssMediaAtRule(_) => Some("media"),
        AnyCssAtRule::CssNamespaceAtRule(_) => Some("namespace"),
        AnyCssAtRule::CssPageAtRule(_) => Some("page"),
        AnyCssAtRule::CssPositionTryAtRule(_) => Some("position-try"),
        AnyCssAtRule::CssPropertyAtRule(_) => Some("property"),
        AnyCssAtRule::CssScopeAtRule(_) => Some("scope"),
        AnyCssAtRule::CssStartingStyleAtRule(_) => Some("starting-style"),
        AnyCssAtRule::CssSupportsAtRule(_) => Some("supports"),
        AnyCssAtRule::CssViewTransitionAtRule(_) => Some("view-transition"),
        // Internal/framework at-rules, not CSS spec
        AnyCssAtRule::CssFunctionAtRule(_)
        | AnyCssAtRule::CssBogusAtRule(_)
        | AnyCssAtRule::CssUnknownBlockAtRule(_)
        | AnyCssAtRule::CssUnknownValueAtRule(_)
        | AnyCssAtRule::CssValueAtRule(_)
        | AnyCssAtRule::TwApplyAtRule(_)
        | AnyCssAtRule::TwConfigAtRule(_)
        | AnyCssAtRule::TwCustomVariantAtRule(_)
        | AnyCssAtRule::TwPluginAtRule(_)
        | AnyCssAtRule::TwReferenceAtRule(_)
        | AnyCssAtRule::TwSlotAtRule(_)
        | AnyCssAtRule::TwSourceAtRule(_)
        | AnyCssAtRule::TwThemeAtRule(_)
        | AnyCssAtRule::TwUtilityAtRule(_)
        | AnyCssAtRule::TwVariantAtRule(_) => None,
    }
}

fn check_at_rule(at_rule: &CssAtRule, options: &UseBaselineOptions) -> Option<UseBaselineState> {
    let inner = at_rule.rule().ok()?;
    let name = at_rule_name(&inner)?;

    if options
        .allow_at_rules
        .iter()
        .any(|item| item.eq_ignore_ascii_case(&name.to_ascii_lowercase_cow()))
    {
        return None;
    }

    if let Some(status) = find_baseline(BASELINE_AT_RULES, name)
        && !is_acceptable(options, status.tier, status.year)
    {
        // Report the at-rule's first token range (the @keyword)
        let range = at_rule.syntax().first_token()?.text_trimmed_range();
        return Some(UseBaselineState {
            feature_name: name.into(),
            category: "at-rule",
            range,
        });
    }
    None
}
