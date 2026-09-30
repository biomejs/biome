use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArguments, JsNewOrCallExpression,
    JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken, global_identifier, inner_string_text,
    static_value::StaticValue, unescape_js_string,
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
    /// Some patterns that are valid without these flags become a syntax error
    /// in Unicode mode, for example `/{/` or `/\-/`. The rule still reports them,
    /// but no fix is offered because adding the `u` flag would break the code.
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
        version: "2.4.5",
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
                    "This regular expression is missing the "<Emphasis>"u"</Emphasis>" or "<Emphasis>"v"</Emphasis>" flag."
                },
            )
            .note(markup! {
                "Without the "<Emphasis>"u"</Emphasis>" or "<Emphasis>"v"</Emphasis>" flag, this regular expression may not correctly handle Unicode characters, "
                "such as emoji or characters outside the Basic Multilingual Plane."
            })
            .note(markup! {
                "The "<Emphasis>"u"</Emphasis>" flag enables Unicode mode which correctly handles surrogate pairs and Unicode escapes. "
                "The "<Emphasis>"v"</Emphasis>" flag (ES2024) enables Unicode Sets mode with additional features like set notation in character classes."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if !is_pattern_valid_with_unicode_flag(node)? {
            return None;
        }
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

                    let mut text = String::new();
                    let mut leading = vec![];
                    let mut trailing = vec![];

                    for t in token.leading_trivia().pieces() {
                        text.push_str(t.text());
                        leading.push(TriviaPiece::new(t.kind(), t.text_len()));
                    }
                    text.push_str(&new_flags);
                    for t in token.trailing_trivia().pieces() {
                        text.push_str(t.text());
                        trailing.push(TriviaPiece::new(t.kind(), t.text_len()));
                    }

                    let new_token = JsSyntaxToken::new_detached(
                        JsSyntaxKind::JS_STRING_LITERAL,
                        &text,
                        leading,
                        trailing,
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
    match expr
        .clone()
        .omit_parentheses()
        .as_any_global_identifier_expression()
        .and_then(|e| global_identifier(&e))
    {
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

/// Returns whether the pattern of the regular expression is known to stay
/// valid once the `u` flag is added.
///
/// Returns `None` if the pattern can't be retrieved statically.
fn is_pattern_valid_with_unicode_flag(node: &AnyRegexExpression) -> Option<bool> {
    match node {
        AnyRegexExpression::JsRegexLiteralExpression(regex) => {
            let (pattern, _) = regex.decompose().ok()?;
            Some(is_valid_unicode_pattern(pattern.text()))
        }
        AnyRegexExpression::JsNewOrCallExpression(expr) => {
            let (_, arguments) = parse_regexp_node(expr)?;
            let pattern = arguments.args().iter().next()?.ok()?;
            let pattern = pattern.as_any_js_expression()?.clone().omit_parentheses();
            match pattern.as_static_value()? {
                StaticValue::EmptyString(_) => Some(true),
                StaticValue::String(token) if token.kind() == JsSyntaxKind::JS_STRING_LITERAL => {
                    let text = inner_string_text(&token);
                    // Legacy octal escapes, allowed in sloppy mode, aren't
                    // supported by `unescape_js_string`.
                    if has_legacy_octal_escape(text.text()) {
                        return Some(false);
                    }
                    let pattern = unescape_js_string(text);
                    // Lone surrogates are replaced with U+FFFD when unescaped,
                    // so the pattern can't be checked reliably.
                    Some(!pattern.contains('\u{fffd}') && is_valid_unicode_pattern(&pattern))
                }
                _ => None,
            }
        }
    }
}

/// Returns whether the raw text of a string literal contains a legacy octal
/// escape (`\1`, `\01`, `\173`) or a non-octal decimal escape (`\8`, `\9`).
fn has_legacy_octal_escape(text: &str) -> bool {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('1'..='9') => return true,
                Some('0') if chars.peek().is_some_and(char::is_ascii_digit) => return true,
                _ => {}
            }
        }
    }
    false
}

/// Conservatively checks that a pattern, valid without the `u` flag, is also
/// valid with it.
///
/// Unicode mode doesn't allow the web compatibility syntax of Annex B, such as
/// lone quantifier brackets (`/{/`, `/]/`), identity escapes (`/\a/`), or
/// character class escapes used as range endpoints (`/[\d-z]/`).
/// When in doubt, this function returns `false`.
fn is_valid_unicode_pattern(pattern: &str) -> bool {
    let chars: Vec<char> = pattern.chars().collect();

    // Count the capturing groups to validate backreferences.
    let mut capturing_groups = 0;
    let mut has_named_groups = false;
    let mut in_class = false;
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '\\' => i += 1,
            '[' => in_class = true,
            ']' => in_class = false,
            '(' if !in_class => match (chars.get(i + 1), chars.get(i + 2), chars.get(i + 3)) {
                (Some('?'), Some('<'), Some(c)) if !matches!(c, '=' | '!') => {
                    capturing_groups += 1;
                    has_named_groups = true;
                }
                (Some('?'), _, _) => {}
                _ => capturing_groups += 1,
            },
            _ => {}
        }
        i += 1;
    }

    // Whether each open group is a lookaround assertion.
    let mut groups: Vec<bool> = Vec::new();
    // Whether the current character class contains an endpoint that may
    // change or be invalid in Unicode mode, and whether it contains a range.
    let mut class_has_unsafe_endpoint = false;
    let mut class_has_range = false;
    let mut class_start = 0;
    in_class = false;
    i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_class {
            match c {
                '\\' => match chars.get(i + 1) {
                    Some('d' | 'D' | 's' | 'S' | 'w' | 'W') => {
                        class_has_unsafe_endpoint = true;
                        i += 1;
                    }
                    Some('b' | '-') => i += 1,
                    // Without the `u` flag, it is an octal escape. With it, a
                    // decimal escape isn't allowed in a character class.
                    Some('1'..='9') => return false,
                    _ => match unicode_character_escape_len(&chars, i, capturing_groups) {
                        Some(len) => i += len - 1,
                        None => return false,
                    },
                },
                ']' => {
                    if class_has_unsafe_endpoint && class_has_range {
                        return false;
                    }
                    in_class = false;
                }
                '-' if i > class_start && chars.get(i + 1) != Some(&']') => {
                    class_has_range = true;
                }
                c if c > '\u{ffff}' => class_has_unsafe_endpoint = true,
                _ => {}
            }
        } else {
            match c {
                '\\' => match chars.get(i + 1) {
                    Some('b' | 'B' | 'd' | 'D' | 's' | 'S' | 'w' | 'W') => i += 1,
                    Some('k') if has_named_groups => i += 1,
                    _ => match unicode_character_escape_len(&chars, i, capturing_groups) {
                        Some(len) => i += len - 1,
                        None => return false,
                    },
                },
                '[' => {
                    in_class = true;
                    class_has_unsafe_endpoint = false;
                    class_has_range = false;
                    class_start = if chars.get(i + 1) == Some(&'^') {
                        i + 2
                    } else {
                        i + 1
                    };
                    i = class_start - 1;
                }
                '(' => {
                    if chars.get(i + 1) == Some(&'?') {
                        match (chars.get(i + 2), chars.get(i + 3)) {
                            (Some(':'), _) => groups.push(false),
                            (Some('=' | '!'), _) | (Some('<'), Some('=' | '!')) => {
                                groups.push(true);
                            }
                            (Some('<'), _) => {
                                let name_len = chars[i + 3..]
                                    .iter()
                                    .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '$'))
                                    .count();
                                if name_len == 0 || chars.get(i + 3 + name_len) != Some(&'>') {
                                    return false;
                                }
                                groups.push(false);
                            }
                            // Modifiers and other extensions
                            _ => return false,
                        }
                    } else {
                        groups.push(false);
                    }
                }
                ')' => {
                    // Lookaheads can be quantified only without the `u` flag.
                    if groups.pop().unwrap_or(true)
                        && matches!(chars.get(i + 1), Some('*' | '+' | '?' | '{'))
                    {
                        return false;
                    }
                }
                '{' => {
                    // A brace must start a valid quantifier in Unicode mode.
                    let min_len = chars[i + 1..].iter().take_while(|c| c.is_ascii_digit()).count();
                    if min_len == 0 {
                        return false;
                    }
                    let mut end = i + 1 + min_len;
                    if chars.get(end) == Some(&',') {
                        end += 1;
                        end += chars[end..].iter().take_while(|c| c.is_ascii_digit()).count();
                    }
                    if chars.get(end) != Some(&'}') {
                        return false;
                    }
                    i = end;
                }
                '}' | ']' => return false,
                _ => {}
            }
        }
        i += 1;
    }
    !in_class
}

