use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsNewOrCallExpression,
    JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPiece, declare_node_union};
use biome_rule_options::use_unicode_regex::UseUnicodeRegexOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce the use of the `u` or `v` flag for regular expressions.
    ///
    /// The `u` flag (Unicode mode) and `v` flag (Unicode Sets mode) enable proper handling
    /// of Unicode characters in regular expressions. Without these flags, regex patterns
    /// may not correctly match Unicode characters like emoji or characters outside the
    /// Basic Multilingual Plane.
    ///
    /// The `u` flag was introduced in ES2015 and enables:
    /// - Correct handling of surrogate pairs (e.g., emoji)
    /// - Unicode code point escapes (`\u{...}`)
    /// - Case-insensitive matching for Unicode characters
    ///
    /// The `v` flag was introduced in ES2024 and provides all `u` flag features plus:
    /// - Set notation in character classes
    /// - String literals in character classes
    /// - Improved Unicode property escapes
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /foo/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo/gi;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new RegExp("foo");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new RegExp("foo", "gi");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /foo/u;
    /// /foo/v;
    /// /foo/giu;
    /// new RegExp("foo", "u");
    /// new RegExp("foo", "giv");
    /// new RegExp("foo", flags); // dynamic flags are ignored
    /// ```
    ///
    pub UseUnicodeRegex {
        version: "next",
        name: "useUnicodeRegex",
        language: "js",
        sources: &[RuleSource::Eslint("require-unicode-regexp").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyRegexExpression = JsRegexLiteralExpression | JsNewOrCallExpression
}

pub enum UseUnicodeRegexState {
    Literal,
    Constructor { has_flags_arg: bool },
}

impl Rule for UseUnicodeRegex {
    type Query = Semantic<AnyRegexExpression>;
    type State = UseUnicodeRegexState;
    type Signals = Option<Self::State>;
    type Options = UseUnicodeRegexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            AnyRegexExpression::JsRegexLiteralExpression(regex) => check_regex_literal(regex),
            AnyRegexExpression::JsNewOrCallExpression(expr) => {
                check_regexp_constructor(expr, model)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use the "<Emphasis>"u"</Emphasis>" or "<Emphasis>"v"</Emphasis>" flag for this regular expression."
                },
            )
            .note(markup! {
                "The "<Emphasis>"u"</Emphasis>" flag enables Unicode mode which correctly handles Unicode characters. "
                "The "<Emphasis>"v"</Emphasis>" flag (ES2024) enables Unicode Sets mode with additional features."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match (node, state) {
            (
                AnyRegexExpression::JsRegexLiteralExpression(regex),
                UseUnicodeRegexState::Literal,
            ) => {
                let token = regex.value_token().ok()?;
                let mut text = String::new();
                let mut leading = vec![];
                let mut trailing = vec![];

                for t in token.leading_trivia().pieces() {
                    text.push_str(t.text());
                    leading.push(TriviaPiece::new(t.kind(), t.text_len()));
                }
                text.push_str(token.text_trimmed());
                text.push('u');
                for t in token.trailing_trivia().pieces() {
                    text.push_str(t.text());
                    trailing.push(TriviaPiece::new(t.kind(), t.text_len()));
                }

                let new_token = JsSyntaxToken::new_detached(
                    JsSyntaxKind::JS_REGEX_LITERAL,
                    &text,
                    leading,
                    trailing,
                );
                mutation.replace_token(token, new_token);
            }
            (
                AnyRegexExpression::JsNewOrCallExpression(expr),
                UseUnicodeRegexState::Constructor { has_flags_arg },
            ) => {
                if !has_flags_arg {
                    // No flags argument - skip auto-fix (would need to add argument)
                    return None;
                }

                let (_, arguments) = parse_regexp_node(expr)?;
                let args = arguments.args();
                let flags_arg = args.iter().nth(1)?.ok()?;

                if let AnyJsCallArgument::AnyJsExpression(flags_expr) = flags_arg {
                    let token = flags_expr.syntax().first_token()?;
                    let token_text = token.text();

                    // Preserve original quote style
                    let quote_char = token_text.chars().next()?;
                    if quote_char != '"' && quote_char != '\'' {
                        // Not a simple string literal (template literal, etc.)
                        return None;
                    }

                    let static_val = flags_expr.as_static_value()?;
                    let flags_text = static_val.as_string_constant()?;
                    let new_flags = format!("{}{}u{}", quote_char, flags_text, quote_char);
                    let new_token = JsSyntaxToken::new_detached(
                        JsSyntaxKind::JS_STRING_LITERAL,
                        &new_flags,
                        [],
                        [],
                    );
                    mutation.replace_token(token, new_token);
                }
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"u"</Emphasis>" flag." }.to_owned(),
            mutation,
        ))
    }
}

fn check_regex_literal(regex: &JsRegexLiteralExpression) -> Option<UseUnicodeRegexState> {
    let (_, flags) = regex.decompose().ok()?;
    let flags_text = flags.text();

    if flags_text.contains('u') || flags_text.contains('v') {
        None
    } else {
        Some(UseUnicodeRegexState::Literal)
    }
}

fn check_regexp_constructor(
    expr: &JsNewOrCallExpression,
    model: &SemanticModel,
) -> Option<UseUnicodeRegexState> {
    let (callee, arguments) = parse_regexp_node(expr)?;

    // Check if callee is global RegExp
    if !is_regexp_object(&callee, model) {
        return None;
    }

    let args = arguments.args();

    // Check if first argument is spread - cannot statically analyze
    let first_arg = args.iter().next();
    if matches!(first_arg, Some(Ok(AnyJsCallArgument::JsSpread(_)))) {
        return None;
    }

    // Need at least pattern argument
    if args.is_empty() {
        return None;
    }

    // Check flags argument (second argument)
    let flags_arg = args.iter().nth(1);

    match flags_arg {
        Some(Ok(AnyJsCallArgument::AnyJsExpression(flags_expr))) => {
            // Try to get static value of flags
            match flags_expr.as_static_value() {
                Some(val) => {
                    // Static flags - check if u or v is present
                    let flags_text = val.as_string_constant()?;
                    if flags_text.contains('u') || flags_text.contains('v') {
                        None
                    } else {
                        Some(UseUnicodeRegexState::Constructor {
                            has_flags_arg: true,
                        })
                    }
                }
                None => {
                    // Dynamic flags (variable) - ignore
                    None
                }
            }
        }
        Some(Ok(AnyJsCallArgument::JsSpread(_))) => {
            // Spread argument - ignore
            None
        }
        Some(Err(_)) => None,
        None => {
            // No flags argument at all
            Some(UseUnicodeRegexState::Constructor {
                has_flags_arg: false,
            })
        }
    }
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

fn parse_regexp_node(node: &JsNewOrCallExpression) -> Option<(AnyJsExpression, JsCallArguments)> {
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
