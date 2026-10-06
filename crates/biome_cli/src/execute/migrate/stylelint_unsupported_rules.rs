//! Metadata about Stylelint rules that have no Biome equivalent.
//!
//! Stylelint deprecated its stylistic rules in v15 and removed them in v16,
//! recommending a formatter instead. A rule is reported as covered when the
//! output of Biome's formatter satisfies it with its conventional options, for
//! example `lower` for the `*-case` rules.
//!
//! `unicode-bom` is absent because Biome's formatter preserves a byte order mark
//! instead of normalizing it.

use biome_analyze::RuleSource::*;
use biome_analyze::UnsupportedRule;
use biome_analyze::UnsupportedRuleReason::*;

/// The array is sorted to allow binary search.
pub const STYLELINT_UNSUPPORTED_RULES: &[UnsupportedRule] = &[
    UnsupportedRule(Stylelint("at-rule-name-case"), FormatterCovers),
    UnsupportedRule(Stylelint("at-rule-name-newline-after"), FormatterCovers),
    UnsupportedRule(Stylelint("at-rule-name-space-after"), FormatterCovers),
    UnsupportedRule(
        Stylelint("at-rule-semicolon-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("at-rule-semicolon-space-before"), FormatterCovers),
    UnsupportedRule(
        Stylelint("block-closing-brace-empty-line-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-closing-brace-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-closing-brace-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-closing-brace-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-closing-brace-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-opening-brace-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-opening-brace-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-opening-brace-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("block-opening-brace-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("color-hex-case"), FormatterCovers),
    UnsupportedRule(Stylelint("declaration-bang-space-after"), FormatterCovers),
    UnsupportedRule(Stylelint("declaration-bang-space-before"), FormatterCovers),
    UnsupportedRule(
        Stylelint("declaration-block-semicolon-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("declaration-block-semicolon-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("declaration-block-semicolon-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("declaration-block-semicolon-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("declaration-block-trailing-semicolon"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("declaration-colon-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("declaration-colon-space-after"), FormatterCovers),
    UnsupportedRule(Stylelint("declaration-colon-space-before"), FormatterCovers),
    UnsupportedRule(Stylelint("function-comma-newline-after"), FormatterCovers),
    UnsupportedRule(Stylelint("function-comma-newline-before"), FormatterCovers),
    UnsupportedRule(Stylelint("function-comma-space-after"), FormatterCovers),
    UnsupportedRule(Stylelint("function-comma-space-before"), FormatterCovers),
    UnsupportedRule(Stylelint("function-max-empty-lines"), FormatterCovers),
    UnsupportedRule(
        Stylelint("function-parentheses-newline-inside"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("function-parentheses-space-inside"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("function-whitespace-after"), FormatterCovers),
    UnsupportedRule(Stylelint("indentation"), FormatterOption("indentWidth")),
    UnsupportedRule(Stylelint("linebreaks"), FormatterOption("lineEnding")),
    UnsupportedRule(Stylelint("max-empty-lines"), FormatterCovers),
    UnsupportedRule(Stylelint("max-line-length"), FormatterOption("lineWidth")),
    UnsupportedRule(
        Stylelint("media-feature-colon-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-feature-colon-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("media-feature-name-case"), FormatterCovers),
    UnsupportedRule(
        Stylelint("media-feature-parentheses-space-inside"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-feature-range-operator-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-feature-range-operator-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-query-list-comma-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-query-list-comma-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-query-list-comma-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("media-query-list-comma-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("no-empty-first-line"), FormatterCovers),
    UnsupportedRule(Stylelint("no-eol-whitespace"), FormatterCovers),
    UnsupportedRule(Stylelint("no-extra-semicolons"), FormatterCovers),
    UnsupportedRule(
        Stylelint("no-missing-end-of-source-newline"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("number-leading-zero"), FormatterCovers),
    UnsupportedRule(Stylelint("number-no-trailing-zeros"), FormatterCovers),
    UnsupportedRule(Stylelint("property-case"), FormatterCovers),
    UnsupportedRule(
        Stylelint("selector-attribute-brackets-space-inside"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-attribute-operator-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-attribute-operator-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-combinator-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-combinator-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-descendant-combinator-no-non-space"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-list-comma-newline-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-list-comma-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-list-comma-space-after"),
        FormatterCovers,
    ),
    UnsupportedRule(
        Stylelint("selector-list-comma-space-before"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("selector-max-empty-lines"), FormatterCovers),
    UnsupportedRule(Stylelint("selector-pseudo-class-case"), FormatterCovers),
    UnsupportedRule(
        Stylelint("selector-pseudo-class-parentheses-space-inside"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("selector-pseudo-element-case"), FormatterCovers),
    UnsupportedRule(Stylelint("string-quotes"), FormatterOption("quoteStyle")),
    UnsupportedRule(Stylelint("unit-case"), FormatterCovers),
    UnsupportedRule(Stylelint("value-list-comma-newline-after"), FormatterCovers),
    UnsupportedRule(
        Stylelint("value-list-comma-newline-before"),
        FormatterCovers,
    ),
    UnsupportedRule(Stylelint("value-list-comma-space-after"), FormatterCovers),
    UnsupportedRule(Stylelint("value-list-comma-space-before"), FormatterCovers),
    UnsupportedRule(Stylelint("value-list-max-empty-lines"), FormatterCovers),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylelint_unsupported_rules_strictly_sorted() {
        let is_sorted = STYLELINT_UNSUPPORTED_RULES
            .windows(2)
            .all(|pair| pair[0].0 < pair[1].0);

        if !is_sorted {
            let mut sorted_rules: Vec<_> = STYLELINT_UNSUPPORTED_RULES.iter().collect();
            sorted_rules.sort_by_key(|rule| rule.0);
            panic!(
                "STYLELINT_UNSUPPORTED_RULES is not sorted or has duplicates. Expected order:\n{sorted_rules:?}"
            );
        }
    }
}
