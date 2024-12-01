use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{CssFunction, CssParameter};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rowan::AstSeparatedList;
use biome_string_case::StrLikeExtension;
use regex::Regex;
use std::sync::LazyLock;

use crate::utils::vendor_prefixed;

declare_lint_rule! {
    /// Disallow non-standard direction values for linear gradient functions.
    ///
    /// A valid and standard direction value is one of the following:
    /// - an angle
    /// - to plus a side-or-corner (`to top`, `to bottom`, `to left`, `to right`; `to top right`, `to right top`, `to bottom left`, etc.)
    ///
    /// A common mistake (matching outdated non-standard syntax) is to use just a side-or-corner without the preceding to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .foo { background: linear-gradient(top, #fff, #000); }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .foo { background: linear-gradient(45, #fff, #000); }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .foo { background: linear-gradient(to top, #fff, #000); }
    /// ```
    ///
    /// ```css
    /// .foo { background: linear-gradient(45deg, #fff, #000); }
    /// ```
    ///
    pub NoInvalidDirectionInLinearGradient {
        version: "1.9.0",
        name: "noInvalidDirectionInLinearGradient",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("function-linear-gradient-no-nonstandard-direction")],
    }
}

// It is necessary to find case-insensitive string.
// Also Check if 'in' is a word boundary.
// For examples,`to top in srgb` is valid but `to top insrgb` is not valid.
pub static IN_KEYWORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bin\b").unwrap());

// This regex checks if a string consists of a number immediately followed by a unit, with no space between them.
// For examples, `45deg`, `45grad` is valid but `45 deg`, `45de` is not valid.
pub static ANGLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\d.]+(?:deg|grad|rad|turn)$").unwrap());

// This need for capture `side-or-corner` keyword from linear-gradient function.
// Ensure starts `side-or-corner` keyword `to` and ends with the keyword `side-or-corner`.
pub static DIRECTION_WITHOUT_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"(?i)^({0})(?: ({0}))?$", "top|left|bottom|right")).unwrap()
});

impl Rule for NoInvalidDirectionInLinearGradient {
    type Query = Ast<CssFunction>;
    type State = CssParameter;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let node_name = node.name().ok()?.to_trimmed_string();
        let linear_gradient_property = [
            "linear-gradient",
            "-webkit-linear-gradient",
            "-moz-linear-gradient",
            "-o-linear-gradient",
            "-ms-linear-gradient",
        ];
        if !linear_gradient_property.contains(&node_name.to_ascii_lowercase_cow().as_ref()) {
            return None;
        }
        let css_parameter = node.items();

        let first_css_parameter = css_parameter.first()?.ok()?;
        let first_css_parameter_text = first_css_parameter.to_trimmed_string();
        if IN_KEYWORD.is_match(&first_css_parameter_text) {
            return None;
        }
        if let Some(first_byte) = first_css_parameter_text.bytes().next() {
            if first_byte.is_ascii_digit() {
                if ANGLE.is_match(&first_css_parameter_text) {
                    return None;
                }
                return Some(first_css_parameter);
            }
        }
        let direction_property = ["top", "left", "bottom", "right"];
        if !direction_property.iter().any(|&keyword| {
            first_css_parameter_text
                .to_ascii_lowercase_cow()
                .contains(keyword)
        }) {
            return None;
        }
        let has_prefix = vendor_prefixed(&node_name);
        if !is_standdard_direction(&first_css_parameter_text, has_prefix) {
            return Some(first_css_parameter);
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected nonstandard direction"
                },
            ).note(markup! {
                "You should fix the direction value to follow the syntax."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient">"MDN web docs"</Hyperlink>" for more details."
            })
        )
    }
}

fn is_standdard_direction(direction: &str, has_prefix: bool) -> bool {
    let matches = match (has_prefix, direction.starts_with("to ")) {
        (true, false) => DIRECTION_WITHOUT_TO.captures(direction),
        (false, true) => DIRECTION_WITHOUT_TO.captures(&direction[3..]),
        _ => None,
    };
    if let Some(matches) = matches {
        match (matches.get(1), matches.get(2)) {
            (Some(_), None) => {
                return true;
            }
            (Some(first_direction), Some(second_direction)) => {
                if first_direction.as_str() != second_direction.as_str() {
                    return true;
                }
            }
            _ => return true,
        }
    }
    false
}
