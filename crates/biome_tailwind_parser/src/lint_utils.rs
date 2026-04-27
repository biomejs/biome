use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};
use biome_tailwind_syntax::{
    AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue, TwCandidateList,
};

use crate::parse_tailwind;

pub fn class_ranges(text: &str) -> Vec<(usize, &str)> {
    let mut class_start = None;
    let mut classes = Vec::new();

    for (index, ch) in text.char_indices() {
        if ch.is_ascii_whitespace() {
            if let Some(start) = class_start.take() {
                classes.push((start, &text[start..index]));
            }
        } else if class_start.is_none() {
            class_start = Some(index);
        }
    }

    if let Some(start) = class_start {
        classes.push((start, &text[start..]));
    }

    classes
}

pub fn text_size(offset: usize) -> TextSize {
    TextSize::from(u32::try_from(offset).expect("class offset should fit into u32"))
}

pub fn push_arbitrary_value_range(
    results: &mut Vec<TextRange>,
    class_start: TextSize,
    value: Option<AnyTwValue>,
) {
    if let Some(AnyTwValue::TwArbitraryValue(value)) = value {
        let range = value.syntax().text_trimmed_range();
        results.push(TextRange::new(
            class_start + range.start(),
            class_start + range.end(),
        ));
    }
}

pub fn push_modifier_range(
    results: &mut Vec<TextRange>,
    class_start: TextSize,
    modifier: Option<AnyTwModifier>,
) {
    if let Some(AnyTwModifier::TwModifier(modifier)) = modifier {
        push_arbitrary_value_range(results, class_start, modifier.value().ok());
    }
}

pub fn scan_tailwind_arbitrary_ranges(text: &str, content_start: TextSize) -> Vec<TextRange> {
    let mut results = Vec::new();

    for (class_offset, class_name) in class_ranges(text) {
        let parse = parse_tailwind(class_name);
        let class_start = content_start + text_size(class_offset);

        collect_arbitrary_ranges_from_parse(&mut results, class_start, &parse.tree().candidates());
    }

    results
}

pub fn collect_arbitrary_ranges_from_parse(
    results: &mut Vec<TextRange>,
    class_start: TextSize,
    candidates: &TwCandidateList,
) {
    for candidate in candidates.iter() {
        let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
            continue;
        };

        match candidate.candidate() {
            Ok(AnyTwCandidate::TwArbitraryCandidate(candidate)) => {
                let range = candidate.syntax().text_trimmed_range();
                results.push(TextRange::new(
                    class_start + range.start(),
                    class_start + range.end(),
                ));
                push_modifier_range(results, class_start, candidate.modifier());
            }
            Ok(AnyTwCandidate::TwFunctionalCandidate(candidate)) => {
                push_arbitrary_value_range(results, class_start, candidate.value().ok());
                push_modifier_range(results, class_start, candidate.modifier());
            }
            _ => {}
        }
    }
}
