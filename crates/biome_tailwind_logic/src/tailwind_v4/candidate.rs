//! Resolves a Tailwind v4 candidate to the rule Tailwind generates for it:
//! the properties it sets, as a [Signature], and the variants it applies
//! under.

use std::cmp::Ordering;
use std::sync::Arc;

use biome_rowan::{AstNode, AstSeparatedList, TokenText};
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, CssGenericComponentValueList,
    TwFullCandidate, TwFunctionalCandidate, TwNumberValue,
};

use super::arbitrary_value_match::value_matches_type;
use super::design_system::{
    BareValueType, CustomFunctionalUtility, TailwindDesignSystem, ValueArgument,
    declared_properties,
};
use super::tailwind_preset_v4::{
    FUNCTIONAL_UTILITIES, KEYWORD_POOL, PROPERTY_INDEX, SIGNATURE_POOL, STATIC_UTILITIES,
};
use super::tailwind_preset_v4_types::{
    ArbitraryBranch, ModifierKind, NamedBranch, NamedValueType, Negative, ThemeNamespace,
    UtilityEntry,
};
use super::variants::{VariantKey, variant_keys_from_candidate};

/// Where Tailwind places the rule it generates for a candidate.
#[derive(Clone, Debug)]
pub(crate) struct CandidatePlacement {
    pub(crate) signature: Signature,
    /// The number of declarations the rule holds.
    pub(crate) count: u8,
    pub(crate) variants: Vec<VariantKey>,
    /// Whether one utility alone matches the candidate. Tailwind generates a
    /// rule from every utility that matches, such as a built-in utility and a
    /// stylesheet utility of the same name, and the placement only describes
    /// the first.
    pub(crate) single_rule: bool,
}

/// The set of CSS properties a candidate's declarations set, encoded as
/// ascending indices into Tailwind's canonical property order — the
/// order in which properties first appear in Tailwind's generated
/// stylesheet. The config analyzer captures these lists verbatim from
/// the `propertySort` that Tailwind's `compileAstNodes` computes per
/// candidate.
///
/// Tailwind orders the rules it generates by signature first, then by
/// declaration count, then by candidate text.
#[derive(Clone, Debug)]
pub(crate) enum Signature {
    /// The index of a generated `SIGNATURE_POOL` entry.
    Pool(u16),
    /// A utility declared with `@utility` in the user's stylesheet.
    Custom {
        order: Box<[u16]>,
        /// The names of the properties the utility's rule sets, when it
        /// holds only plain declarations. See [declared_properties].
        properties: Option<Arc<[Box<str>]>>,
    },
    /// A single property, for arbitrary-property candidates
    /// (`[display:block]`).
    Property(u16),
    /// An arbitrary-property candidate Tailwind can't order: a custom
    /// property (`[--my-var:1]`) or an unknown property name. Tailwind
    /// emits an empty `propertySort`, which sorts after every real
    /// property; `as_slice` returns `&[]` so the prefix comparison in
    /// `Ord` places it last. Two empty signatures compare equal, so
    /// Tailwind orders them by candidate text (`[--my-var:1]` before
    /// `[--my-var:2]`).
    CustomProperty,
}

impl Signature {
    fn as_slice(&self) -> &[u16] {
        match self {
            Self::Pool(index) => SIGNATURE_POOL[usize::from(*index)],
            Self::Custom { order, .. } => order,
            Self::Property(index) => std::slice::from_ref(index),
            Self::CustomProperty => &[],
        }
    }
}

