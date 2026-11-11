use std::str::Chars;

use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyTsType, JsBinaryExpression,
    JsBinaryOperator, JsCallExpression, JsComputedMemberExpression,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
use biome_rule_options::use_valid_character_comparison::UseValidCharacterComparisonOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow comparing a single character with a string that has more than one character.
    ///
    /// `charAt()` returns a string that has one character at most.
    /// The same is true when you read a character from a string with `at()` or square brackets, like `text[0]`.
    /// Comparing that result with a longer string with `===`, `==`, `!==`, or `!=` always gives the same answer, so the comparison is likely a mistake.
    ///
    /// JavaScript counts some characters, such as most emojis, as two characters in a string.
    /// For example, `"😀".length` is `2`, so `text.charAt(0)` can never be equal to `"😀"`.
    ///
    /// Arrays also have `at()` and square-bracket access, and their items can be longer strings.
    /// For this reason, the rule checks `at()` and square brackets only when the value is clearly a string:
    /// a string literal, or a variable declared with the `string` type or with a `const` string literal.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// text.charAt(4) === "a2";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// text.charAt(0) !== "/n";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// "abc".at(0) === "ab";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const text = "abc";
    /// text[0] === "ab";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// text.charAt(4) === "a";
    /// text.charAt(4) === "\n";
    /// text.charAt(4) === "";
    /// text.startsWith("a2");
    /// ```
    ///
    pub UseValidCharacterComparison {
        version: "2.6.7",
        name: "useValidCharacterComparison",
        language: "js",
        sources: &[RuleSource::Oxc("bad-char-at-comparison").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseValidCharacterComparison {
    type Query = Semantic<JsBinaryExpression>;
    /// The operand of the comparison that holds the string with more than one character.
    type State = Operand;
    type Signals = Option<Self::State>;
    type Options = UseValidCharacterComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let comparison = ctx.query();
        if !matches!(
            comparison.operator().ok()?,
            JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality
        ) {
            return None;
        }

        let left = comparison.left().ok()?.omit_parentheses();
        let right = comparison.right().ok()?.omit_parentheses();
        let model = ctx.model();
        if is_multi_character_string(&right) && is_single_character_access(&left, model) {
            Some(Operand::Right)
        } else if is_multi_character_string(&left) && is_single_character_access(&right, model) {
            Some(Operand::Left)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, operand: &Self::State) -> Option<RuleDiagnostic> {
        let comparison = ctx.query();
        let left = comparison.left().ok()?.omit_parentheses();
        let right = comparison.right().ok()?.omit_parentheses();
        let (access, string) = match operand {
            Operand::Left => (right, left),
            Operand::Right => (left, right),
        };
        let is_equality = matches!(
            comparison.operator().ok()?,
            JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality
        );

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                comparison.range(),
                if is_equality {
                    markup! { "This comparison is always "<Emphasis>"false"</Emphasis>"." }
                } else {
                    markup! { "This comparison is always "<Emphasis>"true"</Emphasis>"." }
                },
            )
            .detail(
                access.range(),
                markup! { "This expression returns one character at most." },
            )
            .detail(
                string.range(),
                markup! { "But this string has more than one character." },
            )
            .note(markup! {
                "Compare the character with a string of one character, or use a method such as "<Emphasis>"startsWith()"</Emphasis>" to check for longer text."
            }),
        )
    }
}

#[derive(Clone, Copy)]
pub enum Operand {
    Left,
    Right,
}

/// Returns `true` if `expression` is a string literal, or a template literal
/// without substitutions, whose value is longer than one UTF-16 code unit.
fn is_multi_character_string(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => string
            .inner_string_text()
            .is_ok_and(|text| is_longer_than_one_code_unit(text.text())),
        AnyJsExpression::JsTemplateExpression(template) => {
            let elements = template.elements();
            template.tag().is_none()
                && elements.len() == 1
                && elements
                    .first()
                    .and_then(|element| {
                        element
                            .as_js_template_chunk_element()?
                            .template_chunk_token()
                            .ok()
                    })
                    .is_some_and(|token| is_longer_than_one_code_unit(token.text_trimmed()))
        }
        _ => false,
    }
}

