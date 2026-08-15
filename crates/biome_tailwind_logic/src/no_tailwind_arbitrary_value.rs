use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, TwCandidateList,
};

/// Returns the ranges of arbitrary values and properties in `candidates`.
/// The ranges are relative to the parsed Tailwind source.
pub fn analyze_tailwind_arbitrary_values(candidates: &TwCandidateList) -> Vec<TextRange> {
    let mut results = Vec::new();

    for candidate in candidates.iter() {
        let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
            continue;
        };

        match candidate.candidate() {
            Ok(AnyTwCandidate::TwArbitraryCandidate(candidate)) => {
                results.push(candidate.syntax().text_trimmed_range());
            }
            Ok(AnyTwCandidate::TwFunctionalCandidate(candidate)) => {
                push_arbitrary_value_range(&mut results, candidate.value().ok());
                push_modifier_range(&mut results, candidate.modifier());
            }
            _ => {}
        }
    }

    results
}

fn push_arbitrary_value_range(results: &mut Vec<TextRange>, value: Option<AnyTwValue>) {
    if let Some(AnyTwValue::TwArbitraryValue(value)) = value {
        results.push(value.syntax().text_trimmed_range());
    }
}

fn push_modifier_range(results: &mut Vec<TextRange>, modifier: Option<AnyTwModifier>) {
    if let Some(AnyTwModifier::TwModifier(modifier)) = modifier {
        push_arbitrary_value_range(results, modifier.value().ok());
    }
}

#[cfg(test)]
mod tests {
    use biome_tailwind_parser::parse_tailwind;

    use super::analyze_tailwind_arbitrary_values;

    #[test]
    fn finds_arbitrary_values_and_properties() {
        let parse = parse_tailwind("w-[400px] text-[#555] [color:red] text-red-500/[0.31]");
        let ranges = analyze_tailwind_arbitrary_values(&parse.tree().candidates());

        assert_eq!(ranges.len(), 4);
    }

    #[test]
    fn ignores_arbitrary_variants() {
        let parse = parse_tailwind("[&:nth-child(3)]:px-2 has-[:checked]:bg-red-500");
        let ranges = analyze_tailwind_arbitrary_values(&parse.tree().candidates());

        assert!(ranges.is_empty());
    }
}