/// Equality goes through `as_slice` so that, per the `Ord` contract, a
/// `Pool` and a `Property` holding the same indices are equal.
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Signature {}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares the index lists: the first differing index after the shared
/// prefix decides, and when one list is a prefix of the other, the longer
/// list sorts first. `size-4` (height + width) shares its height index
/// with `h-4` (height alone) and wins by length, while `w-4` (width
/// alone) sorts after both on the later width index.
///
/// This walks the shared prefix rather than delegating to slice's own
/// lexicographic `Ord` because the tie-break is inverted: slice order
/// treats a shorter prefix as lesser, but a candidate whose signature is
/// a prefix of another's must sort first here. The walk itself is cheap
/// regardless — `SIGNATURE_POOL` entries top out at 8 indices.
impl Ord for Signature {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self.as_slice(), other.as_slice());
        let mut i = 0;
        while i < a.len() && i < b.len() && a[i] == b[i] {
            i += 1;
        }
        match (a.get(i), b.get(i)) {
            (Some(a), Some(b)) => a.cmp(b),
            // The longer list wins a shared prefix.
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

impl CandidatePlacement {
    /// Returns the placement of `candidate` like [Self::from_candidate],
    /// including the functional utilities of the user's stylesheet, or
    /// `None` when Tailwind generates nothing for it. Tailwind compiles a
    /// candidate with every utility it matches and places it by the first
    /// rule it generates.
    pub(crate) fn of(
        candidate: &AnyTwFullCandidate,
        design: &TailwindDesignSystem,
    ) -> Option<Self> {
        let key = Self::from_candidate(candidate, design);
        if !design.has_functional_utilities() {
            return key;
        }
        let AnyTwFullCandidate::TwFullCandidate(node) = candidate else {
            return key;
        };
        let Ok(inner) = node.candidate() else {
            return key;
        };
        let base = match &inner {
            AnyTwCandidate::TwFunctionalCandidate(functional) => functional.base_token(),
            AnyTwCandidate::TwStaticCandidate(candidate) => candidate.base_token(),
            _ => return key,
        };
        let Ok(base) = base else {
            return key;
        };
        let base = base.text_trimmed();
        if !design.is_functional_root(base) {
            return key;
        }
        let key = match &inner {
            AnyTwCandidate::TwFunctionalCandidate(functional) => or_first(
                key,
                Self::from_custom_functional(node, functional, base, design),
            ),
            _ => key,
        };
        // A stylesheet root such as `bg-red` makes the parser split
        // `bg-red-500` at that root, but Tailwind also tries its built-in
        // roots, such as `bg`.
        let has_built_in_prefix = base
            .match_indices('-')
            .any(|(end, _)| FUNCTIONAL_UTILITIES.contains_key(&base[..end]));
        if !has_built_in_prefix {
            return key;
        }
        let parse = parse_tailwind(&candidate.syntax().text_trimmed().to_string());
        if parse.has_errors() {
            return key;
        }
        match parse.tree().candidates().iter().flatten().next() {
            Some(candidate) => or_first(key, Self::from_candidate(&candidate, design)),
            None => key,
        }
    }

    /// Places a candidate with the stylesheet's functional utilities whose
    /// root is `base`, such as `tab` for `@utility tab-*`, or `-tab` when the
    /// candidate is negative.
    fn from_custom_functional(
        node: &TwFullCandidate,
        functional: &TwFunctionalCandidate,
        base: &str,
        design: &TailwindDesignSystem,
    ) -> Option<Self> {
        if node.legacy_important_token().is_some() && node.excl_token().is_some() {
            return None;
        }
        let negative_root;
        let root = if node.negative_token().is_some() {
            negative_root = format!("-{base}");
            &negative_root
        } else {
            base
        };
        let value = functional.value().ok()?;
        let modifier = functional.modifier();
        let compiled: Vec<_> = design
            .functional_utility(root)
            .iter()
            .filter_map(|utility| {
                compile_custom_functional(utility, &value, modifier.as_ref(), design)
            })
            .collect();
        let single_rule = compiled.len() == 1;
        let (signature, count) = compiled
            .into_iter()
            .min_by(|(s1, c1), (s2, c2)| s1.cmp(s2).then_with(|| c2.cmp(c1)))?;
        let variants = variant_keys_from_candidate(node, design)?;
        Some(Self {
            signature,
            count,
            variants,
            single_rule,
        })
    }

    /// Places a candidate with the built-in utilities and the static
    /// utilities of the user's stylesheet, or returns `None` for a shape we
    /// can't place.
    fn from_candidate(
        candidate: &AnyTwFullCandidate,
        design: &TailwindDesignSystem,
    ) -> Option<Self> {
        let AnyTwFullCandidate::TwFullCandidate(node) = candidate else {
            return None;
        };

        // An unrecognized variant leaves the candidate unplaced.
        let variants = variant_keys_from_candidate(node, design)?;

        // The legacy leading `!` and the trailing `!` are each fine on
        // their own; Tailwind rejects a candidate spelling both.
        if node.legacy_important_token().is_some() && node.excl_token().is_some() {
            return None;
        }

        let is_negative = node.negative_token().is_some();

        let inner = node.candidate().ok()?;
        let built_in = built_in_placement(&inner, is_negative, design);

        // A utility from the user's stylesheet. The grammar splits a dashed
        // name like `btn-primary` into a base and a value, so the whole
        // candidate text is matched. Built-in static utilities take
        // precedence.
        let custom = if design.has_utilities() && !is_negative {
            let name = inner.syntax().text_trimmed().to_string();
            design
                .utility(&name)
                .map(|utility| (utility, STATIC_UTILITIES.contains_key(name.as_str())))
        } else {
            None
        };
        if let Some((utility, false)) = custom {
            return Some(Self {
                signature: Signature::Custom {
                    order: utility.signature.clone(),
                    properties: utility.properties.clone(),
                },
                count: utility.count,
                variants,
                single_rule: built_in.is_none(),
            });
        }

        let (signature, count) = built_in?;
        Some(Self {
            signature,
            count,
            variants,
            single_rule: custom.is_none(),
        })
    }
}

/// Places a candidate with the built-in utilities, or returns `None` for a
/// shape we can't place.
fn built_in_placement(
    inner: &AnyTwCandidate,
    is_negative: bool,
    design: &TailwindDesignSystem,
) -> Option<(Signature, u8)> {
    match inner {
        AnyTwCandidate::TwArbitraryCandidate(a) => {
            let property_token = a.property_token().ok()?;
            // An arbitrary property accepts a numeric opacity-style
            // modifier (`[color:red]/50`, `[padding:1px]/2`,
            // `[--my-var:1]/(--x)`) but not a bare word or percentage
            // (`[color:red]/foo`), regardless of the property.
            if let Some(modifier) = a.modifier()
                && !modifier_accepted(ModifierKind::Opacity, &modifier, design)
            {
                return None;
            }
            // Tailwind places every arbitrary-property candidate. A
            // property in the known order sorts by its index; a custom
            // property (`[--my-var:1]`) or an unknown property name has
            // an empty Tailwind order and sorts after every real
            // property, ordered among themselves by candidate text.
            let signature = PROPERTY_INDEX
                .get(property_token.text_trimmed())
                .map_or(Signature::CustomProperty, |&property_idx| {
                    Signature::Property(property_idx)
                });
            Some((signature, 1))
        }
        AnyTwCandidate::TwBogusCandidate(_) => None,

        AnyTwCandidate::TwStaticCandidate(s) => {
            let name = s.base_token().ok()?;
            let name = name.text_trimmed();
            if let Some(modifier) = s.modifier() {
                // Static registrations take no modifier; the few
                // valid bare-with-modifier forms (`@container/main`
                // names the container, `shadow/50` sets the shadow
                // color opacity) compile through a functional root's
                // bare placement, picked by modifier shape.
                if is_negative {
                    None
                } else {
                    FUNCTIONAL_UTILITIES.get(name).and_then(|entry| {
                        let placement =
                            if modifier_accepted(ModifierKind::Opacity, &modifier, design) {
                                entry.bare_opacity
                            } else {
                                entry.bare_name
                            };
                        placement.map(|(sig, count)| (Signature::Pool(sig), count))
                    })
                }
            } else if let Some(entry) = STATIC_UTILITIES
                .get(name)
                // Tailwind registers negative statics individually
                // (`-m-px` exists, `-flex` does not).
                .filter(|entry| !is_negative || entry.has_negative)
            {
                Some((Signature::Pool(entry.sig), entry.count))
            } else if !is_negative
                && let Some(entry) = FUNCTIONAL_UTILITIES.get(name)
                && let Some((sig, count)) = entry.bare
            {
                // Functional utilities with a default compile bare
                // (`border`, `ring`, `shadow`) without a static
                // registration.
                Some((Signature::Pool(sig), count))
            } else {
                None
            }
        }

        AnyTwCandidate::TwFunctionalCandidate(f) => {
            let base = f.base_token().ok()?;

            let value = f.value().ok()?;

            // Tailwind resolves a candidate's full name as a static
            // utility before trying functional roots: `w-full`,
            // `m-auto`, and `justify-center` are static
            // registrations even though the grammar splits them
            // into base and value. Statics take no modifier, so a
            // modifier skips the lookup.
            if f.modifier().is_none()
                && let Some(text) = named_text(&value)
                && let Some(entry) = joined_static_entry(base.text_trimmed(), text.text())
                && (!is_negative || entry.has_negative)
            {
                Some((Signature::Pool(entry.sig), entry.count))
            } else {
                let entry = FUNCTIONAL_UTILITIES.get(base.text_trimmed())?;

                let (named_branches, arbitrary_branches) = if is_negative {
                    match entry.negative {
                        None => return None,
                        Some(Negative::SameBranches) => {
                            (entry.named_branches, entry.arbitrary_branches)
                        }
                        Some(Negative::Distinct {
                            named_branches,
                            arbitrary_branches,
                        }) => (named_branches, arbitrary_branches),
                    }
                } else {
                    (entry.named_branches, entry.arbitrary_branches)
                };

                let modifier = f.modifier();
                let resolved = if let AnyTwValue::TwArbitraryValue(arb) = &value {
                    resolve_arbitrary_branch(
                        arbitrary_branches,
                        &arb.value(),
                        modifier.as_ref(),
                        design,
                    )
                } else if let AnyTwValue::TwCssVariableValue(_) = &value {
                    resolve_css_variable_branch(arbitrary_branches, modifier.as_ref(), design)
                } else {
                    resolve_named_branch(named_branches, &value, modifier.as_ref(), design)
                };
                resolved.map(|(sig, count)| (Signature::Pool(sig), count))
            }
        }
    }
}

/// Of two placements of the same candidate, the one Tailwind places first.
/// When both exist, Tailwind generates a rule from each.
fn or_first(
    placement: Option<CandidatePlacement>,
    other: Option<CandidatePlacement>,
) -> Option<CandidatePlacement> {
    let (Some(first), Some(second)) = (&placement, &other) else {
        return placement.or(other);
    };
    let mut placement = if second
        .signature
        .cmp(&first.signature)
        .then_with(|| first.count.cmp(&second.count))
        .is_lt()
    {
        other
    } else {
        placement
    }?;
    placement.single_rule = false;
    Some(placement)
}

/// Compiles a candidate's value and modifier with a functional utility the
/// way Tailwind does, and returns the signature and count of the declarations
/// it keeps, or `None` when Tailwind generates nothing.
///
/// A declaration is dropped when one of its `--value(…)` or `--modifier(…)`
/// functions accepts neither the value nor the modifier. The candidate is
/// invalid when no `--value(…)` accepts its value, when it has a modifier that
/// no `--modifier(…)` of the utility accepts, or when its modifier is accepted
/// neither by a `--modifier(…)` nor as the denominator of a ratio.
fn compile_custom_functional(
    utility: &CustomFunctionalUtility,
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
    design: &TailwindDesignSystem,
) -> Option<(Signature, u8)> {
    let modifier = match modifier {
        None => None,
        Some(AnyTwModifier::TwModifier(modifier)) => Some(modifier.value().ok()?),
        Some(AnyTwModifier::TwBogusModifier(_)) => return None,
    };
    let mut used_value = false;
    let mut resolved_value = false;
    let mut used_modifier = false;
    let mut resolved_modifier = false;
    let mut resolved_ratio = false;
    // The kept declarations, and whether a `--value(…)` resolved in them other
    // than as a ratio.
    let mut kept = Vec::with_capacity(utility.declarations.len());
    'declarations: for declaration in &utility.declarations {
        let mut resolved_non_ratio = false;
        for function in &declaration.functions {
            if function.is_modifier {
                used_modifier = true;
                let resolved = modifier.as_ref().is_some_and(|modifier| {
                    resolve_value_function(&function.arguments, modifier, None, design).is_some()
                });
                if !resolved {
                    continue 'declarations;
                }
                resolved_modifier = true;
            } else {
                used_value = true;
                match resolve_value_function(&function.arguments, value, modifier.as_ref(), design)
                {
                    None => continue 'declarations,
                    Some(is_ratio) => {
                        resolved_value = true;
                        resolved_ratio |= is_ratio;
                        resolved_non_ratio |= !is_ratio;
                    }
                }
            }
        }
        kept.push((declaration, resolved_non_ratio));
    }
    if !used_value
        || !resolved_value
        || modifier.is_some() && used_modifier && !resolved_modifier
        || resolved_ratio && resolved_modifier
        || modifier.is_some() && !resolved_ratio && !resolved_modifier
    {
        return None;
    }
    // A ratio replaces the declarations that took the value some other way.
    if resolved_ratio {
        kept.retain(|(_, resolved_non_ratio)| !resolved_non_ratio);
    }
    let count = u8::try_from(kept.len()).unwrap_or(u8::MAX);
    let properties = if utility.has_only_declarations {
        declared_properties(kept.iter().map(|(declaration, _)| &*declaration.name))
    } else {
        None
    };
    let mut order: Vec<u16> = kept
        .into_iter()
        .filter_map(|(declaration, _)| declaration.property)
        .collect();
    order.sort_unstable();
    order.dedup();
    Some((
        Signature::Custom {
            order: order.into_boxed_slice(),
            properties,
        },
        count,
    ))
}

