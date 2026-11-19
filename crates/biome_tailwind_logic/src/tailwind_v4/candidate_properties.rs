use std::sync::Arc;

use biome_rowan::AstNode;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwValue, CssGenericComponentValueList,
};

use super::arbitrary_value_match::{starts_with_var_function, value_matches_type};
use super::candidate::{CandidatePlacement, Signature, named_or_number_text};
use super::design_system::TailwindDesignSystem;
use super::tailwind_preset_v4::{
    FUNCTIONAL_UTILITIES, MODIFIER_SIGNATURES, PROPERTY_INDEX, PROPERTY_SET_SIGNATURES,
    SIGNATURE_POOL, VARIABLE_SIGNATURES,
};
use super::tailwind_preset_v4_types::{ArbitraryBranch, CssDataType, Negative, ThemeNamespace};

/// The CSS properties a candidate sets, apart from their values and from the
/// candidate's variants. Two candidates with the same variants and equal
/// properties override each other.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateProperties(PropertySet);

#[derive(Clone, Debug, Eq, PartialEq)]
enum PropertySet {
    /// The properties of a built-in utility, as an index into
    /// `SIGNATURE_POOL`. Utilities share it only when they set the same
    /// properties under the same selectors and at-rules. An arbitrary-property
    /// candidate or a stylesheet utility shares it too when a built-in utility
    /// sets exactly its properties, as `[display:grid]` does with `flex`.
    BuiltIn(u16),
    /// The property names, in ascending order, of an arbitrary-property
    /// candidate or a stylesheet utility that no built-in utility sets
    /// exactly, such as `--gap` in `[--gap:1rem]`. They're set outside any
    /// selector or at-rule.
    TopLevel(Arc<[Box<str>]>),
}

impl PropertySet {
    /// The properties of a candidate that sets exactly `names`, in ascending
    /// order, outside any selector or at-rule.
    fn top_level(names: Arc<[Box<str>]>) -> Self {
        match PROPERTY_SET_SIGNATURES.get(names.join(";").as_str()) {
            Some(signature) => Self::BuiltIn(*signature),
            None => Self::TopLevel(names),
        }
    }
}

impl CandidateProperties {
    /// Returns the properties `candidate` sets, or `None` when they can't be
    /// told:
    ///
    /// - Tailwind generates nothing for the candidate, as with an unknown
    ///   utility or variant.
    /// - More than one utility matches the candidate, such as a built-in
    ///   utility and a stylesheet utility of the same name. Tailwind
    ///   generates a rule from each.
    /// - The candidate is a utility that the stylesheet of `design` declares
    ///   with `@utility`, in a rule that holds more than plain declarations,
    ///   such as a nested rule or `@apply`, or that sets a property twice.
    ///   `design` doesn't record what such a rule compiles to.
    /// - The value has a type hint, as in `text-(length:--size)`. The hint
    ///   picks the properties, and `CandidatePlacement::of` doesn't read it.
    /// - The value is an arbitrary one whose type picks the properties in a
    ///   way `CandidatePlacement::of` can't follow, as in `bg-[10px]`.
    /// - The value is a theme key that the stylesheet of `design` pairs with
    ///   more properties than the utility sets by default, as
    ///   `--text-lg--font-weight` does.
    pub fn of(candidate: &AnyTwFullCandidate, design: &TailwindDesignSystem) -> Option<Self> {
        let CandidatePlacement {
            signature,
            single_rule,
            ..
        } = CandidatePlacement::of(candidate, design)?;
        if !single_rule {
            return None;
        }
        let inner = candidate.as_tw_full_candidate()?.candidate().ok()?;
        match (signature, inner) {
            (Signature::Pool(signature), AnyTwCandidate::TwFunctionalCandidate(functional)) => {
                let value = functional.value().ok()?;
                if has_type_hint(&value) {
                    return None;
                }
                let base = functional.base_token().ok()?;
                let base = base.text_trimmed();
                let is_negative = candidate.as_tw_full_candidate()?.negative_token().is_some();
                let signature = match &value {
                    AnyTwValue::TwCssVariableValue(_) => {
                        variable_signature(base, is_negative).unwrap_or(signature)
                    }
                    AnyTwValue::TwArbitraryValue(arbitrary)
                        if starts_with_var_function(&arbitrary.value()) =>
                    {
                        variable_signature(base, is_negative).unwrap_or(signature)
                    }
                    AnyTwValue::TwArbitraryValue(arbitrary)
                        if !is_arbitrary_type_known(
                            base,
                            is_negative,
                            signature,
                            &arbitrary.value(),
                        ) =>
                    {
                        return None;
                    }
                    _ => signature,
                };
                let mut sets_line_height = false;
                if let Some(key) = named_or_number_text(&value) {
                    let pairs =
                        |namespace, property| design.theme_pairs(namespace, key.text(), property);
                    if sets_only(signature, "font-size") {
                        if pairs(ThemeNamespace::Text, "letter-spacing")
                            || pairs(ThemeNamespace::Text, "font-weight")
                        {
                            return None;
                        }
                        sets_line_height = pairs(ThemeNamespace::Text, "line-height");
                    } else if sets_only(signature, "font-family")
                        && (pairs(ThemeNamespace::Font, "font-feature-settings")
                            || pairs(ThemeNamespace::Font, "font-variation-settings"))
                    {
                        return None;
                    }
                }
                // A font size whose theme key has a line height sets the same
                // properties as a font size with a line-height modifier.
                let signature = if functional.modifier().is_some() || sets_line_height {
                    MODIFIER_SIGNATURES
                        .get(&signature)
                        .copied()
                        .unwrap_or(signature)
                } else {
                    signature
                };
                Some(Self(PropertySet::BuiltIn(signature)))
            }
            (Signature::Pool(signature), _) => Some(Self(PropertySet::BuiltIn(signature))),
            (
                Signature::Property(_) | Signature::CustomProperty,
                AnyTwCandidate::TwArbitraryCandidate(arbitrary),
            ) => {
                let property = arbitrary.property_token().ok()?;
                Some(Self(PropertySet::top_level(Arc::from([Box::from(
                    property.text_trimmed(),
                )]))))
            }
            (Signature::Custom { properties, .. }, _) => {
                Some(Self(PropertySet::top_level(properties?)))
            }
            _ => None,
        }
    }
}