/// Returns `true` if `expression` returns a string of one character at most:
/// `x.charAt(i)`, or `x.at(i)` and `x[i]` where `x` is known to be a string.
fn is_single_character_access(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expression {
        AnyJsExpression::JsCallExpression(call) => is_character_method_call(call, model),
        AnyJsExpression::JsComputedMemberExpression(member) => {
            is_string_index_access(member, model)
        }
        _ => false,
    }
}

/// Returns `true` for `x.charAt(i)`, and for `x.at(i)` where `x` is known to be a string.
fn is_character_method_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    if call.is_optional_chain()
        || !call.arguments().is_ok_and(|arguments| {
            let arguments = arguments.args();
            arguments.len() == 1
                && matches!(
                    arguments.first(),
                    Some(Ok(AnyJsCallArgument::AnyJsExpression(_)))
                )
        })
    {
        return false;
    }
    let Some(member) = call
        .callee()
        .ok()
        .map(AnyJsExpression::omit_parentheses)
        .and_then(|callee| callee.as_js_static_member_expression().cloned())
    else {
        return false;
    };
    let Some(name) = member
        .member()
        .ok()
        .and_then(|name| name.as_js_name()?.value_token().ok())
    else {
        return false;
    };
    match name.text_trimmed() {
        "charAt" => true,
        "at" => member
            .object()
            .is_ok_and(|object| is_definitely_string(&object.omit_parentheses(), model)),
        _ => false,
    }
}

/// Returns `true` for `x[i]` where `x` is known to be a string and `i` is a
/// number literal or a string literal that is a valid array index, such as `"0"`.
fn is_string_index_access(member: &JsComputedMemberExpression, model: &SemanticModel) -> bool {
    if member.is_optional_chain() {
        return false;
    }
    let is_index = match member.member().map(AnyJsExpression::omit_parentheses) {
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(_),
        )) => true,
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        )) => string
            .inner_string_text()
            .is_ok_and(|text| is_canonical_index(text.text())),
        _ => false,
    };
    is_index
        && member
            .object()
            .is_ok_and(|object| is_definitely_string(&object.omit_parentheses(), model))
}

/// Returns `true` if `expression` is a string literal, or a variable declared
/// with the `string` type or with a `const` string literal.
fn is_definitely_string(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let AnyJsExpression::JsIdentifierExpression(identifier) = expression else {
        return is_string_literal(expression);
    };
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) = identifier
        .name()
        .ok()
        .and_then(|reference| model.binding(&reference))
        .and_then(|binding| binding.tree().declaration())
    else {
        return false;
    };
    let has_string_annotation = declarator
        .variable_annotation()
        .and_then(|annotation| annotation.type_annotation().ok()??.ty().ok())
        .is_some_and(|ty| matches!(ty, AnyTsType::TsStringType(_)));
    has_string_annotation
        || (declarator
            .declaration()
            .is_some_and(|declaration| declaration.is_const())
            && declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
                .is_some_and(|initializer| is_string_literal(&initializer.omit_parentheses())))
}

fn is_string_literal(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            _
        ))
    )
}

/// Returns `true` if `text` is the canonical form of a non-negative integer
/// that is at most `Number.MAX_SAFE_INTEGER`, such as `"0"` or `"12"`, but not
/// `"012"` or `"1.0"`.
fn is_canonical_index(text: &str) -> bool {
    let bytes = text.as_bytes();
    match bytes {
        [] => false,
        [b'0'] => true,
        [b'0', ..] => false,
        _ => {
            bytes
                .iter()
                .all(|byte| matches!(lookup_byte(*byte), Dispatch::ZER | Dispatch::DIG))
                && text
                    .parse::<u64>()
                    .is_ok_and(|index| index <= MAX_SAFE_INTEGER)
        }
    }
}

