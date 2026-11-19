use biome_package::PackageJson;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, TwCandidateList,
    TwFullCandidate, TwRoot,
};
use biome_unicode_table::{Dispatch, lookup_byte};
use smallvec::SmallVec;

/// A utility that Tailwind CSS v4 only keeps for compatibility with v3.
#[derive(Clone, Debug)]
pub struct LegacyUtility {
    pub candidate: TwFullCandidate,
    replacement: Replacement,
    since: TailwindVersion,
}

/// The part of a legacy utility to rewrite, and what to rewrite it to.
#[derive(Clone, Copy, Debug)]
enum Replacement {
    /// Replaces the base, as in `flex-grow-0` to `grow-0`.
    Base(&'static str),
    /// Replaces the value, as in `bg-left-top` to `bg-top-left`.
    Value(&'static str),
    /// Replaces the base and the value, as in `order-none` to `order-0`.
    Utility(&'static str),
    /// Rewrites `max-w-screen-<name>` to `max-w-(--breakpoint-<name>)`.
    BreakpointVariable,
}

/// The first `tailwindcss` version that has a replacement.
#[derive(Clone, Copy, Debug)]
enum TailwindVersion {
    V3_0,
    V4_0,
    V4_1,
    V4_2,
}

impl TailwindVersion {
    /// Returns the range of versions released before this one.
    const fn older_versions(self) -> &'static str {
        match self {
            Self::V3_0 => "<3.0.0",
            Self::V4_0 => "<4.0.0",
            Self::V4_1 => "<4.1.0",
            Self::V4_2 => "<4.2.0",
        }
    }
}

impl LegacyUtility {
    /// Returns `false` when `manifest` allows a `tailwindcss` version that
    /// doesn't have the replacement yet.
    ///
    /// A range such as `^4.1.0` can resolve to 4.1.x, so it hides replacements
    /// added in 4.2. Versions that aren't semver ranges, such as `latest`,
    /// don't hide anything.
    pub fn is_replacement_available(&self, manifest: &PackageJson) -> bool {
        !manifest.matches_dependency("tailwindcss", self.since.older_versions())
    }

    /// The range of the utility within the class string, including the `-` of
    /// a negative value but not its variants or important flag.
    pub fn range(&self) -> Option<TextRange> {
        let candidate = self.candidate.candidate().ok()?;
        let start = self.candidate.negative_token().map_or_else(
            || candidate.range().start(),
            |negative| negative.text_trimmed_range().start(),
        );
        Some(TextRange::new(start, candidate.range().end()))
    }

    /// Returns the utility as written and the utility that replaces it.
    pub fn texts(&self, root: &TwRoot) -> Option<(String, String)> {
        let range = self.range()?;
        let original = root.syntax().text_with_trivia().slice(range).to_string();
        let replacement = self.apply(&original, range.start())?;
        Some((original, replacement))
    }

    /// Returns the class string of `root` with this utility replaced.
    pub fn fixed_class_string(&self, root: &TwRoot) -> Option<String> {
        self.apply(
            &root.syntax().text_with_trivia().to_string(),
            TextSize::from(0),
        )
    }

    /// Applies the replacement to `text`, which starts at `offset` in the class string.
    fn apply(&self, text: &str, offset: TextSize) -> Option<String> {
        let (range, replacement) = self.edit()?;
        let range = range.checked_sub(offset)?;
        let mut fixed = String::with_capacity(text.len() + replacement.len());
        fixed.push_str(text.get(..usize::from(range.start()))?);
        fixed.push_str(&replacement);
        fixed.push_str(text.get(usize::from(range.end())..)?);
        Some(fixed)
    }

    /// Returns the range within the class string to rewrite and its new text.
    fn edit(&self) -> Option<(TextRange, String)> {
        let (base, value) = match self.candidate.candidate().ok()? {
            AnyTwCandidate::TwFunctionalCandidate(candidate) => {
                (candidate.base_token().ok()?, Some(candidate.value().ok()?))
            }
            AnyTwCandidate::TwStaticCandidate(candidate) => (candidate.base_token().ok()?, None),
            _ => return None,
        };
        let base_range = base.text_trimmed_range();
        Some(match self.replacement {
            Replacement::Base(replacement) => (base_range, replacement.to_string()),
            Replacement::Value(replacement) => (value?.range(), replacement.to_string()),
            Replacement::Utility(replacement) => {
                (base_range.cover(value?.range()), replacement.to_string())
            }
            Replacement::BreakpointVariable => {
                let value = value?;
                (
                    base_range.cover(value.range()),
                    format!("max-w-(--breakpoint-{})", value.syntax().text_trimmed()),
                )
            }
        })
    }
}

/// Returns the legacy utilities in `candidates`.
pub fn legacy_utilities(candidates: &TwCandidateList) -> SmallVec<[LegacyUtility; 2]> {
    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| {
            let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
                return None;
            };
            let (replacement, since) = classify(&candidate)?;
            Some(LegacyUtility {
                candidate,
                replacement,
                since,
            })
        })
        .collect()
}

