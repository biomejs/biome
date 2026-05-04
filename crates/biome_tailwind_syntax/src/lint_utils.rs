use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};

use crate::{AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, TwCandidateList};

/// Collects text ranges of all arbitrary values in the given candidate list.
///
/// `content_start` is the source offset of the first character of the parsed
/// string, used to translate parse-relative ranges into source ranges.
pub fn arbitrary_ranges(candidates: &TwCandidateList, content_start: TextSize) -> Vec<TextRange> {
    let mut results = Vec::new();

    for candidate in candidates.iter() {
        let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
            continue;
        };

        match candidate.candidate() {
            Ok(AnyTwCandidate::TwArbitraryCandidate(candidate)) => {
                let range = candidate.syntax().text_trimmed_range();
                results.push(TextRange::new(
                    content_start + range.start(),
                    content_start + range.end(),
                ));
            }
            Ok(AnyTwCandidate::TwFunctionalCandidate(candidate)) => {
                push_arbitrary_value_range(&mut results, content_start, candidate.value().ok());
                push_modifier_range(&mut results, content_start, candidate.modifier());
            }
            _ => {}
        }
    }

    results
}

fn push_arbitrary_value_range(
    results: &mut Vec<TextRange>,
    content_start: TextSize,
    value: Option<AnyTwValue>,
) {
    if let Some(AnyTwValue::TwArbitraryValue(value)) = value {
        let range = value.syntax().text_trimmed_range();
        results.push(TextRange::new(
            content_start + range.start(),
            content_start + range.end(),
        ));
    }
}

fn push_modifier_range(
    results: &mut Vec<TextRange>,
    content_start: TextSize,
    modifier: Option<AnyTwModifier>,
) {
    if let Some(AnyTwModifier::TwModifier(modifier)) = modifier {
        push_arbitrary_value_range(results, content_start, modifier.value().ok());
    }
}