/// The value of `Number.MAX_SAFE_INTEGER` in JavaScript.
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Returns `true` if the string written as `raw` is longer than one UTF-16
/// code unit once its escape sequences are processed.
///
/// `raw` is the source text between the quotes or backticks of a string
/// literal or template chunk. A line continuation adds no code unit, and a
/// `\r\n` line break in a template chunk becomes a single `\n`.
fn is_longer_than_one_code_unit(raw: &str) -> bool {
    // `unescape_js_string` isn't used because it allocates a new string and
    // doesn't support legacy octal escapes such as `\12`.
    let mut length = 0;
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        length += match c {
            '\\' => match chars.next() {
                Some('\r') => {
                    if chars.as_str().starts_with('\n') {
                        chars.next();
                    }
                    0
                }
                Some('\n' | '\u{2028}' | '\u{2029}') => 0,
                Some('u') => match chars.as_str().strip_prefix('{') {
                    Some(rest) => {
                        let (digits, rest) = rest.split_once('}').unwrap_or((rest, ""));
                        chars = rest.chars();
                        if u32::from_str_radix(digits, 16)
                            .is_ok_and(|code_point| code_point > 0xFFFF)
                        {
                            2
                        } else {
                            1
                        }
                    }
                    None => {
                        skip_hex_digits(&mut chars, 4);
                        1
                    }
                },
                Some('x') => {
                    skip_hex_digits(&mut chars, 2);
                    1
                }
                Some(first @ '0'..='7') => {
                    // A legacy octal escape has up to three digits, and its
                    // value can't be greater than `\377`.
                    let max_digits = if first <= '3' { 2 } else { 1 };
                    for _ in 0..max_digits {
                        if !chars
                            .as_str()
                            .starts_with(|c: char| ('0'..='7').contains(&c))
                        {
                            break;
                        }
                        chars.next();
                    }
                    1
                }
                Some(escaped) => escaped.len_utf16(),
                None => 0,
            },
            '\r' => {
                if chars.as_str().starts_with('\n') {
                    chars.next();
                }
                1
            }
            c => c.len_utf16(),
        };
        if length > 1 {
            return true;
        }
    }
    false
}

/// Advances `chars` past the next `count` characters if they're all hexadecimal digits.
///
/// An escape such as `\ukeff` is invalid, so its characters are left in place
/// and counted as text.
fn skip_hex_digits(chars: &mut Chars, count: usize) {
    let is_escape = chars
        .as_str()
        .as_bytes()
        .get(..count)
        .is_some_and(|digits| digits.iter().all(u8::is_ascii_hexdigit));
    if is_escape {
        *chars = chars.as_str()[count..].chars();
    }
}

#[cfg(test)]
mod tests {
    use super::is_longer_than_one_code_unit;

    #[test]
    fn counts_code_units_after_escapes() {
        let short = [
            "",
            "a",
            "\\n",
            "\\t",
            "\\\\",
            "\\x41",
            "\\u0041",
            "\\u{41}",
            "\\u{FFFF}",
            "\\0",
            "\\12",
            "\\377",
            "\\8",
            "\u{feff}",
            "a\\\n",
            "\\\r\n",
            "\r\n",
        ];
        for raw in short {
            assert!(!is_longer_than_one_code_unit(raw), "{raw:?} is short");
        }

        let long = [
            "ab",
            "/n",
            "\\n\\n",
            "\\x41b",
            "\\u0041b",
            "\\u{1F600}",
            "😀",
            "\\uD83D\\uDE00",
            "\\08",
            "\\400",
            "\\ukeff",
        ];
        for raw in long {
            assert!(is_longer_than_one_code_unit(raw), "{raw:?} is long");
        }
    }
}