/// Matches `value` against the arguments of a `--value(…)` or
/// `--modifier(…)` function and returns whether it matched as a ratio, or
/// `None` when no argument accepts it. `denominator` is the candidate's
/// modifier, which makes the value a fraction such as `16/9`.
fn resolve_value_function(
    arguments: &[ValueArgument],
    value: &AnyTwValue,
    denominator: Option<&AnyTwValue>,
    design: &TailwindDesignSystem,
) -> Option<bool> {
    arguments.iter().find_map(|argument| {
        let matched = match argument {
            ValueArgument::Literal(literal) => {
                named_or_number_text(value).is_some_and(|text| text.text() == literal.as_ref())
            }
            ValueArgument::Theme(reference) => named_or_number_text(value)
                .is_some_and(|text| design.has_theme_value(reference, text.text())),
            ValueArgument::Bare(BareValueType::Integer) => is_bare_integer(value),
            ValueArgument::Bare(BareValueType::Number) => matches!(value,
                AnyTwValue::TwNumberValue(number) if bare_number(number).is_some_and(is_quarter_step)),
            ValueArgument::Bare(BareValueType::Percentage) => matches!(value,
                AnyTwValue::TwPercentageValue(percentage) if percentage
                    .value_token()
                    .is_ok_and(|token| is_bare_integer_text(token.text_trimmed()))),
            ValueArgument::Bare(BareValueType::Ratio) => {
                if is_integral_number(value) && denominator.is_some_and(is_integral_number) {
                    return Some(true);
                }
                false
            }
            ValueArgument::AnyArbitrary => matches!(
                value,
                AnyTwValue::TwArbitraryValue(_) | AnyTwValue::TwCssVariableValue(_)
            ),
            ValueArgument::Arbitrary(data_type) => match (value, data_type) {
                (AnyTwValue::TwArbitraryValue(arbitrary), Some(data_type)) => {
                    value_matches_type(&arbitrary.value(), *data_type)
                }
                _ => false,
            },
        };
        matched.then_some(false)
    })
}