/// Returns how to replace `candidate` if it's a legacy utility, and the first
/// `tailwindcss` version that has the replacement.
fn classify(candidate: &TwFullCandidate) -> Option<(Replacement, TailwindVersion)> {
    let is_negative = candidate.negative_token().is_some();
    match candidate.candidate().ok()? {
        AnyTwCandidate::TwStaticCandidate(candidate) => {
            if is_negative || candidate.modifier().is_some() {
                return None;
            }
            match candidate.base_token().ok()?.text_trimmed() {
                "flex-grow" => Some((Replacement::Base("grow"), TailwindVersion::V3_0)),
                "flex-shrink" => Some((Replacement::Base("shrink"), TailwindVersion::V3_0)),
                _ => None,
            }
        }
        AnyTwCandidate::TwFunctionalCandidate(candidate) => {
            let base = candidate.base_token().ok()?;
            let value = candidate.value().ok()?;
            let modifier = candidate.modifier();
            match base.text_trimmed() {
                "start" => is_inset_value(&value, modifier.as_ref(), is_negative)
                    .then_some((Replacement::Base("inset-s"), TailwindVersion::V4_2)),
                "end" => is_inset_value(&value, modifier.as_ref(), is_negative)
                    .then_some((Replacement::Base("inset-e"), TailwindVersion::V4_2)),
                _ if is_negative || modifier.is_some() => None,
                "flex-grow" => is_flex_factor(&value)
                    .then_some((Replacement::Base("grow"), TailwindVersion::V3_0)),
                "flex-shrink" => is_flex_factor(&value)
                    .then_some((Replacement::Base("shrink"), TailwindVersion::V3_0)),
                base => {
                    let AnyTwValue::TwNamedValue(value) = value else {
                        return None;
                    };
                    let value = value.value_token().ok()?;
                    let value = value.text_trimmed();
                    match base {
                        "bg" => {
                            position_replacement(value).map(|value| (value, TailwindVersion::V4_1))
                        }
                        "bg-gradient-to" => GRADIENT_DIRECTIONS
                            .binary_search(&value)
                            .is_ok()
                            .then_some((Replacement::Base("bg-linear-to"), TailwindVersion::V4_0)),
                        "break" => (value == "words").then_some((
                            Replacement::Utility("wrap-break-word"),
                            TailwindVersion::V4_1,
                        )),
                        "decoration" => matches!(value, "slice" | "clone").then_some((
                            Replacement::Base("box-decoration"),
                            TailwindVersion::V3_0,
                        )),
                        // Custom breakpoints aren't known here, so only the
                        // default ones are reported.
                        "max-w-screen" => DEFAULT_BREAKPOINTS
                            .contains(&value)
                            .then_some((Replacement::BreakpointVariable, TailwindVersion::V4_0)),
                        "object" => {
                            position_replacement(value).map(|value| (value, TailwindVersion::V4_1))
                        }
                        "order" => (value == "none")
                            .then_some((Replacement::Utility("order-0"), TailwindVersion::V4_0)),
                        "overflow" => (value == "ellipsis").then_some((
                            Replacement::Utility("text-ellipsis"),
                            TailwindVersion::V3_0,
                        )),
                        _ => None,
                    }
                }
            }
        }
        _ => None,
    }
}