/// Returns the length of the character escape starting at `chars[start]`
/// (a backslash) if it is valid in Unicode mode and has the same meaning
/// without the `u` flag.
fn unicode_character_escape_len(
    chars: &[char],
    start: usize,
    capturing_groups: usize,
) -> Option<usize> {
    let is_hex = |index: usize| chars.get(index).is_some_and(char::is_ascii_hexdigit);
    match chars.get(start + 1)? {
        '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
        | '/' | 'f' | 'n' | 'r' | 't' | 'v' => Some(2),
        '0' => (!chars.get(start + 2).is_some_and(char::is_ascii_digit)).then_some(2),
        'c' => chars
            .get(start + 2)
            .is_some_and(char::is_ascii_alphabetic)
            .then_some(3),
        'x' => (is_hex(start + 2) && is_hex(start + 3)).then_some(4),
        'u' => {
            if !(start + 2..start + 6).all(is_hex) {
                return None;
            }
            let hex: String = chars[start + 2..start + 6].iter().collect();
            let value = u32::from_str_radix(&hex, 16).ok()?;
            // Surrogate pairs are combined into a single code point in Unicode mode.
            (!(0xd800..=0xdfff).contains(&value)).then_some(6)
        }
        '1'..='9' => {
            let len = chars[start + 1..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .count();
            let digits: String = chars[start + 1..start + 1 + len].iter().collect();
            // Without the `u` flag, a backreference to a missing group is an octal escape.
            let group = digits.parse::<usize>().ok()?;
            (group <= capturing_groups).then_some(len + 1)
        }
        _ => None,
    }
}
