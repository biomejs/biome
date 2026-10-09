use crate::class_category::candidate_category;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_syntax::{AnyTwCandidate, AnyTwFullCandidate, TwCandidateList, TwFullCandidate};

/// Returns parse-relative ranges of the arbitrary values and arbitrary properties
/// that `options` doesn't allow.
///
/// Arbitrary modifiers, such as `[0.5]` in `bg-red-500/[0.5]`, are not reported.
pub fn arbitrary_value_ranges(
    candidates: &TwCandidateList,
    options: &NoTailwindArbitraryValueOptions,
) -> Vec<TextRange> {
    candidates
        .iter()
        .flatten()
        .filter_map(|candidate| {
            let AnyTwFullCandidate::TwFullCandidate(full) = candidate else {
                return None;
            };
            let candidate = full.candidate().ok()?;
            let range = match &candidate {
                AnyTwCandidate::TwArbitraryCandidate(candidate) => {
                    candidate.syntax().text_trimmed_range()
                }
                AnyTwCandidate::TwFunctionalCandidate(candidate) => candidate
                    .value()
                    .ok()?
                    .as_tw_arbitrary_value()?
                    .syntax()
                    .text_trimmed_range(),
                _ => return None,
            };
            (!is_allowed(&full, &candidate, options)).then_some(range)
        })
        .collect()
}

/// Returns whether `options` allows `candidate`, the utility of `full`, by its category
/// or by its class. Classes are compared without variants or `!`.
fn is_allowed(
    full: &TwFullCandidate,
    candidate: &AnyTwCandidate,
    options: &NoTailwindArbitraryValueOptions,
) -> bool {
    if options
        .allowed_categories()
        .contains(&candidate_category(candidate))
    {
        return true;
    }
    let negative = full.negative_token().is_some();
    let text = candidate.syntax().text_trimmed();
    options.allowed_classes().iter().any(|class| {
        let class = if negative {
            class.strip_prefix('-')
        } else {
            Some(class.as_ref())
        };
        class.is_some_and(|class| text == class)
    })
}