/// Returns whether Tailwind CSS generates styles for `start-*` or `end-*`
/// with this value, without reading the project's theme.
///
/// Values such as `start-date` are left alone because they're likely custom
/// classes rather than Tailwind CSS utilities.
fn is_inset_value(value: &AnyTwValue, modifier: Option<&AnyTwModifier>, is_negative: bool) -> bool {
    match value {
        AnyTwValue::TwArbitraryValue(_) | AnyTwValue::TwCssVariableValue(_) => modifier.is_none(),
        AnyTwValue::TwNumberValue(number) => {
            let Ok(number) = number.value_token() else {
                return false;
            };
            match modifier {
                None => is_spacing_multiplier(number.text_trimmed()),
                // A fraction such as `start-1/2`.
                Some(AnyTwModifier::TwModifier(modifier)) => {
                    is_positive_integer(number.text_trimmed())
                        && modifier.value().is_ok_and(|denominator| {
                            denominator.as_tw_number_value().is_some_and(|denominator| {
                                denominator
                                    .value_token()
                                    .is_ok_and(|token| is_positive_integer(token.text_trimmed()))
                            })
                        })
                }
                Some(_) => false,
            }
        }
        AnyTwValue::TwNamedValue(named) => {
            modifier.is_none()
                && named
                    .value_token()
                    .is_ok_and(|token| match token.text_trimmed() {
                        "full" | "px" => true,
                        "auto" => !is_negative,
                        _ => false,
                    })
        }
        _ => false,
    }
}

/// Returns whether Tailwind CSS generates styles for `flex-grow-*` or
/// `flex-shrink-*` with this value.
fn is_flex_factor(value: &AnyTwValue) -> bool {
    match value {
        AnyTwValue::TwArbitraryValue(_) | AnyTwValue::TwCssVariableValue(_) => true,
        AnyTwValue::TwNumberValue(number) => number
            .value_token()
            .is_ok_and(|token| is_positive_integer(token.text_trimmed())),
        _ => false,
    }
}

/// Returns whether `text` is a whole number written without leading zeros, such as `0` or `12`.
fn is_positive_integer(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| matches!(lookup_byte(byte), Dispatch::ZER | Dispatch::DIG))
        && (text == "0" || !text.starts_with('0'))
}

/// Returns whether `text` is a multiple of 0.25 written without extra zeros,
/// such as `4` or `0.5`.
fn is_spacing_multiplier(text: &str) -> bool {
    match text.split_once('.') {
        Some((integer, fraction)) => {
            is_positive_integer(integer) && matches!(fraction, "25" | "5" | "75")
        }
        None => is_positive_integer(text),
    }
}

fn position_replacement(value: &str) -> Option<Replacement> {
    POSITIONS
        .iter()
        .find_map(|(legacy, replacement)| (*legacy == value).then_some(*replacement))
        .map(Replacement::Value)
}

/// Directions of `bg-gradient-to-*`, sorted for binary search.
const GRADIENT_DIRECTIONS: &[&str] = &["b", "bl", "br", "l", "r", "t", "tl", "tr"];

/// Breakpoint names in the default Tailwind CSS theme.
const DEFAULT_BREAKPOINTS: &[&str] = &["sm", "md", "lg", "xl", "2xl"];

/// Legacy `bg-*` and `object-*` positions and the values that replace them.
const POSITIONS: &[(&str, &str)] = &[
    ("left-bottom", "bottom-left"),
    ("left-top", "top-left"),
    ("right-bottom", "bottom-right"),
    ("right-top", "top-right"),
];

#[cfg(test)]
mod tests {
    use super::{GRADIENT_DIRECTIONS, legacy_utilities};
    use biome_tailwind_parser::parse_tailwind;

