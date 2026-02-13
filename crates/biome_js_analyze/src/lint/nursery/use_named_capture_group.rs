use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsNewOrCallExpression,
    JsRegexLiteralExpression, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize};
use biome_rule_options::use_named_capture_group::UseNamedCaptureGroupOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce using named capture groups in regular expression.
    ///
    /// Numbered capture groups like `(...)` can be difficult to work with,
    /// as they are matched by their position and not by a descriptive name.
    /// Named capture groups (`(?<name>...)`) associate a descriptive name
    /// with each match, making the regular expression more readable and
    /// its intent clearer.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /(ba[rz])/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /([0-9]{4})/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /(?:ab)(cd)/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new RegExp("(foo)");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// RegExp("(foo)");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /(?<id>ba[rz])/;
    /// ```
    ///
    /// ```js
    /// /(?:ba[rz])/;
    /// ```
    ///
    /// ```js
    /// /ba[rz]/;
    /// ```
    ///
    /// ```js
    /// /(?<year>[0-9]{4})-(?<month>[0-9]{2})/;
    /// ```
    ///
    /// ```js
    /// new RegExp("(?<id>foo)");
    /// ```
    ///
    /// ```js
    /// new RegExp(pattern);
    /// ```
    ///
    pub UseNamedCaptureGroup {
        version: "next",
        name: "useNamedCaptureGroup",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-named-capture-group").same()],
        recommended: false,
    }
}

/// Find byte offsets of unnamed capture groups in a regex pattern.
///
/// Returns a list of byte offsets (relative to pattern start) for each
/// unnamed capture group `(` found.
fn find_unnamed_capture_groups(pattern: &[u8]) -> Vec<u32> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < pattern.len() {
        match pattern[i] {
            b'\\' => {
                // Skip escaped character
                i += 2;
            }
            b'[' => {
                // Skip character class
                i += 1;
                while i < pattern.len() {
                    match pattern[i] {
                        b'\\' => {
                            i += 2;
                        }
                        b']' => {
                            i += 1;
                            break;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
            }
            b'(' => {
                if pattern.get(i + 1) == Some(&b'?') {
                    // `(?:`, `(?=`, `(?!`, `(?<=`, `(?<!`, `(?<name>` — skip
                    i += 1;
                } else {
                    // Unnamed capture group
                    result.push(i as u32);
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    result
}

fn has_unnamed_capture_groups(pattern: &[u8]) -> bool {
    !find_unnamed_capture_groups(pattern).is_empty()
}

fn is_regexp_object(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    match global_identifier(&expr.clone().omit_parentheses()) {
        Some((reference, name)) => match model.binding(&reference) {
            Some(_) if !reference.is_global_this() && !reference.has_name("window") => false,
            _ => name.text() == "RegExp",
        },
        None => false,
    }
}

fn parse_regexp_node(
    node: &JsNewOrCallExpression,
) -> Option<(AnyJsExpression, JsCallArguments)> {
    match node {
        JsNewOrCallExpression::JsNewExpression(node) => {
            let callee = node.callee().ok()?;
            let args = node.arguments()?;
            Some((callee, args))
        }
        JsNewOrCallExpression::JsCallExpression(node) => {
            let callee = node.callee().ok()?;
            let args = node.arguments().ok()?;
            Some((callee, args))
        }
    }
}

fn extract_pattern_from_args(
    arguments: &JsCallArguments,
) -> Option<String> {
    let args = arguments.args();
    let mut iter = args.iter();
    let first_arg = iter.next()?;
    let Ok(AnyJsCallArgument::AnyJsExpression(expr)) = first_arg else {
        return None;
    };
    let static_val = expr.omit_parentheses().as_static_value()?;
    let text = static_val.as_string_constant()?;
    Some(text.to_string())
}

impl Rule for UseNamedCaptureGroup {
    type Query = Semantic<AnyJsExpression>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = UseNamedCaptureGroupOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyJsExpression::AnyJsLiteralExpression(
                biome_js_syntax::AnyJsLiteralExpression::JsRegexLiteralExpression(regex),
            ) => run_regex_literal(regex),
            AnyJsExpression::JsNewExpression(_) | AnyJsExpression::JsCallExpression(_) => {
                run_regexp_constructor(node, ctx.model())
            }
            _ => Default::default(),
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Capture group is not named."
                },
            )
            .note(markup! {
                "Named capture groups improve readability by associating a descriptive name with each match. Use "<Emphasis>"(?<name>...)"</Emphasis>" instead of "<Emphasis>"(...)"</Emphasis>"."
            }),
        )
    }
}

fn run_regex_literal(node: &JsRegexLiteralExpression) -> Box<[TextRange]> {
    let Ok((pattern, _flags)) = node.decompose() else {
        return Default::default();
    };
    let pattern_text = pattern.text();
    let offsets = find_unnamed_capture_groups(pattern_text.as_bytes());
    if offsets.is_empty() {
        return Default::default();
    }
    let pattern_start = node.range().start() + TextSize::from(1);
    offsets
        .into_iter()
        .map(|offset| {
            let start = pattern_start + TextSize::from(offset);
            TextRange::new(start, start + TextSize::from(1))
        })
        .collect()
}

fn run_regexp_constructor(node: &AnyJsExpression, model: &SemanticModel) -> Box<[TextRange]> {
    let new_or_call = match node {
        AnyJsExpression::JsNewExpression(n) => JsNewOrCallExpression::from(n.clone()),
        AnyJsExpression::JsCallExpression(n) => JsNewOrCallExpression::from(n.clone()),
        _ => return Default::default(),
    };
    let Some((callee, arguments)) = parse_regexp_node(&new_or_call) else {
        return Default::default();
    };
    if !is_regexp_object(&callee, model) {
        return Default::default();
    }
    let Some(pattern_text) = extract_pattern_from_args(&arguments) else {
        return Default::default();
    };
    if !has_unnamed_capture_groups(pattern_text.as_bytes()) {
        return Default::default();
    }
    // For constructor calls, emit a single diagnostic on the entire expression
    Box::new([node.range()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unnamed_capture_groups() {
        assert_eq!(find_unnamed_capture_groups(b"(foo)"), vec![0]);
        assert_eq!(find_unnamed_capture_groups(b"(?:foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(?<name>foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(?=foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(?!foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(?<=foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(?<!foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"(foo)(bar)"), vec![0, 5]);
        assert_eq!(find_unnamed_capture_groups(b"\\(foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"\\\\(foo)"), vec![2]);
        assert_eq!(find_unnamed_capture_groups(b"[(]foo"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups(b"[\\]]foo"), Vec::<u32>::new());
    }
}