/// Whether the `SIGNATURE_POOL` entry `signature` holds `property` alone.
fn sets_only(signature: u16, property: &str) -> bool {
    PROPERTY_INDEX
        .get(property)
        .is_some_and(|index| SIGNATURE_POOL[usize::from(signature)] == [*index])
}

/// The `SIGNATURE_POOL` index of the utility `base` with a `var()` value,
/// when it isn't the one of its `Fallback` branch.
fn variable_signature(base: &str, is_negative: bool) -> Option<u16> {
    if is_negative {
        VARIABLE_SIGNATURES.get(format!("-{base}").as_str())
    } else {
        VARIABLE_SIGNATURES.get(base)
    }
    .copied()
}

/// Whether `CandidatePlacement::of` read the type of `value`, an arbitrary
/// value of the utility `base`, the way Tailwind does, given the `signature`
/// it resolved to. The type doesn't matter for a utility without typed branches.
///
/// The type is unknown in two cases:
///
/// - The value is a color that holds a math function, as in
///   `border-[hsl(calc(180+30)_50%_50%)]`. It passes for a length too, and
///   Tailwind and the typed branches don't settle that the same way.
/// - The typed branches lead to several placements, as those of `bg` and
///   `mask` do, and the value resolved to one of them. Tailwind reads
///   `bg-[10px]` as a position, while the `Length` branch makes it a size.
fn is_arbitrary_type_known(
    base: &str,
    is_negative: bool,
    signature: u16,
    value: &CssGenericComponentValueList,
) -> bool {
    let Some(entry) = FUNCTIONAL_UTILITIES.get(base) else {
        return true;
    };
    let branches = match (is_negative, entry.negative) {
        (
            true,
            Some(Negative::Distinct {
                arbitrary_branches, ..
            }),
        ) => arbitrary_branches,
        _ => entry.arbitrary_branches,
    };
    let mut typed = branches.iter().filter_map(|branch| match branch {
        ArbitraryBranch::Typed(_, _, typed, _) => Some(*typed),
        ArbitraryBranch::Fallback(..) => None,
    });
    let Some(first) = typed.next() else {
        return true;
    };
    if value_matches_type(value, CssDataType::Color)
        && value_matches_type(value, CssDataType::Length)
    {
        return false;
    }
    typed.all(|other| other == first)
        || branches.iter().any(|branch| {
            matches!(branch, ArbitraryBranch::Fallback(_, fallback, _) if *fallback == signature)
        })
}

/// Whether `value` starts with a type hint, such as `length:` in
/// `text-(length:--size)` and `text-[length:var(--size)]`.
fn has_type_hint(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwCssVariableValue(variable) => variable.type_hint().is_some(),
        AnyTwValue::TwArbitraryValue(arbitrary) => arbitrary
            .value()
            .syntax()
            .first_token()
            .and_then(|token| token.next_token())
            .is_some_and(|token| token.text_trimmed() == ":"),
        _ => false,
    }
}
