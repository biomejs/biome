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

        // Adding the `u` flag switches the pattern to Unicode mode, which is a
        // stricter grammar. Withhold the fix when the pattern would become
        // invalid, otherwise the fix produces a regex that throws at runtime.
        if !unicode_fix_is_safe(node, state) {
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

/// Returns whether appending the `u` flag keeps the pattern valid.
///
/// Returns `false` only for statically known patterns that are invalid in
/// Unicode mode. When the pattern cannot be determined statically, the
/// current behavior is preserved.
fn unicode_fix_is_safe(node: &AnyRegexExpression, state: &UseUnicodeRegexState) -> bool {
    match (node, state) {
        (AnyRegexExpression::JsRegexLiteralExpression(regex), UseUnicodeRegexState::Literal) => {
            let Ok((pattern, _)) = regex.decompose() else {
                return false;
            };
            is_valid_in_unicode_mode(pattern.text())
        }
        (
            AnyRegexExpression::JsNewOrCallExpression(expr),
            UseUnicodeRegexState::Constructor { .. },
        ) => match constructor_pattern(expr) {
            Some(pattern) => is_valid_in_unicode_mode(&pattern),
            None => true,
        },
        _ => true,
    }
}

/// Returns the pattern of a `RegExp` constructor call when it is a static
/// string literal, with escape sequences processed.
fn constructor_pattern(expr: &JsNewOrCallExpression) -> Option<String> {
    let (_, arguments) = parse_regexp_node(expr)?;
    let args = arguments.args();
    let first_arg = args.iter().next()?;
    let expr = match first_arg.ok()? {
        AnyJsCallArgument::AnyJsExpression(expr) => expr,
        _ => return None,
    };
    let value = expr.omit_parentheses().as_static_value()?;
    let StaticValue::String(token) = value else {
        return None;
    };
    Some(unescape_js_string(inner_string_text(&token)).to_string())
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
// Unicode-mode validity check for regular expression patterns.
//
// Adding the `u` flag switches a pattern to Unicode mode, which uses a
// stricter grammar: the Annex B extensions that are allowed in non-unicode
// mode (lone quantifier braces, identity escapes, out-of-order class ranges,
// decimal escapes, and others) become syntax errors. This check mirrors the
// V8/SpiderMonkey behavior so the `u`-flag fix is only offered when the
// resulting pattern stays valid.

use std::collections::HashSet;

/// Kind of a single pattern atom produced by an escape sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeKind {
    /// A single character with the given code point value.
    Char(u32),
    /// A character class escape (`\d`, `\D`, `\s`, `\S`, `\w`, `\W`):
    /// invalid as a character class range endpoint in unicode mode.
    ClassEscape,
    /// An assertion (`\b`, `\B` outside character classes): not quantifiable.
    Assertion,
    /// A backreference (`\1`, `\k<name>`): quantifiable, but never a range
    /// endpoint (decimal escapes are invalid inside classes in unicode mode).
    Atom,
}

fn is_hex_digit(c: Option<&char>) -> bool {
    matches!(c, Some('0'..='9' | 'a'..='f' | 'A'..='F'))
}

/// Scans a group name starting at `chars[i] == '<'` (the `<` of `(?<name>` or
/// `\k<name>`).
///
/// Returns the name and the index just past the closing `>`, or `None` when
/// there is no valid name. Group names are not validated beyond being
/// non-empty: the name syntax does not depend on unicode mode, so a name that
/// is invalid in unicode mode is already invalid without it.
fn scan_group_name(chars: &[char], i: usize) -> Option<(String, usize)> {
    if chars.get(i) != Some(&'<') {
        return None;
    }
    let mut j = i + 1;
    let start = j;
    while j < chars.len() && chars[j] != '>' {
        j += 1;
    }
    if j == start || j >= chars.len() {
        return None;
    }
    let name: String = chars[start..j].iter().collect();
    // Group names cannot start with an ASCII digit in unicode mode.
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    Some((name, j + 1))
}

/// Parses one escape sequence starting at `chars[i] == '\\'`.
///
/// Returns the atom kind and the index just past the escape, or `None` when
/// the escape is not valid in unicode mode.
fn parse_escape(
    chars: &[char],
    i: usize,
    in_class: bool,
    group_count: u32,
    group_names: &HashSet<String>,
) -> Option<(EscapeKind, usize)> {
    let c = *chars.get(i + 1)?;
    match c {
        // Identity escapes allowed in unicode mode: syntax characters and `/`.
        // `-` is additionally allowed inside character classes.
        '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
        | '/' => Some((EscapeKind::Char(c as u32), i + 2)),
        '-' if in_class => Some((EscapeKind::Char('-' as u32), i + 2)),
        // Character escapes.
        'f' => Some((EscapeKind::Char(0x0c), i + 2)),
        'n' => Some((EscapeKind::Char(0x0a), i + 2)),
        'r' => Some((EscapeKind::Char(0x0d), i + 2)),
        't' => Some((EscapeKind::Char(0x09), i + 2)),
        'v' => Some((EscapeKind::Char(0x0b), i + 2)),
        // `\b`: assertion outside classes, backspace inside classes.
        'b' if in_class => Some((EscapeKind::Char(0x08), i + 2)),
        'b' => Some((EscapeKind::Assertion, i + 2)),
        // `\B`: assertion, not allowed inside classes in unicode mode.
        'B' if !in_class => Some((EscapeKind::Assertion, i + 2)),
        // Character class escapes.
        'd' | 'D' | 's' | 'S' | 'w' | 'W' => Some((EscapeKind::ClassEscape, i + 2)),
        // Unicode property escapes: conservatively treated as unknown, the
        // fix is skipped for any pattern using them. (Note: `\p{...}` without
        // the `u` flag is an identity escape, so adding `u` would change the
        // pattern's meaning rather than just being invalid.)
        'p' | 'P' => None,
        // `\0` is only valid when not followed by a decimal digit.
        '0' => {
            if matches!(chars.get(i + 2), Some('0'..='9')) {
                None
            } else {
                Some((EscapeKind::Char(0), i + 2))
            }
        }
        // Decimal escapes: backreferences in unicode mode, never valid
        // inside character classes.
        '1'..='9' if !in_class => {
            let mut j = i + 1;
            while matches!(chars.get(j), Some('0'..='9')) {
                j += 1;
            }
            let n: u32 = chars[i + 1..j].iter().collect::<String>().parse().ok()?;
            if n >= 1 && n <= group_count {
                Some((EscapeKind::Atom, j))
            } else {
                None
            }
        }
        // Control escapes require an ASCII letter.
        'c' => {
            let letter = *chars.get(i + 2)?;
            if letter.is_ascii_alphabetic() {
                Some((EscapeKind::Char(letter as u32 % 32), i + 3))
            } else {
                None
            }
        }
        // Hex escapes require exactly two hex digits.
        'x' => {
            if is_hex_digit(chars.get(i + 2)) && is_hex_digit(chars.get(i + 3)) {
                let value =
                    u32::from_str_radix(&chars[i + 2..i + 4].iter().collect::<String>(), 16)
                        .ok()?;
                Some((EscapeKind::Char(value), i + 4))
            } else {
                None
            }
        }
        // Unicode escapes: `\uHHHH` or `\u{H+}` with a valid code point.
        'u' => {
            if chars.get(i + 2) == Some(&'{') {
                let mut j = i + 3;
                let start = j;
                while matches!(chars.get(j), Some(c) if c.is_ascii_hexdigit()) {
                    j += 1;
                }
                let len = j - start;
                if !(1..=6).contains(&len) || chars.get(j) != Some(&'}') {
                    return None;
                }
                let value =
                    u32::from_str_radix(&chars[start..j].iter().collect::<String>(), 16).ok()?;
                if value > 0x0010_FFFF {
                    return None;
                }
                Some((EscapeKind::Char(value), j + 1))
            } else if (0..4).all(|k| is_hex_digit(chars.get(i + 2 + k))) {
                let value =
                    u32::from_str_radix(&chars[i + 2..i + 6].iter().collect::<String>(), 16)
                        .ok()?;
                Some((EscapeKind::Char(value), i + 6))
            } else {
                None
            }
        }
        // Named backreference `\k<name>`: only valid outside character
        // classes, and the name must be defined.
        'k' if !in_class => {
            let (name, next) = scan_group_name(chars, i + 2)?;
            if group_names.contains(&name) {
                Some((EscapeKind::Atom, next))
            } else {
                None
            }
        }
        // Any other identity escape is invalid in unicode mode.
        _ => None,
    }
}
/// Parses a quantifier starting at `chars[i] == '{'`.
///
/// Returns the index just past the quantifier, or `None` for a lone `{`.
/// Digit runs that overflow `u32` are saturated: engines still accept the
/// quantifier shape, so it must not be mistaken for a lone `{`.
fn parse_quantifier(chars: &[char], i: usize) -> Option<usize> {
    let mut j = i + 1;
    let start = j;
    while matches!(chars.get(j), Some('0'..='9')) {
        j += 1;
    }
    if j == start {
        return None;
    }
    let min = parse_u32_saturating(&chars[start..j]);
    if chars.get(j) == Some(&',') {
        j += 1;
        let start_max = j;
        while matches!(chars.get(j), Some('0'..='9')) {
            j += 1;
        }
        if j > start_max {
            let max = parse_u32_saturating(&chars[start_max..j]);
            if max < min {
                return None;
            }
        }
    }
    if chars.get(j) != Some(&'}') {
        return None;
    }
    Some(j + 1)
}

/// Parses a run of ASCII digits, saturating at `u32::MAX` on overflow.
fn parse_u32_saturating(digits: &[char]) -> u32 {
    let mut value: u32 = 0;
    for c in digits {
        // Callers only pass ASCII digits.
        let digit = (*c as u32) - ('0' as u32);
        value = value.saturating_mul(10).saturating_add(digit);
    }
    value
}

/// Scans a group opening starting at `chars[i] == '('`.
///
/// Returns the index just past the opening and whether the group is a
/// lookaround assertion (`(?=...)`, `(?!...)`, `(?<=...)`, `(?<!...)`), or
/// `None` when the opening is not valid in unicode mode.
fn parse_group_open(chars: &[char], i: usize) -> Option<(usize, bool)> {
    if chars.get(i + 1) != Some(&'?') {
        return Some((i + 1, false));
    }
    match chars.get(i + 2) {
        Some(':' | '=' | '!') => Some((i + 3, matches!(chars[i + 2], '=' | '!'))),
        Some('<') => match chars.get(i + 3) {
            Some('=' | '!') => Some((i + 4, true)),
            _ => {
                let (_, next) = scan_group_name(chars, i + 2)?;
                Some((next, false))
            }
        },
        _ => None,
    }
}
/// Returns `true` if `pattern` remains a valid regular expression when the
/// `u` flag is added.
///
/// Patterns that are only valid in non-unicode (Annex B) mode, such as `/{/`,
/// `/]/`, `/\a/` or `/[\d-z]/`, become `SyntaxError`s in unicode mode, so the
/// `u` flag must not be added to them automatically.
fn is_valid_in_unicode_mode(pattern: &str) -> bool {
    let chars: Vec<char> = pattern.chars().collect();

    // Phase 1: count capturing groups and collect their names, so that
    // backreferences (including forward references) can be validated.
    // Duplicate group names are invalid, but so are they without the `u`
    // flag, so they are only tracked here for `\k<name>` validation.
    let (group_count, group_names) = {
        let mut count = 0u32;
        let mut names = HashSet::new();
        let mut duplicate_name = false;
        let mut i = 0;
        let mut in_class = false;
        while i < chars.len() {
            let c = chars[i];
            if in_class {
                if c == '\\' {
                    i += 1;
                } else if c == ']' {
                    in_class = false;
                }
            } else {
                match c {
                    '\\' => {
                        i += 1;
                    }
                    '[' => in_class = true,
                    '(' => {
                        if chars.get(i + 1) == Some(&'?') {
                            match chars.get(i + 2) {
                                Some(':' | '=' | '!') => {}
                                Some('<') => match chars.get(i + 3) {
                                    Some('=' | '!') => {}
                                    _ => {
                                        count += 1;
                                        if let Some((name, _)) = scan_group_name(&chars, i + 2)
                                            && !names.insert(name)
                                        {
                                            duplicate_name = true;
                                        }
                                    }
                                },
                                _ => {}
                            }
                        } else {
                            count += 1;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        if duplicate_name {
            return false;
        }
        (count, names)
    };

    // Phase 2: validate the pattern under unicode mode semantics.
    let mut i = 0;
    let mut in_class = false;
    // Whether a quantifier (`*`, `+`, `?`, `{n}`) may follow: only after a
    // quantifiable atom.
    let mut quantifiable = false;
    // For each open group, whether it is a lookaround assertion (which cannot
    // be quantified).
    let mut group_stack: Vec<bool> = Vec::new();
    // Kind of the previous atom inside a character class, for `-` ranges.
    let mut prev_atom: Option<EscapeKind> = None;

    while i < chars.len() {
        let c = chars[i];
        if in_class {
            match c {
                '\\' => {
                    let Some((kind, next)) =
                        parse_escape(&chars, i, true, group_count, &group_names)
                    else {
                        return false;
                    };
                    prev_atom = Some(kind);
                    i = next;
                }
                ']' => {
                    in_class = false;
                    prev_atom = None;
                    quantifiable = true;
                    i += 1;
                }
                '-' => {
                    let next = chars.get(i + 1);
                    if prev_atom.is_some() && !matches!(next, None | Some(']')) {
                        // A range: neither endpoint may be a character class
                        // escape or an assertion, and the range must ascend.
                        let left = match prev_atom {
                            Some(EscapeKind::Char(value)) => value,
                            _ => return false,
                        };
                        let (kind, next_i) = match chars[i + 1] {
                            '\\' => {
                                let Some((kind, next)) =
                                    parse_escape(&chars, i + 1, true, group_count, &group_names)
                                else {
                                    return false;
                                };
                                (kind, next)
                            }
                            _ => (EscapeKind::Char(chars[i + 1] as u32), i + 2),
                        };
                        let right = match kind {
                            EscapeKind::Char(value) => value,
                            _ => return false,
                        };
                        if left > right {
                            return false;
                        }
                        prev_atom = Some(EscapeKind::Char(right));
                        i = next_i;
                    } else {
                        // Literal `-` at the start or end of the class.
                        prev_atom = Some(EscapeKind::Char('-' as u32));
                        i += 1;
                    }
                }
                _ => {
                    prev_atom = Some(EscapeKind::Char(c as u32));
                    i += 1;
                }
            }
        } else {
            match c {
                '\\' => {
                    let Some((kind, next)) =
                        parse_escape(&chars, i, false, group_count, &group_names)
                    else {
                        return false;
                    };
                    quantifiable = !matches!(kind, EscapeKind::Assertion);
                    i = next;
                }
                '[' => {
                    in_class = true;
                    prev_atom = None;
                    quantifiable = false;
                    i += 1;
                }
                // A lone `]` is invalid in unicode mode.
                ']' => return false,
                '^' | '$' => {
                    // Assertions: not quantifiable.
                    quantifiable = false;
                    i += 1;
                }
                '*' | '+' | '?' => {
                    if !quantifiable {
                        return false;
                    }
                    quantifiable = false;
                    i += 1;
                    // A `?` after a quantifier makes it lazy (`a+?`).
                    if chars.get(i) == Some(&'?') {
                        i += 1;
                    }
                }
                '{' => {
                    let Some(next) = parse_quantifier(&chars, i) else {
                        // A lone `{` is invalid in unicode mode.
                        return false;
                    };
                    if !quantifiable {
                        return false;
                    }
                    quantifiable = false;
                    i = next;
                    // A `?` after a quantifier makes it lazy (`a{2}?`).
                    if chars.get(i) == Some(&'?') {
                        i += 1;
                    }
                }
                // A lone `}` is invalid in unicode mode.
                '}' => return false,
                '|' => {
                    quantifiable = false;
                    i += 1;
                }
                '(' => {
                    let Some((next, is_lookaround)) = parse_group_open(&chars, i) else {
                        return false;
                    };
                    group_stack.push(is_lookaround);
                    quantifiable = false;
                    i = next;
                }
                ')' => {
                    let Some(is_lookaround) = group_stack.pop() else {
                        // An unmatched `)` is invalid.
                        return false;
                    };
                    // Lookaround assertions cannot be quantified.
                    quantifiable = !is_lookaround;
                    i += 1;
                }
                '.' => {
                    quantifiable = true;
                    i += 1;
                }
                _ => {
                    quantifiable = true;
                    i += 1;
                }
            }
        }
    }

    !in_class && group_stack.is_empty()
}
