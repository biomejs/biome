use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssMediaAndCombinableCondition, AnyCssMediaCondition, AnyCssMediaInParens,
    AnyCssMediaOrCombinableCondition, AnyCssMediaQuery, AnyCssMediaTypeCondition,
    AnyCssMediaTypeQuery, AnyCssQueryFeature, CssMediaAndCondition, CssMediaConditionQuery,
    CssMediaOrCondition, CssMediaQueryList,
};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;

use crate::utils::is_media_feature_name;

declare_lint_rule! {
    /// Disallow unknown media feature names.
    ///
    /// This rule considers media feature names defined in the CSS Specifications, up to and including Editor's Drafts, to be known.
    /// This rule also checks vendor-prefixed media feature names.
    ///
    /// Data sources of known CSS media feature are:
    /// - MDN reference on [CSS media feature](https://developer.mozilla.org/en-US/docs/Web/CSS/@media)
    /// - W3C reference on [Media Queries Level 3](https://www.w3.org/TR/mediaqueries-3/)
    /// - W3C reference on [Media Queries Level 4](https://www.w3.org/TR/mediaqueries-4/)
    /// - W3C reference on [Media Queries Level 5](https://www.w3.org/TR/mediaqueries-5/)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @media screen and (unknown > 320px) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media only screen and (min-width: 320px) and (max-width: 480px) and (unknown: 150dpi) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media (not(unknown < 320px)) and (max-width > 640px) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media (400px <= unknown <= 700px) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @media screen and (width > 320px) {}
    /// ```
    ///
    /// ```css
    /// @media only screen and (min-width: 320px) and (max-width: 480px) and (resolution: 150dpi) {}
    /// ```
    ///
    /// ```css
    /// @media (not(min-width < 320px)) and (max-width > 640px) {}
    /// ```
    ///
    /// ```css
    /// @media (400px <= width <= 700px) {}
    /// ```
    ///
    /// ```css
    /// @media screen and (-webkit-width > 320px) {}
    /// ```
    ///
    pub NoUnknownMediaFeatureName {
        version: "1.8.0",
        name: "noUnknownMediaFeatureName",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("media-feature-name-no-unknown")],
    }
}

impl Rule for NoUnknownMediaFeatureName {
    type Query = Ast<CssMediaQueryList>;
    type State = CssMediaQueryList;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let media_query_list = ctx.query();
        for any_css_media_query in media_query_list {
            match any_css_media_query.ok()? {
                AnyCssMediaQuery::CssMediaConditionQuery(css_media_condition_query) => {
                    if is_invalid_feature_name_included_in_css_media_condition_query(
                        css_media_condition_query,
                    )? {
                        return Some(media_query_list.clone());
                    }
                }
                AnyCssMediaQuery::AnyCssMediaTypeQuery(any_css_media_type_query) => {
                    if is_invalid_feature_name_included_in_css_media_type_query(
                        any_css_media_type_query,
                    )? {
                        return Some(media_query_list.clone());
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Don't use unknown media feature names."
                },
            )
            .note(markup! {
                "Unexpected unknown media feature name."
            })
            .note(markup! {
                "You should use media feature names defined in the CSS Specifications."
            }),
        )
    }
}

fn is_invalid_feature_name_included_in_css_media_condition_query(
    css_media_condition_query: CssMediaConditionQuery,
) -> Option<bool> {
    match css_media_condition_query.condition().ok()? {
        AnyCssMediaCondition::AnyCssMediaInParens(any_css_media_in_parens) => {
            has_invalid_media_feature_name(any_css_media_in_parens)
        }
        AnyCssMediaCondition::CssMediaAndCondition(css_media_and_condition) => {
            is_css_media_and_condition_invalid(css_media_and_condition)
        }
        AnyCssMediaCondition::CssMediaOrCondition(css_media_or_condition) => {
            is_css_media_or_condition_invalid(css_media_or_condition)
        }
        AnyCssMediaCondition::CssMediaNotCondition(css_media_not_condition) => {
            has_invalid_media_feature_name(css_media_not_condition.condition().ok()?)
        }
    }
}

fn is_invalid_feature_name_included_in_css_media_type_query(
    any_css_media_type_query: AnyCssMediaTypeQuery,
) -> Option<bool> {
    match any_css_media_type_query {
        AnyCssMediaTypeQuery::CssMediaTypeQuery(_) => Some(false),
        AnyCssMediaTypeQuery::CssMediaAndTypeQuery(css_media_and_type_query) => {
            match css_media_and_type_query.right().ok()? {
                AnyCssMediaTypeCondition::AnyCssMediaInParens(any_css_media_in_parens) => {
                    has_invalid_media_feature_name(any_css_media_in_parens)
                }
                AnyCssMediaTypeCondition::CssMediaAndCondition(css_media_and_condition) => {
                    is_css_media_and_condition_invalid(css_media_and_condition)
                }
                AnyCssMediaTypeCondition::CssMediaNotCondition(css_media_not_condition) => {
                    has_invalid_media_feature_name(css_media_not_condition.condition().ok()?)
                }
            }
        }
    }
}