/// Whether `value` is a number with an integer value, however it's written,
/// so `7`, `07`, and `7.0`. Tailwind reads the parts of a ratio this way.
fn is_integral_number(value: &AnyTwValue) -> bool {
    let AnyTwValue::TwNumberValue(number) = value else {
        return false;
    };
    number
        .value_token()
        .ok()
        .and_then(|token| token.text_trimmed().parse::<f64>().ok())
        .is_some_and(|n| n.fract() == 0.0)
}

/// Whether `text` is a non-negative integer written the way Tailwind
/// accepts it, so `7` but not `07`.
fn is_bare_integer_text(text: &str) -> bool {
    !text.is_empty()
        && text.bytes().all(|byte| byte.is_ascii_digit())
        && (text == "0" || !text.starts_with('0'))
}

/// Does this utility's branch list declare a `NamedBranch::Typed(Ratio)` slot?
/// Used as the gate for fraction-aliased modifiers (`w-1/2` ↔ `w-[50%]`) —
/// mirrors Tailwind's `supportsFractions` flag
fn entry_has_ratio_branch(branches: &[NamedBranch]) -> bool {
    branches
        .iter()
        .any(|b| matches!(b, NamedBranch::Typed(NamedValueType::Ratio, ..)))
}

/// `n/m` Tailwind fraction shorthand: the value and the modifier are bare
/// integers, and the utility actually accepts fractions.
fn is_fraction_modifier(
    value: &AnyTwValue,
    modifier: &AnyTwModifier,
    branches: &[NamedBranch],
) -> bool {
    let AnyTwModifier::TwModifier(m) = modifier else {
        return false;
    };
    is_bare_integer(value)
        && m.value()
            .is_ok_and(|denominator| is_bare_integer(&denominator))
        && entry_has_ratio_branch(branches)
}

