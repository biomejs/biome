use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_rowan::AstNode;
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;

declare_lint_rule! {
    /// Disallow empty arbitrary values in Tailwind CSS classes.
    ///
    /// Empty arbitrary values like `w-[]` or `text-[]` are invalid and will not
    /// produce any CSS output. They typically indicate incomplete code or a mistake.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="w-[]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="text-[] bg-[]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="hover:p-[]" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="w-[100px]" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="text-[14px] bg-[#ff0000]" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="w-24 p-4" />;
    /// ```
    ///
    pub NoEmptyTailwindArbitraryValue {
        version: "next",
        name: "noEmptyTailwindArbitraryValue",
        language: "jsx",
        recommended: true,
    }
}

/// State containing the found empty arbitrary classes
pub struct EmptyArbitraryState {
    pub empty_classes: Vec<String>,
}

impl Rule for NoEmptyTailwindArbitraryValue {
    type Query = Ast<AnyClassStringLike>;
    type State = EmptyArbitraryState;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();

        if !node.should_visit(options)? {
            return None;
        }

        let value = node.value()?;
        let value_str = value.text();

        let mut empty_classes = Vec::new();

        for class in value_str.split_whitespace() {
            if contains_empty_arbitrary_value(class) {
                empty_classes.push(class.to_string());
            }
        }

        if empty_classes.is_empty() {
            return None;
        }

        Some(EmptyArbitraryState { empty_classes })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let classes_str = state
            .empty_classes
            .iter()
            .map(|s| format!("`{}`", s))
            .collect::<Vec<_>>()
            .join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Empty arbitrary value"<Emphasis>{
                        if state.empty_classes.len() > 1 { "s" } else { "" }
                    }</Emphasis>" detected: "{classes_str}
                },
            )
            .note(markup! {
                "Empty arbitrary values like "<Emphasis>"w-[]"</Emphasis>" are invalid and produce no CSS. Add a value or remove the brackets."
            }),
        )
    }
}

/// Check if a class contains an empty arbitrary value (e.g., `w-[]`, `text-[]`)
fn contains_empty_arbitrary_value(class: &str) -> bool {
    // Look for -[] pattern (empty brackets after a hyphen)
    class.contains("-[]")
        // Also check for standalone [] at the end
        || class.ends_with("[]")
        // Check for [&] which is also essentially empty/useless
        || class.contains("-[&]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_empty_arbitrary_value() {
        // Should detect empty arbitrary values
        assert!(contains_empty_arbitrary_value("w-[]"));
        assert!(contains_empty_arbitrary_value("text-[]"));
        assert!(contains_empty_arbitrary_value("bg-[]"));
        assert!(contains_empty_arbitrary_value("hover:w-[]"));
        assert!(contains_empty_arbitrary_value("md:p-[]"));

        // Should not detect non-empty arbitrary values
        assert!(!contains_empty_arbitrary_value("w-[100px]"));
        assert!(!contains_empty_arbitrary_value("bg-[#ff0000]"));
        assert!(!contains_empty_arbitrary_value("text-[14px]"));

        // Should not detect regular classes
        assert!(!contains_empty_arbitrary_value("w-24"));
        assert!(!contains_empty_arbitrary_value("bg-red-500"));
        assert!(!contains_empty_arbitrary_value("text-sm"));
    }
}