fn is_css_media_and_condition_invalid(
    css_media_and_condition: CssMediaAndCondition,
) -> Option<bool> {
    if has_invalid_media_feature_name(css_media_and_condition.left().ok()?)? {
        return Some(true);
    }
    let mut stack = vec![css_media_and_condition.right().ok()?];
    while !stack.is_empty() {
        let element = stack.pop()?;
        match element {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(any_css_media_in_parens) => {
                if has_invalid_media_feature_name(any_css_media_in_parens)? {
                    return Some(true);
                }
            }
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(css_media_and_condition) => {
                if has_invalid_media_feature_name(css_media_and_condition.left().ok()?)? {
                    return Some(true);
                }
                stack.push(css_media_and_condition.right().ok()?);
            }
        }
    }
    Some(false)
}

fn is_css_media_or_condition_invalid(css_media_or_condition: CssMediaOrCondition) -> Option<bool> {
    if has_invalid_media_feature_name(css_media_or_condition.left().ok()?)? {
        return Some(true);
    }
    let mut stack = vec![css_media_or_condition.right().ok()?];
    while !stack.is_empty() {
        let element = stack.pop()?;
        match element {
            AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(any_css_media_in_parens) => {
                if has_invalid_media_feature_name(any_css_media_in_parens)? {
                    return Some(true);
                }
            }
            AnyCssMediaOrCombinableCondition::CssMediaOrCondition(css_media_or_condition) => {
                if has_invalid_media_feature_name(css_media_or_condition.left().ok()?)? {
                    return Some(true);
                }
                stack.push(css_media_or_condition.right().ok()?);
            }
        }
    }
    Some(false)
}

fn has_invalid_media_feature_name(any_css_media_in_parens: AnyCssMediaInParens) -> Option<bool> {
    let mut any_css_media_in_parens_stack = vec![any_css_media_in_parens];
    while !any_css_media_in_parens_stack.is_empty() {
        let any_css_media_in_parens = any_css_media_in_parens_stack.pop()?;
        match any_css_media_in_parens {
            AnyCssMediaInParens::CssMediaFeatureInParens(css_media_feature_in_parens) => {
                let feature_name = get_feature_name(css_media_feature_in_parens.feature().ok()?)?;
                if is_media_feature_name(&feature_name) {
                    continue;
                }
                return Some(true);
            }
            AnyCssMediaInParens::CssMediaConditionInParens(css_media_condition_in_parens) => {
                match css_media_condition_in_parens.condition().ok()? {
                    AnyCssMediaCondition::AnyCssMediaInParens(any_css_media_in_parens) => {
                        any_css_media_in_parens_stack.push(any_css_media_in_parens);
                    }
                    AnyCssMediaCondition::CssMediaAndCondition(css_media_and_condition) => {
                        any_css_media_in_parens_stack.push(css_media_and_condition.left().ok()?);
                        let mut css_media_and_condition_stack =
                            vec![css_media_and_condition.right().ok()?];
                        while !css_media_and_condition_stack.is_empty() {
                            let element = css_media_and_condition_stack.pop()?;
                            match element {
                                AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(
                                    any_css_media_in_parens,
                                ) => {
                                    any_css_media_in_parens_stack.push(any_css_media_in_parens);
                                }
                                AnyCssMediaAndCombinableCondition::CssMediaAndCondition(
                                    css_media_and_condition,
                                ) => {
                                    any_css_media_in_parens_stack
                                        .push(css_media_and_condition.left().ok()?);
                                    css_media_and_condition_stack
                                        .push(css_media_and_condition.right().ok()?);
                                }
                            }
                        }
                    }
                    AnyCssMediaCondition::CssMediaOrCondition(css_media_or_condition) => {
                        any_css_media_in_parens_stack.push(css_media_or_condition.left().ok()?);
                        let mut css_media_or_condition_stack =
                            vec![css_media_or_condition.right().ok()?];
                        while !css_media_or_condition_stack.is_empty() {
                            let element = css_media_or_condition_stack.pop()?;
                            match element {
                                AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(
                                    any_css_media_in_parens,
                                ) => {
                                    any_css_media_in_parens_stack.push(any_css_media_in_parens);
                                }
                                AnyCssMediaOrCombinableCondition::CssMediaOrCondition(
                                    css_media_or_condition,
                                ) => {
                                    any_css_media_in_parens_stack
                                        .push(css_media_or_condition.left().ok()?);
                                    css_media_or_condition_stack
                                        .push(css_media_or_condition.right().ok()?);
                                }
                            }
                        }
                    }
                    AnyCssMediaCondition::CssMediaNotCondition(css_media_not_condition) => {
                        any_css_media_in_parens_stack
                            .push(css_media_not_condition.condition().ok()?);
                    }
                }
            }
        }
    }
    Some(false)
}

fn get_feature_name(any_css_query_feature: AnyCssQueryFeature) -> Option<String> {
    let value_token = match any_css_query_feature {
        AnyCssQueryFeature::CssQueryFeaturePlain(css_query_feature_plain) => {
            css_query_feature_plain.name().ok()?.value_token()
        }
        AnyCssQueryFeature::CssQueryFeatureRange(css_query_feature_range) => {
            css_query_feature_range.left().ok()?.value_token()
        }
        AnyCssQueryFeature::CssQueryFeatureReverseRange(css_query_feature_reversed_range) => {
            css_query_feature_reversed_range.right().ok()?.value_token()
        }
        AnyCssQueryFeature::CssQueryFeatureRangeInterval(css_query_feature_range_interval) => {
            css_query_feature_range_interval.name().ok()?.value_token()
        }
        AnyCssQueryFeature::CssQueryFeatureBoolean(css_query_feature_boolean) => {
            css_query_feature_boolean.name().ok()?.value_token()
        }
    };
    Some(value_token.ok()?.text().to_string().trim().to_string())
}