/// The value of a bare number written the way Tailwind accepts it: the text
/// JavaScript's `String(Number(text))` gives back, so `7` and `0.5` but not
/// `07`, `1.50`, or `.5`.
fn bare_number(number: &TwNumberValue) -> Option<f64> {
    let token = number.value_token().ok()?;
    let text = token.text_trimmed();
    let (whole, fraction) = match text.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (text, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let canonical = digits(whole)
        && (whole == "0" || !whole.starts_with('0'))
        && fraction.is_none_or(|fraction| digits(fraction) && !fraction.ends_with('0'));
    if canonical { text.parse().ok() } else { None }
}

fn is_bare_integer(value: &AnyTwValue) -> bool {
    matches!(value, AnyTwValue::TwNumberValue(number)
        if bare_number(number).is_some_and(|n| n.fract() == 0.0))
}

/// Whether `n` is a multiple of 0.25, the step Tailwind's spacing scale,
/// opacity, and line-height take.
fn is_quarter_step(n: f64) -> bool {
    (n * 4.0).fract() == 0.0
}

fn named_text(value: &AnyTwValue) -> Option<TokenText> {
    let AnyTwValue::TwNamedValue(named) = value else {
        return None;
    };
    named
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

pub(super) fn named_or_number_text(value: &AnyTwValue) -> Option<TokenText> {
    match value {
        AnyTwValue::TwNamedValue(named) => named
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        AnyTwValue::TwNumberValue(number) => number
            .value_token()
            .ok()
            .map(|token| token.token_text_trimmed()),
        _ => None,
    }
}

fn named_value_type_matches(
    value_type: NamedValueType,
    value: &AnyTwValue,
    has_fraction_modifier: bool,
) -> bool {
    match (value_type, value, has_fraction_modifier) {
        (NamedValueType::Integer, AnyTwValue::TwNumberValue(_), false) => is_bare_integer(value),
        (NamedValueType::Multiplier, AnyTwValue::TwNumberValue(number), false) => {
            bare_number(number).is_some_and(is_quarter_step)
        }
        (NamedValueType::Percentage, AnyTwValue::TwPercentageValue(_), false)
        | (NamedValueType::Ratio, AnyTwValue::TwNumberValue(_), true) => true,
        _ => false,
    }
}

/// Look up `base-value` in `STATIC_UTILITIES` without allocating,
/// reassembling the name in a stack buffer sized for the longest static
/// utility name (`font-stretch-ultra-condensed`, 28 bytes).
fn joined_static_entry(base: &str, value: &str) -> Option<&'static UtilityEntry> {
    const LONGEST_STATIC_NAME: usize = 32;
    let len = base.len() + 1 + value.len();
    if len > LONGEST_STATIC_NAME {
        return None;
    }
    let mut buf = [0u8; LONGEST_STATIC_NAME];
    buf[..base.len()].copy_from_slice(base.as_bytes());
    buf[base.len()] = b'-';
    buf[base.len() + 1..len].copy_from_slice(value.as_bytes());
    // Joining two `str`s with an ASCII byte always forms valid UTF-8.
    let name = str::from_utf8(&buf[..len]).ok()?;
    STATIC_UTILITIES.get(name)
}

/// Walk a basename's named branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Branch order in
/// the preset already reflects the resolution precedence we want
/// (Keyword → Theme → Typed).
fn resolve_named_branch(
    branches: &[NamedBranch],
    value: &AnyTwValue,
    modifier: Option<&AnyTwModifier>,
    design: &TailwindDesignSystem,
) -> Option<(u16, u8)> {
    let has_fraction_modifier = match modifier {
        None => false,
        Some(m) if is_fraction_modifier(value, m, branches) => true,
        Some(AnyTwModifier::TwModifier(_)) => false,
        Some(AnyTwModifier::TwBogusModifier(_)) => return None,
    };

    for &branch in branches {
        let (modifier_kind, property_idx, property_count) = match branch {
            // Theme-namespace lookup (`text-lg` ↔ `--text-lg`). Both Named
            // and Number kinds query it — users can register numeric
            // theme keys like `--spacing-12`.
            NamedBranch::Theme(namespace, m, p, c) => {
                if has_fraction_modifier {
                    continue;
                }
                let Some(text) = named_or_number_text(value) else {
                    continue;
                };
                if !design.has_theme_key(namespace, text.text()) {
                    continue;
                }
                (m, p, c)
            }
            // Hard-coded keyword pool (`origin-top`, `accent-current`).
            NamedBranch::Keyword(pool_idx, m, p, c) => {
                let Some(text) = named_text(value) else {
                    continue;
                };
                if !KEYWORD_POOL[usize::from(pool_idx)].contains(&text.text()) {
                    continue;
                }
                (m, p, c)
            }
            NamedBranch::Typed(value_type, m, p, c) => {
                if !named_value_type_matches(value_type, value, has_fraction_modifier) {
                    continue;
                }
                (m, p, c)
            }
        };
        // A non-fraction modifier is valid only on the branch that accepts
        // one: a color branch takes an opacity modifier, a font-size branch
        // a line-height modifier. A modifier on any other match — or an
        // ill-formed one — makes the candidate invalid (`w-1/foo`, `p-4/2`).
        if !has_fraction_modifier
            && let Some(modifier) = modifier
            && !modifier_accepted(modifier_kind, modifier, design)
        {
            return None;
        }
        return Some((property_idx, property_count));
    }
    None
}

/// Whether `modifier` is a well-formed `/modifier` for a branch that accepts
/// `kind`. Opacity takes a number, arbitrary value, or CSS variable;
/// line-height takes those plus a `--leading-*` theme keyword. A percentage
/// or a bare word is never a valid modifier.
fn modifier_accepted(
    kind: ModifierKind,
    modifier: &AnyTwModifier,
    design: &TailwindDesignSystem,
) -> bool {
    let AnyTwModifier::TwModifier(modifier) = modifier else {
        return false;
    };
    let Ok(value) = modifier.value() else {
        return false;
    };
    match kind {
        ModifierKind::None => false,
        ModifierKind::Opacity => is_numeric_modifier(&value),
        ModifierKind::LineHeight => {
            is_numeric_modifier(&value)
                || matches!(&value, AnyTwValue::TwNamedValue(_))
                    && named_text(&value).is_some_and(|text| {
                        design.has_theme_key(ThemeNamespace::Leading, text.text())
                    })
        }
    }
}

/// A number (`/50`), an arbitrary value (`/[0.5]`), or a CSS variable
/// (`/(--x)`) — the modifier value kinds every modifier-accepting branch
/// allows. A bare number must be a multiple of 0.25 (`/2.5`, not `/1.3`).
fn is_numeric_modifier(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwNumberValue(number) => bare_number(number).is_some_and(is_quarter_step),
        AnyTwValue::TwArbitraryValue(_) | AnyTwValue::TwCssVariableValue(_) => true,
        _ => false,
    }
}

