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
use std::collections::HashMap;

declare_lint_rule! {
    /// Enforce using the opacity modifier syntax for Tailwind CSS colors.
    ///
    /// Tailwind CSS v3+ supports an opacity modifier syntax like `bg-red-500/50` instead of
    /// using separate opacity utilities like `bg-red-500 bg-opacity-50`. The modifier syntax
    /// is more concise, more readable, and works better with arbitrary values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="bg-red-500 bg-opacity-50" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="text-blue-600 text-opacity-75" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="border-green-400 border-opacity-25" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="bg-red-500/50" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="text-blue-600/75" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="border-green-400/25" />;
    /// ```
    ///
    pub UseTailwindColorOpacityModifier {
        version: "next",
        name: "useTailwindColorOpacityModifier",
        language: "jsx",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

/// Opacity utility prefixes and their corresponding color utility prefixes
const OPACITY_TO_COLOR_MAP: &[(&str, &str)] = &[
    ("bg-opacity-", "bg-"),
    ("text-opacity-", "text-"),
    ("border-opacity-", "border-"),
    ("divide-opacity-", "divide-"),
    ("ring-opacity-", "ring-"),
    ("placeholder-opacity-", "placeholder-"),
];

/// State for the fix
pub struct OpacityModifierState {
    /// The fixed class string
    pub fixed_classes: String,
    /// Description of the transformation
    pub transformations: Vec<String>,
}

impl Rule for UseTailwindColorOpacityModifier {
    type Query = Ast<AnyClassStringLike>;
    type State = OpacityModifierState;
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

        let classes: Vec<&str> = value_str.split_whitespace().collect();

        // Find opacity utilities and their corresponding color utilities
        let mut opacity_classes: HashMap<&str, (&str, &str)> = HashMap::new(); // opacity_prefix -> (opacity_class, opacity_value)
        let mut color_classes: HashMap<&str, &str> = HashMap::new(); // color_prefix -> color_class

        for class in &classes {
            // Check if it's an opacity utility
            for (opacity_prefix, color_prefix) in OPACITY_TO_COLOR_MAP {
                if let Some(opacity_value) = class.strip_prefix(opacity_prefix) {
                    opacity_classes.insert(color_prefix, (class, opacity_value));
                    break;
                }
            }

            // Check if it's a color utility that could use the modifier
            for (opacity_prefix, color_prefix) in OPACITY_TO_COLOR_MAP {
                if class.starts_with(color_prefix) && !class.contains('/') {
                    // Skip if it's actually an opacity utility
                    if class.starts_with(opacity_prefix) {
                        break;
                    }
                    // Make sure it's actually a color class (has a color value like -red-500)
                    let rest = &class[color_prefix.len()..];
                    if is_color_value(rest) {
                        color_classes.insert(color_prefix, class);
                    }
                    break;
                }
            }
        }

        // Find matches where we have both a color and its opacity
        let mut transformations = Vec::new();
        let mut classes_to_remove: Vec<&str> = Vec::new();
        let mut classes_to_modify: HashMap<&str, String> = HashMap::new();

        for (color_prefix, (opacity_class, opacity_value)) in &opacity_classes {
            if let Some(color_class) = color_classes.get(color_prefix) {
                // Found a match! Transform color_class to include opacity
                let new_class = format!("{}/{}", color_class, opacity_value);
                transformations.push(format!(
                    "`{}` + `{}` → `{}`",
                    color_class, opacity_class, new_class
                ));
                classes_to_remove.push(opacity_class);
                classes_to_modify.insert(color_class, new_class);
            }
        }

        if transformations.is_empty() {
            return None;
        }

        // Build the fixed class string
        let fixed_classes: Vec<String> = classes
            .iter()
            .filter(|c| !classes_to_remove.contains(c))
            .map(|c| {
                if let Some(modified) = classes_to_modify.get(c) {
                    modified.clone()
                } else {
                    (*c).to_string()
                }
            })
            .collect();

        Some(OpacityModifierState {
            fixed_classes: fixed_classes.join(" "),
            transformations,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let transformations_str = state.transformations.join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use the opacity modifier syntax instead of separate opacity utilities."
                },
            )
            .note(markup! {
                "Transformations: "{transformations_str}
            })
            .note(markup! {
                "The opacity modifier syntax (e.g., "<Emphasis>"bg-red-500/50"</Emphasis>") is more concise and the preferred approach in modern Tailwind CSS."
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
            markup! { "Use opacity modifier syntax" }.to_owned(),
            mutation,
        ))
    }
}

/// Check if a string looks like a Tailwind color value (e.g., "red-500", "blue-600", "[#ff0000]")
fn is_color_value(s: &str) -> bool {
    // Arbitrary color values
    if s.starts_with('[') && s.ends_with(']') {
        return true;
    }

    // Named colors with shades (e.g., red-500, blue-600)
    // Also handles things like slate-50, zinc-900, etc.
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() >= 2 {
        let last = parts.last().unwrap();
        // Check if last part is a shade number
        if last.parse::<u32>().is_ok() {
            return true;
        }
    }

    // Special color values
    matches!(
        s,
        "inherit"
            | "current"
            | "transparent"
            | "black"
            | "white"
            | "slate"
            | "gray"
            | "zinc"
            | "neutral"
            | "stone"
            | "red"
            | "orange"
            | "amber"
            | "yellow"
            | "lime"
            | "green"
            | "emerald"
            | "teal"
            | "cyan"
            | "sky"
            | "blue"
            | "indigo"
            | "violet"
            | "purple"
            | "fuchsia"
            | "pink"
            | "rose"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_color_value() {
        assert!(is_color_value("red-500"));
        assert!(is_color_value("blue-600"));
        assert!(is_color_value("slate-50"));
        assert!(is_color_value("[#ff0000]"));
        assert!(is_color_value("black"));
        assert!(is_color_value("white"));
        assert!(is_color_value("transparent"));

        assert!(!is_color_value("4"));
        assert!(!is_color_value("auto"));
        assert!(!is_color_value("full"));
    }
}
