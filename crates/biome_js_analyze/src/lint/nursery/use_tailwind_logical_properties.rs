use crate::JsRuleAction;
use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make::{
    js_string_literal, js_string_literal_expression, js_string_literal_single_quotes, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;

declare_lint_rule! {
    /// Enforce using logical properties for Tailwind CSS spacing and positioning.
    ///
    /// Logical properties like `ps-*` (padding-inline-start) and `me-*` (margin-inline-end)
    /// automatically adapt to the text direction, making your styles work correctly in
    /// both LTR and RTL layouts without additional code.
    ///
    /// This rule suggests replacing physical properties (`pl-*`, `pr-*`, `ml-*`, `mr-*`)
    /// with their logical equivalents (`ps-*`, `pe-*`, `ms-*`, `me-*`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="pl-4" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="mr-2 ml-4" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="left-0 right-4" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="ps-4" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="me-2 ms-4" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="start-0 end-4" />;
    /// ```
    ///
    /// ## Notes
    ///
    /// Physical properties may still be appropriate when you explicitly need
    /// direction-independent positioning (e.g., for visual design elements that
    /// should not flip in RTL layouts).
    ///
    pub UseTailwindLogicalProperties {
        version: "next",
        name: "useTailwindLogicalProperties",
        language: "jsx",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// Mapping from physical to logical property prefixes
/// (physical_prefix, logical_prefix)
const PHYSICAL_TO_LOGICAL: &[(&str, &str)] = &[
    // Padding
    ("pl-", "ps-"),
    ("pr-", "pe-"),
    // Margin
    ("ml-", "ms-"),
    ("mr-", "me-"),
    // Positioning
    ("left-", "start-"),
    ("right-", "end-"),
    // Inset
    ("inset-x-", "inset-inline-"),
    // Border radius (corners)
    ("rounded-tl-", "rounded-ss-"),
    ("rounded-tr-", "rounded-se-"),
    ("rounded-bl-", "rounded-es-"),
    ("rounded-br-", "rounded-ee-"),
    ("rounded-l-", "rounded-s-"),
    ("rounded-r-", "rounded-e-"),
    // Border width
    ("border-l-", "border-s-"),
    ("border-r-", "border-e-"),
    // Scroll margin
    ("scroll-ml-", "scroll-ms-"),
    ("scroll-mr-", "scroll-me-"),
    // Scroll padding
    ("scroll-pl-", "scroll-ps-"),
    ("scroll-pr-", "scroll-pe-"),
];

/// Single-value physical properties (no prefix after)
const PHYSICAL_TO_LOGICAL_EXACT: &[(&str, &str)] = &[
    ("left-0", "start-0"),
    ("right-0", "end-0"),
    ("left-auto", "start-auto"),
    ("right-auto", "end-auto"),
    ("left-full", "end-full"),
    ("right-full", "end-full"),
    ("text-left", "text-start"),
    ("text-right", "text-end"),
    ("float-left", "float-start"),
    ("float-right", "float-end"),
    ("clear-left", "clear-start"),
    ("clear-right", "clear-end"),
];

/// State for the fix
pub struct LogicalPropertiesState {
    pub fixed_classes: String,
    pub transformations: Vec<(String, String)>, // (old, new)
}

impl Rule for UseTailwindLogicalProperties {
    type Query = Ast<AnyClassStringLike>;
    type State = LogicalPropertiesState;
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

        let mut transformations = Vec::new();
        let mut fixed_classes = Vec::new();

        for class in value_str.split_whitespace() {
            if let Some(logical_class) = convert_to_logical(class) {
                transformations.push((class.to_string(), logical_class.clone()));
                fixed_classes.push(logical_class);
            } else {
                fixed_classes.push(class.to_string());
            }
        }

        if transformations.is_empty() {
            return None;
        }

        Some(LogicalPropertiesState {
            fixed_classes: fixed_classes.join(" "),
            transformations,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let changes: Vec<String> = state
            .transformations
            .iter()
            .map(|(old, new)| format!("`{}` → `{}`", old, new))
            .collect();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use logical properties for better RTL/LTR support."
                },
            )
            .note(markup! {
                "Suggested changes: "{changes.join(", ")}
            })
            .note(markup! {
                "Logical properties automatically adapt to text direction, improving internationalization support."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let fixed = &state.fixed_classes;
        match node {
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let is_double_quote = jsx_string_node
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_jsx_quote().is_double());
                let replacement = jsx_string(if is_double_quote {
                    js_string_literal(fixed)
                } else {
                    js_string_literal_single_quotes(fixed)
                });
                mutation.replace_node(jsx_string_node.clone(), replacement);
            }
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let is_double_quote = string_literal
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_quote().is_double());
                let replacement = js_string_literal_expression(if is_double_quote {
                    js_string_literal(fixed)
                } else {
                    js_string_literal_single_quotes(fixed)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use logical properties" }.to_owned(),
            mutation,
        ))
    }
}

/// Convert a physical property class to its logical equivalent
/// Returns None if no conversion is needed
fn convert_to_logical(class: &str) -> Option<String> {
    // Extract any variants (hover:, md:, etc.)
    let parts: Vec<&str> = class.rsplitn(2, ':').collect();
    let (utility, variants) = if parts.len() == 2 {
        (parts[0], Some(parts[1]))
    } else {
        (parts[0], None)
    };

    // Check exact matches first
    for (physical, logical) in PHYSICAL_TO_LOGICAL_EXACT {
        if utility == *physical {
            return Some(if let Some(v) = variants {
                format!("{}:{}", v, logical)
            } else {
                (*logical).to_string()
            });
        }
    }

    // Check prefix matches
    for (physical_prefix, logical_prefix) in PHYSICAL_TO_LOGICAL {
        if utility.starts_with(physical_prefix) {
            let rest = &utility[physical_prefix.len()..];
            let logical_utility = format!("{}{}", logical_prefix, rest);
            return Some(if let Some(v) = variants {
                format!("{}:{}", v, logical_utility)
            } else {
                logical_utility
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_padding() {
        assert_eq!(convert_to_logical("pl-4"), Some("ps-4".to_string()));
        assert_eq!(convert_to_logical("pr-2"), Some("pe-2".to_string()));
    }

    #[test]
    fn test_convert_margin() {
        assert_eq!(convert_to_logical("ml-4"), Some("ms-4".to_string()));
        assert_eq!(convert_to_logical("mr-auto"), Some("me-auto".to_string()));
    }

    #[test]
    fn test_convert_position() {
        assert_eq!(convert_to_logical("left-0"), Some("start-0".to_string()));
        assert_eq!(convert_to_logical("right-4"), Some("end-4".to_string()));
    }

    #[test]
    fn test_convert_with_variants() {
        assert_eq!(
            convert_to_logical("hover:pl-4"),
            Some("hover:ps-4".to_string())
        );
        assert_eq!(
            convert_to_logical("md:mr-2"),
            Some("md:me-2".to_string())
        );
    }

    #[test]
    fn test_no_conversion_needed() {
        assert_eq!(convert_to_logical("ps-4"), None);
        assert_eq!(convert_to_logical("flex"), None);
        assert_eq!(convert_to_logical("pt-4"), None); // top/bottom don't need conversion
        assert_eq!(convert_to_logical("mb-2"), None);
    }

    #[test]
    fn test_convert_text_alignment() {
        assert_eq!(
            convert_to_logical("text-left"),
            Some("text-start".to_string())
        );
        assert_eq!(
            convert_to_logical("text-right"),
            Some("text-end".to_string())
        );
    }
}