/// Walk a basename's arbitrary branch list and return the first matching
/// branch's `(property_idx, property_count)` placement. Typed branches
/// precede the type-blind fallback in generated preset order. A modifier is
/// gated against the matched branch just as in [resolve_named_branch]: an
/// arbitrary value on a branch that takes no modifier (`p-[4px]/2`,
/// `w-[1px]/foo`) is not a real candidate.
fn resolve_arbitrary_branch(
    branches: &[ArbitraryBranch],
    list: &CssGenericComponentValueList,
    modifier: Option<&AnyTwModifier>,
    design: &TailwindDesignSystem,
) -> Option<(u16, u8)> {
    for &branch in branches {
        let (modifier_kind, property_idx, property_count) = match branch {
            ArbitraryBranch::Typed(value_type, m, p, c) => {
                if !value_matches_type(list, value_type) {
                    continue;
                }
                (m, p, c)
            }
            ArbitraryBranch::Fallback(m, p, c) => (m, p, c),
        };
        if let Some(modifier) = modifier
            && !modifier_accepted(modifier_kind, modifier, design)
        {
            return None;
        }
        return Some((property_idx, property_count));
    }
    None
}

/// Resolve a CSS variable value (`w-(--x)`), which Tailwind treats as the
/// arbitrary value `w-[var(--x)]`. A `var()` value never matches a typed
/// branch (see [value_matches_type]), so only the fallback branch places it.
fn resolve_css_variable_branch(
    branches: &[ArbitraryBranch],
    modifier: Option<&AnyTwModifier>,
    design: &TailwindDesignSystem,
) -> Option<(u16, u8)> {
    let (modifier_kind, property_idx, property_count) =
        branches.iter().find_map(|&branch| match branch {
            ArbitraryBranch::Fallback(m, p, c) => Some((m, p, c)),
            ArbitraryBranch::Typed(..) => None,
        })?;
    if let Some(modifier) = modifier
        && !modifier_accepted(modifier_kind, modifier, design)
    {
        return None;
    }
    Some((property_idx, property_count))
}

