use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsCallExpression, JsPropertyObjectMember, JsxAttribute,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_react_native_literal_colors::NoReactNativeLiteralColorsOptions;

use crate::frameworks::is_framework_api_reference;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow color literals in React Native styles.
    ///
    /// Hard-coding colors inside styles makes it harder to keep them consistent
    /// across components and to swap the palette when the design system evolves.
    /// Extracting colors into named constants or a shared theme module produces
    /// more maintainable code.
    ///
    /// This rule reports properties whose name contains `color` (case-insensitive)
    /// and whose value is a string literal, when they appear inside a
    /// `StyleSheet.create` call or inside a JSX attribute whose name contains
    /// `style` (case-insensitive). A ternary expression is also reported when
    /// either branch is a string literal.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const Hello = () => <Text style={{ backgroundColor: '#FFFFFF' }}>hi</Text>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const styles = StyleSheet.create({
    ///     text: { color: 'red' }
    /// });
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const Hello = (flag) => (
    ///     <Text style={{ backgroundColor: flag ? '#fff' : '#000' }}>hi</Text>
    /// );
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const red = '#f00';
    /// const styles = StyleSheet.create({
    ///     text: { color: red }
    /// });
    /// ```
    ///
    /// ```jsx
    /// const Hello = () => (
    ///     <Text style={{ backgroundColor: theme.background }}>hi</Text>
    /// );
    /// ```
    ///
    pub NoReactNativeLiteralColors {
        version: "next",
        name: "noReactNativeLiteralColors",
        language: "js",
        sources: &[RuleSource::EslintReactNative("no-color-literals").same()],
        domains: &[RuleDomain::ReactNative],
        recommended: false,
    }
}

impl Rule for NoReactNativeLiteralColors {
    type Query = Semantic<AnyStyleSink>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoReactNativeLiteralColorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyStyleSink::JsxAttribute(attribute) => {
                if !is_style_attribute(attribute) {
                    return Vec::new();
                }
                node.collect_color_literal_properties()
            }
            AnyStyleSink::JsCallExpression(call) => {
                if !is_stylesheet_create(call, ctx.model()) {
                    return Vec::new();
                }
                node.collect_color_literal_properties()
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Color literals are not allowed inside styles."
                },
            )
            .note(markup! {
                "Inline colors are hard to keep consistent across screens and to adapt when the design palette changes."
            })
            .note(markup! {
                "Extract the color into a named constant or a shared theme module, and reference it from the style."
            }),
        )
    }
}

declare_node_union! {
    /// The two places where React Native style objects can appear: a JSX
    /// attribute like `style={...}` or a call like `StyleSheet.create(...)`.
    pub AnyStyleSink = JsxAttribute | JsCallExpression
}

impl AnyStyleSink {
    /// Walks all descendant `JsPropertyObjectMember` nodes and returns the text
    /// range of each one whose name contains `color` and whose value is a color
    /// literal (a string, or a ternary where at least one branch is a string).
    fn collect_color_literal_properties(&self) -> Vec<TextRange> {
        self.syntax()
            .descendants()
            .filter_map(JsPropertyObjectMember::cast)
            .filter(|property| {
                property
                    .name()
                    .ok()
                    .and_then(|name| name.name())
                    .is_some_and(|name| contains_ignore_ascii_case(name.text(), "color"))
            })
            .filter(|property| {
                property
                    .value()
                    .ok()
                    .is_some_and(|value| has_color_literal_value(&value))
            })
            .map(|property| property.range())
            .collect()
    }
}

fn has_color_literal_value(value: &AnyJsExpression) -> bool {
    match value {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsConditionalExpression(conditional) => {
            conditional
                .consequent()
                .ok()
                .is_some_and(|consequent| consequent.is_string_literal())
                || conditional
                    .alternate()
                    .ok()
                    .is_some_and(|alternate| alternate.is_string_literal())
        }
        _ => false,
    }
}

fn is_style_attribute(attribute: &JsxAttribute) -> bool {
    attribute
        .name()
        .ok()
        .and_then(|name| name.name().ok())
        .is_some_and(|token| contains_ignore_ascii_case(token.text_trimmed(), "style"))
}

/// Returns `true` when `call` is a call to `StyleSheet.create` where
/// `StyleSheet` is either imported from `react-native`/`react-native-web` or
/// is an unresolved global with that name. A `StyleSheet` identifier bound to
/// a user declaration (local variable, import from another package, …) is
/// rejected, so the rule only fires on the real React Native API.
fn is_stylesheet_create(call: &JsCallExpression, model: &biome_js_semantic::SemanticModel) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    is_framework_api_reference(
        &callee,
        model,
        "create",
        REACT_NATIVE_PACKAGE_NAMES,
        Some("StyleSheet"),
    )
}

const REACT_NATIVE_PACKAGE_NAMES: &[&str] = &["react-native", "react-native-web"];

/// Returns `true` when `needle` appears anywhere inside `haystack`, treating
/// uppercase and lowercase ASCII letters as equal. For example, searching for
/// `"color"` inside `"backgroundColor"` returns `true`.
///
/// Only ASCII letters are folded. Any other character (digits, punctuation,
/// or non-ASCII letters like `É`) must match byte-for-byte.
fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    haystack
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
}

#[cfg(test)]
mod tests {
    use super::contains_ignore_ascii_case;

    #[test]
    fn matches_exactly() {
        assert!(contains_ignore_ascii_case("color", "color"));
    }

    #[test]
    fn matches_at_start() {
        assert!(contains_ignore_ascii_case("colorScheme", "color"));
    }

    #[test]
    fn matches_at_end() {
        assert!(contains_ignore_ascii_case("backgroundcolor", "color"));
    }

    #[test]
    fn matches_in_the_middle() {
        assert!(contains_ignore_ascii_case("myColorValue", "color"));
    }

    #[test]
    fn ignores_ascii_case() {
        assert!(contains_ignore_ascii_case("backgroundColor", "color"));
        assert!(contains_ignore_ascii_case("BACKGROUNDCOLOR", "color"));
        assert!(contains_ignore_ascii_case("background", "GROUND"));
    }

    #[test]
    fn rejects_missing_needle() {
        assert!(!contains_ignore_ascii_case("padding", "color"));
    }

    #[test]
    fn empty_needle_always_matches() {
        assert!(contains_ignore_ascii_case("anything", ""));
        assert!(contains_ignore_ascii_case("", ""));
    }

    #[test]
    fn needle_longer_than_haystack() {
        assert!(!contains_ignore_ascii_case("hi", "hello"));
        assert!(!contains_ignore_ascii_case("", "color"));
    }

    #[test]
    fn non_ascii_is_not_case_folded() {
        // "É" (U+00C9) and "é" (U+00E9) are two different bytes once
        // encoded as UTF-8, so they must not be treated as equal.
        assert!(!contains_ignore_ascii_case("café", "CAFÉ"));
        assert!(contains_ignore_ascii_case("café", "café"));
    }

    #[test]
    fn punctuation_and_digits_must_match_exactly() {
        assert!(contains_ignore_ascii_case("color-2", "COLOR-2"));
        assert!(!contains_ignore_ascii_case("color-2", "color_2"));
    }
}
