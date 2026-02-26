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
    /// /(?:ba[rz])/;
    /// /ba[rz]/;
    /// /(?<year>[0-9]{4})-(?<month>[0-9]{2})/;
    /// new RegExp("(?<id>foo)");
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

/// Find byte offsets of unnamed capture groups in a regex pattern.
///
/// Returns a list of byte offsets (relative to pattern start) for each
/// unnamed capture group `(` found.
fn find_unnamed_capture_groups(pattern: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let mut bytes = pattern.as_bytes().iter().enumerate().peekable();
    while let Some((i, &byte)) = bytes.next() {
        match byte {
            b'\\' => {
                bytes.next();
            }
            b'[' => {
                while let Some((_, &b)) = bytes.next() {
                    match b {
                        b'\\' => {
                            bytes.next();
                        }
                        b']' => break,
                        _ => {}
                    }
                }
            }
            b'(' => {
                if bytes.peek().is_some_and(|&(_, &b)| b == b'?') {
                    // `(?:`, `(?=`, `(?!`, `(?<=`, `(?<!`, `(?<name>` — skip
                } else {
                    result.push(i as u32);
                }
            }
            _ => {}
        }
    }
    result
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

fn get_first_arg_expr(arguments: &JsCallArguments) -> Option<AnyJsExpression> {
    let first_arg = arguments.args().iter().next()?;
    let Ok(AnyJsCallArgument::AnyJsExpression(expr)) = first_arg else {
        return None;
    };
    Some(expr)
}

/// Try to compute precise TextRange for each unnamed group in a string literal.
/// Returns `Some` if the argument is a simple string literal without escape sequences
/// (so byte offsets map 1:1 to source positions). Returns `None` otherwise.
fn try_precise_string_ranges(arg_expr: &AnyJsExpression) -> Option<Box<[TextRange]>> {
    let AnyJsExpression::AnyJsLiteralExpression(
        biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
    ) = arg_expr
    else {
        return None;
    };
    let token = string_lit.value_token().ok()?;
    let token_text = token.text_trimmed();
    let raw_inner = &token_text[1..token_text.len() - 1];
    let inner_text = string_lit.inner_string_text().ok()?;
    // If raw source and interpreted text differ, escapes are present
    if raw_inner != inner_text.text() {
        return None;
    }
    let offsets = find_unnamed_capture_groups(raw_inner);
    if offsets.is_empty() {
        return Some(Default::default());
    }
    let content_start = token.text_trimmed_range().start() + TextSize::from(1);
    Some(
        offsets
            .into_iter()
            .map(|offset| {
                let start = content_start + TextSize::from(offset);
                TextRange::new(start, start + TextSize::from(1))
            })
            .collect(),
    )
}

fn run_regex_literal(node: &JsRegexLiteralExpression) -> Box<[TextRange]> {
    let Ok((pattern, _flags)) = node.decompose() else {
        return Default::default();
    };
    let pattern_text = pattern.text();
    let offsets = find_unnamed_capture_groups(pattern_text);
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
    let Some(arg_expr) = get_first_arg_expr(&arguments) else {
        return Default::default();
    };
    // Try precise per-group diagnostics for simple string literals (no escapes)
    if let Some(ranges) = try_precise_string_ranges(&arg_expr) {
        return ranges;
    }
    // Fallback: use interpreted value, single diagnostic on the whole expression
    let Some(static_val) = arg_expr.omit_parentheses().as_static_value() else {
        return Default::default();
    };
    let Some(pattern) = static_val.as_string_constant() else {
        return Default::default();
    };
    if find_unnamed_capture_groups(pattern).is_empty() {
        return Default::default();
    }
    Box::new([node.range()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unnamed_capture_groups() {
        assert_eq!(find_unnamed_capture_groups("(foo)"), vec![0]);
        assert_eq!(find_unnamed_capture_groups("(?:foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(?<name>foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(?=foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(?!foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(?<=foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(?<!foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("(foo)(bar)"), vec![0, 5]);
        assert_eq!(find_unnamed_capture_groups("\\(foo)"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("\\\\(foo)"), vec![2]);
        assert_eq!(find_unnamed_capture_groups("[(]foo"), Vec::<u32>::new());
        assert_eq!(find_unnamed_capture_groups("[\\]]foo"), Vec::<u32>::new());
    }
}