#[cfg(test)]
mod tests {
    use super::super::tailwind_preset_v4_types::CssDataType;
    use super::*;

    #[test]
    fn arbitrary_candidate_takes_signature_from_property_index() {
        let display_idx = *PROPERTY_INDEX.get("display").unwrap();
        let parsed = parse_tailwind("[display:block]");
        let candidate = parsed.tree().candidates().iter().next().unwrap().unwrap();
        let placement =
            CandidatePlacement::of(&candidate, &TailwindDesignSystem::default()).unwrap();
        assert_eq!(placement.signature, Signature::Property(display_idx));
        assert_eq!(placement.count, 1);
    }

    fn functional_parts(input: &str) -> (AnyTwValue, Option<AnyTwModifier>) {
        let parsed = parse_tailwind(input);
        let full = parsed.tree().candidates().iter().next().unwrap().unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        (functional.value().unwrap(), functional.modifier())
    }

    // region: branch resolution

    #[test]
    fn resolve_named_branch_returns_first_matching_branch() {
        // Two NamedBranch::Typed(Multiplier) branches with different property_idx;
        // first one to match wins.
        let (value, modifier) = functional_parts("p-5");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Multiplier, ModifierKind::None, 10, 1),
            NamedBranch::Typed(NamedValueType::Multiplier, ModifierKind::None, 20, 1),
        ];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_classifies_value_internally() {
        let (value, modifier) = functional_parts("p-5");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Multiplier,
            ModifierKind::None,
            10,
            1,
        )];

        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_typed_when_matcher_returns_false_then_falls_back() {
        let branches = &[
            ArbitraryBranch::Typed(CssDataType::Number, ModifierKind::None, 10, 1),
            ArbitraryBranch::Fallback(ModifierKind::None, 20, 1),
        ];
        let full = parse_tailwind("p-[10px]")
            .tree()
            .candidates()
            .iter()
            .next()
            .unwrap()
            .unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        assert_eq!(
            resolve_arbitrary_branch(
                branches,
                &arbitrary.value(),
                None,
                &TailwindDesignSystem::default()
            ),
            Some((20, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_skips_named_branches_by_construction() {
        let full = parse_tailwind("p-[10px]")
            .tree()
            .candidates()
            .iter()
            .next()
            .unwrap()
            .unwrap();
        let full = full.as_tw_full_candidate().unwrap();
        let candidate = full.candidate().unwrap();
        let AnyTwCandidate::TwFunctionalCandidate(functional) = candidate else {
            panic!("expected functional candidate")
        };
        let AnyTwValue::TwArbitraryValue(arbitrary) = functional.value().unwrap() else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Fallback(ModifierKind::None, 20, 1)];
        assert_eq!(
            resolve_arbitrary_branch(
                branches,
                &arbitrary.value(),
                None,
                &TailwindDesignSystem::default()
            ),
            Some((20, 1))
        );
    }

    #[test]
    fn resolve_arbitrary_branch_rejects_a_modifier_on_a_branch_that_takes_none() {
        // `p-[4px]/2`: the padding arbitrary branch accepts no modifier, so
        // the `/2` makes the candidate invalid.
        let (value, modifier) = functional_parts("p-[4px]/2");
        let AnyTwValue::TwArbitraryValue(arb) = &value else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Fallback(ModifierKind::None, 20, 1)];
        assert_eq!(
            resolve_arbitrary_branch(
                branches,
                &arb.value(),
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            None
        );
    }

    #[test]
    fn resolve_arbitrary_branch_accepts_a_numeric_modifier_on_an_opacity_branch() {
        // `bg-[#fff]/50`: an arbitrary color takes a numeric opacity modifier.
        let (value, modifier) = functional_parts("bg-[#fff]/50");
        let AnyTwValue::TwArbitraryValue(arb) = &value else {
            panic!("expected arbitrary value")
        };
        let branches = &[ArbitraryBranch::Typed(
            CssDataType::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_arbitrary_branch(
                branches,
                &arb.value(),
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_returns_none_when_kind_does_not_match_value_type() {
        // A named value like "abc" never satisfies NamedBranch::Typed(Multiplier)
        // because dispatch is by parser node kind, not text scanning.
        let (value, modifier) = functional_parts("p-abc");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Multiplier,
            ModifierKind::None,
            1,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            None
        );
    }

    #[test]
    fn resolve_named_branch_ratio_matches_ratio_typed_branch() {
        let (value, modifier) = functional_parts("w-1/2");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Ratio,
            ModifierKind::None,
            7,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((7, 1))
        );
    }

    #[test]
    fn resolve_named_branch_percentage_only_matches_percentage_typed_branch() {
        let (value, modifier) = functional_parts("from-25%");
        let branches = &[
            NamedBranch::Typed(NamedValueType::Multiplier, ModifierKind::None, 1, 1),
            NamedBranch::Typed(NamedValueType::Percentage, ModifierKind::None, 2, 1),
        ];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((2, 1))
        );
    }

    #[test]
    fn resolve_named_branch_takes_only_numbers_tailwind_accepts() {
        let spacing = &[NamedBranch::Typed(
            NamedValueType::Multiplier,
            ModifierKind::None,
            10,
            1,
        )];
        let index = &[NamedBranch::Typed(
            NamedValueType::Integer,
            ModifierKind::None,
            20,
            1,
        )];
        for (class, branches, expected) in [
            ("p-0.5", spacing, Some((10, 1))),
            ("p-1.25", spacing, Some((10, 1))),
            ("p-1.3", spacing, None),
            ("p-01", spacing, None),
            ("p-1.50", spacing, None),
            ("z-10", index, Some((20, 1))),
            ("z-1.5", index, None),
        ] {
            let (value, modifier) = functional_parts(class);
            assert_eq!(
                resolve_named_branch(
                    branches,
                    &value,
                    modifier.as_ref(),
                    &TailwindDesignSystem::default()
                ),
                expected,
                "{class}"
            );
        }
    }

    #[test]
    fn resolve_named_branch_accepts_valid_opacity_modifier() {
        // A numeric opacity modifier is valid on a color branch.
        let (value, modifier) = functional_parts("bg-red-500/50");
        let branches = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_modifier_on_a_branch_that_takes_none() {
        // `w-1/foo`: the number branch takes no modifier, so the `/foo`
        // modifier makes the candidate invalid rather than resolving like `w-1`.
        let (value, modifier) = functional_parts("w-1/foo");
        let branches = &[NamedBranch::Typed(
            NamedValueType::Multiplier,
            ModifierKind::None,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            None
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_bare_word_opacity_modifier() {
        // `bg-red-500/foo`: a color branch takes a numeric opacity modifier,
        // not a bare word.
        let (value, modifier) = functional_parts("bg-red-500/foo");
        let branches = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                branches,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            None
        );
    }

    #[test]
    fn resolve_named_branch_accepts_a_leading_keyword_for_line_height() {
        // `text-lg/loose` is a valid line-height modifier (`loose` is a
        // `--leading-*` keyword).
        let (value, modifier) = functional_parts("text-lg/loose");
        let line_height = &[NamedBranch::Theme(
            ThemeNamespace::Text,
            ModifierKind::LineHeight,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                line_height,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            Some((10, 1))
        );
    }

    #[test]
    fn resolve_named_branch_rejects_a_leading_keyword_on_an_opacity_branch() {
        // `bg-red-500/loose`: a leading keyword is not a valid opacity modifier.
        let (value, modifier) = functional_parts("bg-red-500/loose");
        let opacity = &[NamedBranch::Theme(
            ThemeNamespace::Color,
            ModifierKind::Opacity,
            10,
            1,
        )];
        assert_eq!(
            resolve_named_branch(
                opacity,
                &value,
                modifier.as_ref(),
                &TailwindDesignSystem::default()
            ),
            None
        );
    }

    // endregion: branch resolution
}
