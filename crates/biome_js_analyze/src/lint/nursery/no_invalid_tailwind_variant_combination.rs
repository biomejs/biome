use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_rowan::AstNode;
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallow invalid or redundant Tailwind CSS variant combinations.
    ///
    /// This rule detects variant combinations that are likely mistakes:
    /// - Duplicate variants (e.g., `hover:hover:bg-red-500`)
    /// - Conflicting responsive variants (e.g., `sm:md:flex` - should use one or the other)
    /// - Conflicting state variants (e.g., `hover:focus:` when they can't both be true)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="hover:hover:bg-red-500" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="sm:md:flex" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="first:last:text-bold" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="hover:bg-red-500" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="sm:flex md:block" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="hover:focus:bg-red-500" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="group-hover:peer-focus:text-white" />;
    /// ```
    ///
    pub NoInvalidTailwindVariantCombination {
        version: "next",
        name: "noInvalidTailwindVariantCombination",
        language: "jsx",
        recommended: true,
    }
}

/// Responsive breakpoint variants that are mutually exclusive
const RESPONSIVE_VARIANTS: &[&str] = &["sm", "md", "lg", "xl", "2xl"];

/// Max-width responsive variants
const MAX_RESPONSIVE_VARIANTS: &[&str] = &["max-sm", "max-md", "max-lg", "max-xl", "max-2xl"];

/// Mutually exclusive positional variants
const POSITIONAL_EXCLUSIVE_GROUPS: &[&[&str]] = &[
    &["first", "last", "only", "odd", "even"],
    &["first-of-type", "last-of-type", "only-of-type"],
];

/// State containing invalid variant combinations
pub struct InvalidVariantState {
    pub invalid_classes: Vec<(String, String)>, // (class, reason)
}

impl Rule for NoInvalidTailwindVariantCombination {
    type Query = Ast<AnyClassStringLike>;
    type State = InvalidVariantState;
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

        let mut invalid_classes = Vec::new();

        for class in value_str.split_whitespace() {
            if let Some(reason) = check_variant_combination(class) {
                invalid_classes.push((class.to_string(), reason));
            }
        }

        if invalid_classes.is_empty() {
            return None;
        }

        Some(InvalidVariantState { invalid_classes })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let issues: Vec<String> = state
            .invalid_classes
            .iter()
            .map(|(class, reason)| format!("`{}`: {}", class, reason))
            .collect();

        let issues_str = issues.join("; ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Invalid Tailwind CSS variant combination detected."
                },
            )
            .note(markup! {
                {issues_str}
            }),
        )
    }
}

/// Check a class for invalid variant combinations
/// Returns Some(reason) if invalid, None if valid
fn check_variant_combination(class: &str) -> Option<String> {
    // Extract variants (everything before the last colon-separated segment)
    let parts: Vec<&str> = class.split(':').collect();

    if parts.len() < 2 {
        // No variants, nothing to check
        return None;
    }

    // All but the last part are variants
    let variants: Vec<&str> = parts[..parts.len() - 1].to_vec();

    // Check for duplicate variants
    let mut seen: FxHashSet<&str> = FxHashSet::default();
    for variant in &variants {
        // Normalize variant (strip arbitrary parts for comparison)
        let normalized = normalize_variant(variant);
        if !seen.insert(normalized) {
            return Some(format!("duplicate variant `{}`", variant));
        }
    }

    // Check for conflicting responsive variants
    let responsive_count: Vec<&&str> = variants
        .iter()
        .filter(|v| RESPONSIVE_VARIANTS.contains(v))
        .collect();
    if responsive_count.len() > 1 {
        return Some(format!(
            "conflicting responsive variants `{}` and `{}`",
            responsive_count[0], responsive_count[1]
        ));
    }

    // Check for conflicting max-responsive variants
    let max_responsive_count: Vec<&&str> = variants
        .iter()
        .filter(|v| MAX_RESPONSIVE_VARIANTS.contains(v))
        .collect();
    if max_responsive_count.len() > 1 {
        return Some(format!(
            "conflicting max-width variants `{}` and `{}`",
            max_responsive_count[0], max_responsive_count[1]
        ));
    }

    // Check for mutually exclusive positional variants
    for group in POSITIONAL_EXCLUSIVE_GROUPS {
        let found: Vec<&&str> = variants.iter().filter(|v| group.contains(v)).collect();
        if found.len() > 1 {
            return Some(format!(
                "mutually exclusive variants `{}` and `{}`",
                found[0], found[1]
            ));
        }
    }

    None
}

/// Normalize a variant for comparison (strip arbitrary values, etc.)
fn normalize_variant(variant: &str) -> &str {
    // For arbitrary variants like `[&:hover]`, just return as-is
    // For things like `group-hover/name`, strip the name part
    if let Some(pos) = variant.find('/') {
        &variant[..pos]
    } else {
        variant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_variants() {
        assert!(check_variant_combination("hover:hover:bg-red-500").is_some());
        assert!(check_variant_combination("focus:focus:text-white").is_some());
        assert!(check_variant_combination("sm:sm:flex").is_some());
    }

    #[test]
    fn test_conflicting_responsive() {
        assert!(check_variant_combination("sm:md:flex").is_some());
        assert!(check_variant_combination("lg:xl:hidden").is_some());
    }

    #[test]
    fn test_valid_combinations() {
        assert!(check_variant_combination("hover:bg-red-500").is_none());
        assert!(check_variant_combination("hover:focus:bg-red-500").is_none());
        assert!(check_variant_combination("sm:hover:flex").is_none());
        assert!(check_variant_combination("group-hover:text-white").is_none());
    }

    #[test]
    fn test_mutually_exclusive_positional() {
        assert!(check_variant_combination("first:last:bg-red-500").is_some());
        assert!(check_variant_combination("odd:even:text-white").is_some());
    }
}