    #[test]
    fn gradient_directions_are_sorted() {
        for pair in GRADIENT_DIRECTIONS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must sort before {}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn replaces_legacy_utilities() {
        for (class, expected) in [
            ("bg-gradient-to-t", "bg-linear-to-t"),
            ("bg-gradient-to-tr", "bg-linear-to-tr"),
            ("bg-gradient-to-r", "bg-linear-to-r"),
            ("bg-gradient-to-br", "bg-linear-to-br"),
            ("bg-gradient-to-b", "bg-linear-to-b"),
            ("bg-gradient-to-bl", "bg-linear-to-bl"),
            ("bg-gradient-to-l", "bg-linear-to-l"),
            ("bg-gradient-to-tl", "bg-linear-to-tl"),
            ("bg-left-top", "bg-top-left"),
            ("bg-right-top", "bg-top-right"),
            ("bg-left-bottom", "bg-bottom-left"),
            ("bg-right-bottom", "bg-bottom-right"),
            ("object-left-top", "object-top-left"),
            ("object-right-top", "object-top-right"),
            ("object-left-bottom", "object-bottom-left"),
            ("object-right-bottom", "object-bottom-right"),
            ("max-w-screen-lg", "max-w-(--breakpoint-lg)"),
            ("max-w-screen-2xl", "max-w-(--breakpoint-2xl)"),
            ("overflow-ellipsis", "text-ellipsis"),
            ("decoration-slice", "box-decoration-slice"),
            ("decoration-clone", "box-decoration-clone"),
            ("flex-grow", "grow"),
            ("flex-grow-0", "grow-0"),
            ("flex-grow-[2]", "grow-[2]"),
            ("flex-grow-(--factor)", "grow-(--factor)"),
            ("flex-shrink", "shrink"),
            ("flex-shrink-0", "shrink-0"),
            ("order-none", "order-0"),
            ("break-words", "wrap-break-word"),
            ("start-auto", "inset-s-auto"),
            ("start-full", "inset-s-full"),
            ("-start-full", "-inset-s-full"),
            ("start-px", "inset-s-px"),
            ("-start-px", "-inset-s-px"),
            ("start-4", "inset-s-4"),
            ("start-0.5", "inset-s-0.5"),
            ("start-1/2", "inset-s-1/2"),
            ("start-(--offset)", "inset-s-(--offset)"),
            ("start-[3px]", "inset-s-[3px]"),
            ("end-auto", "inset-e-auto"),
            ("end-full", "inset-e-full"),
            ("-end-full", "-inset-e-full"),
            ("end-px", "inset-e-px"),
            ("-end-px", "-inset-e-px"),
            ("-end-4", "-inset-e-4"),
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}: {:?}", parse.diagnostics());
            let root = parse.tree();
            let found = legacy_utilities(&root.candidates());
            assert_eq!(found.len(), 1, "{class}");
            assert_eq!(
                found[0].texts(&root),
                Some((class.to_string(), expected.to_string())),
                "{class}"
            );
        }
    }

    #[test]
    fn fix_keeps_variants_and_surrounding_classes() {
        let source = "p-4 md:hover:flex-grow-0! text-red-500";
        let parse = parse_tailwind(source);
        let root = parse.tree();
        let found = legacy_utilities(&root.candidates());
        assert_eq!(found.len(), 1);
        assert_eq!(&source[found[0].range().unwrap()], "flex-grow-0");
        assert_eq!(
            found[0].fixed_class_string(&root).as_deref(),
            Some("p-4 md:hover:grow-0! text-red-500")
        );
    }

    #[test]
    fn ignores_current_utilities() {
        for class in [
            "bg-linear-to-r",
            "bg-top-left",
            "bg-left",
            "object-top-left",
            "max-w-screen",
            "max-w-lg",
            "max-w-(--breakpoint-lg)",
            "text-ellipsis",
            "overflow-hidden",
            "box-decoration-slice",
            "decoration-wavy",
            "grow",
            "grow-0",
            "shrink-0",
            "flex",
            "flex-1",
            "order-0",
            "order-first",
            "wrap-break-word",
            "break-all",
            "inset-s-4",
            "-inset-e-px",
            "bg-gradient-to-x",
            "-bg-left-top",
            "-overflow-ellipsis",
            "max-w-screen-[100px]",
            "max-w-screen-wrapper",
            "start-date",
            "end-of-list",
            "-start-auto",
            "start-0.3",
            "start-01",
            "start-1/x",
            "start-full/50",
            "flex-grow-wrapper",
            "flex-grow-01",
            "flex-grow-1.5",
            "order-none/50",
            "[order:none]",
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}: {:?}", parse.diagnostics());
            assert!(
                legacy_utilities(&parse.tree().candidates()).is_empty(),
                "{class}"
            );
        }
    }
}
