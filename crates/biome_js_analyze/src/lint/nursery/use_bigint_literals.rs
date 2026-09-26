use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsOptionalChainExpression,
    JsArrayElementList, JsArrowFunctionExpression, JsCallArgumentList, JsCallExpression,
    JsComputedMemberExpression, JsExpressionStatement, JsForInStatement, JsForOfStatement,
    JsForStatement, JsIfStatement, JsInitializerClause, JsLanguage, JsParenthesizedExpression,
    JsPropertyObjectMember, JsReturnStatement, JsSyntaxKind, JsSyntaxToken, JsThrowStatement,
    JsUnaryOperator, JsWhileStatement, JsWithStatement, T, global_identifier,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet, TextRange, TextSize,
    TokenText,
};
use biome_rule_options::use_bigint_literals::UseBigintLiteralsOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce the use of bigint literals over the `BigInt()` constructor.
    ///
    /// A bigint literal with the `n` suffix is more concise than calling `BigInt()`
    /// with a number or string literal, and it is evaluated at parse time rather than
    /// converted at runtime.
    ///
    /// The rule only reports calls whose argument is a number or string literal that
    /// can be expressed as a bigint literal. Calls with a computed argument are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const bigint = BigInt(1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const large = BigInt("9007199254740993");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const hex = BigInt("0xFF");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const negative = BigInt(-1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const bigint = 1n;
    /// const large = 9007199254740993n;
    /// const hex = 0xFFn;
    /// const computed = BigInt(getSomeNumber());
    /// const fraction = BigInt(1.5);
    /// ```
    ///
    pub UseBigintLiterals {
        version: "next",
        name: "useBigintLiterals",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-bigint-literals").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseBigintLiterals {
    type Query = Semantic<JsCallExpression>;
    type State = BigintLiteral;
    type Signals = Option<Self::State>;
    type Options = UseBigintLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional() {
            return None;
        }

        let callee = call.callee().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&callee.as_any_global_identifier_expression()?)?;
        if name.text() != "BigInt" || ctx.model().binding(&reference).is_some() {
            return None;
        }

        // Parentheses end short-circuiting, but removing an optional callee
        // would still change its result or suppress an exception.
        let mut object = callee;
        while let Some(member) = AnyJsOptionalChainExpression::cast_ref(object.syntax()) {
            if member.is_optional() {
                return None;
            }
            // A template tag executes code; its raw text doesn't identify a global.
            if let AnyJsOptionalChainExpression::JsComputedMemberExpression(computed) = &member
                && let AnyJsExpression::JsTemplateExpression(template) =
                    computed.member().ok()?.omit_parentheses()
                && template.tag().is_some()
            {
                return None;
            }
            object = member.object().ok()?.omit_parentheses();
        }

        let arguments = call.arguments().ok()?.args();
        if arguments.len() != 1 {
            return None;
        }
        let AnyJsCallArgument::AnyJsExpression(argument) = arguments.first()?.ok()? else {
            return None;
        };

        BigintLiteral::from_argument(argument)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"BigInt()"</Emphasis>" call can be replaced with a bigint literal."
                },
            )
            .note(markup! {
                "A bigint literal with the "<Emphasis>"n"</Emphasis>" suffix is more concise, and it is evaluated at parse time instead of being converted at runtime."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        // Keep number notation unchanged and preserve comments inside the call.
        if state
            .number_value
            .is_some_and(|value| !is_exact_number_literal(state.magnitude.text(), value))
            || has_comments_inside(call)
        {
            return None;
        }

        let magnitude = if state.magnitude.is_empty() {
            "0"
        } else {
            state.magnitude.text()
        };
        let literal_text = format!("{magnitude}n");
        let literal = make::js_bigint_literal_expression(JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_BIGINT_LITERAL,
            &literal_text,
            [],
            [],
        ));
        let mut replacement = AnyJsExpression::AnyJsLiteralExpression(literal.into());

        if state.is_negative {
            // The replacement starts with `-` or `(`, both of which would continue
            // the previous statement when no semicolon separates them.
            if starts_unterminated_statement(call) {
                return None;
            }
            replacement = make::js_unary_expression(make::token(T![-]), replacement).into();
            if needs_parentheses(call) {
                replacement = make::parenthesized(replacement).into();
            }
        }

        // Parentheses can separate a call from a keyword without whitespace.
        // Replacing the call must not join tokens such as `return1n` or `1nin`.
        let first = call.syntax().first_token()?;
        if !state.is_negative
            && first.prev_token().is_some_and(|previous| {
                previous.kind().is_keyword()
                    && previous.text_trimmed_range().end() == first.text_trimmed_range().start()
            })
        {
            replacement = replacement.with_leading_trivia_pieces(
                make::token_decorated_with_space(T![=])
                    .leading_trivia()
                    .pieces(),
            )?;
        }
        let last = call.syntax().last_token()?;
        if replacement.syntax().last_token()?.kind() == JsSyntaxKind::JS_BIGINT_LITERAL
            && last.next_token().is_some_and(|next| {
                (next.kind().is_keyword() || next.kind() == JsSyntaxKind::IDENT)
                    && last.text_trimmed_range().end() == next.text_trimmed_range().start()
            })
        {
            replacement = replacement.with_trailing_trivia_pieces(
                make::token_decorated_with_space(T![=])
                    .trailing_trivia()
                    .pieces(),
            )?;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(AnyJsExpression::from(call.clone()), replacement)?;

        let sign = if state.is_negative { "-" } else { "" };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>{sign}{literal_text}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// The bigint literal that replaces a `BigInt()` call.
pub struct BigintLiteral {
    /// The unsigned literal without the `n` suffix. An empty slice represents zero.
    magnitude: TokenText,
    is_negative: bool,
    /// Present for numeric arguments, whose spelling must preserve this value in a fix.
    number_value: Option<f64>,
}

impl BigintLiteral {
    fn from_argument(argument: AnyJsExpression) -> Option<Self> {
        let mut argument = argument.omit_parentheses();
        let mut is_negative = false;
        let mut has_unary_operator = false;
        while let AnyJsExpression::JsUnaryExpression(unary) = &argument {
            match unary.operator().ok()? {
                JsUnaryOperator::Plus => {}
                JsUnaryOperator::Minus => is_negative = !is_negative,
                _ => return None,
            }
            has_unary_operator = true;
            argument = unary.argument().ok()?.omit_parentheses();
        }

        match argument {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(number),
            ) => {
                let token = number.value_token().ok()?;
                let value = number.as_number()?;
                Self::from_number(token.token_text_trimmed(), value, is_negative)
            }
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string),
            ) if !has_unary_operator => {
                let text = string.inner_string_text().ok()?;
                Self::from_string(text)
            }
            _ => None,
        }
    }

    /// Converts a number literal, following the semantics of `BigInt(number)`.
    fn from_number(raw: TokenText, value: f64, is_negative: bool) -> Option<Self> {
        if !value.is_finite() || value.fract() != 0.0 {
            return None;
        }
        let is_negative = is_negative && value != 0.0;
        Some(Self {
            magnitude: raw,
            is_negative,
            number_value: Some(value),
        })
    }

    /// Converts the raw content of a string literal, following the semantics of
    /// `BigInt(string)`. Returns `None` when the conversion would throw.
    fn from_string(raw: TokenText) -> Option<Self> {
        // Escape sequences would need to be decoded first.
        if raw.contains('\\') {
            return None;
        }
        let start = raw.len() - TextSize::of(raw.trim_start_matches(is_js_whitespace));
        let len = TextSize::of(raw.trim_matches(is_js_whitespace));
        let text = raw.slice(TextRange::at(start, len));
        if text.is_empty() {
            return Some(Self::zero(text));
        }

        let radix_digits = text
            .strip_prefix("0b")
            .or_else(|| text.strip_prefix("0B"))
            .map(|digits| (2, digits))
            .or_else(|| {
                text.strip_prefix("0o")
                    .or_else(|| text.strip_prefix("0O"))
                    .map(|digits| (8, digits))
            })
            .or_else(|| {
                text.strip_prefix("0x")
                    .or_else(|| text.strip_prefix("0X"))
                    .map(|digits| (16, digits))
            });
        if let Some((radix, digits)) = radix_digits {
            if digits.is_empty() || !digits.bytes().all(|byte| is_radix_digit(byte, radix)) {
                return None;
            }
            if digits.bytes().all(|b| b == b'0') {
                return Some(Self::zero(text));
            }
            return Some(Self {
                magnitude: text,
                is_negative: false,
                number_value: None,
            });
        }

        let (is_negative, digits) = match text.as_bytes()[0] {
            b'-' => (true, &text[1..]),
            b'+' => (false, &text[1..]),
            _ => (false, text.text()),
        };
        if digits.is_empty() || !digits.bytes().all(|byte| is_radix_digit(byte, 10)) {
            return None;
        }
        // Decimal bigint literals can't have leading zeroes.
        let digits = digits.trim_start_matches('0');
        if digits.is_empty() {
            return Some(Self::zero(text));
        }
        let len = TextSize::of(digits);
        let range = TextRange::at(text.len() - len, len);
        Some(Self {
            magnitude: text.slice(range),
            is_negative,
            number_value: None,
        })
    }

    fn zero(raw: TokenText) -> Self {
        Self {
            magnitude: raw.slice(TextRange::default()),
            is_negative: false,
            number_value: None,
        }
    }
}

fn is_radix_digit(byte: u8, radix: u32) -> bool {
    match lookup_byte(byte) {
        Dispatch::ZER => true,
        Dispatch::DIG => u32::from(byte - b'0') < radix,
        Dispatch::IDT if radix == 16 => (b'a'..=b'f').contains(&byte.to_ascii_lowercase()),
        _ => false,
    }
}

/// Returns `true` if the raw number literal can be reused as a bigint literal
/// by appending the `n` suffix, without changing the value.
///
/// The literal has to be an integer without a fraction or an exponent, and its
/// value must be exactly representable as a number, since `BigInt(number)`
/// operates on the already rounded value.
fn is_exact_number_literal(raw: &str, value: f64) -> bool {
    let (prefix, digits) = raw.split_at(raw.len().min(2));
    let bits_per_digit = match prefix {
        "0b" | "0B" => 1,
        "0o" | "0O" => 3,
        "0x" | "0X" => 4,
        _ => {
            if raw.bytes().any(|b| matches!(b, b'.' | b'e' | b'E')) {
                return false;
            }
            // Legacy octal literals and literals with leading zeroes aren't valid
            // bigint literals. Rounding is detected by comparing with the exact value.
            let value = format!("{value:.0}");
            return raw.bytes().filter(|&b| b != b'_').eq(value.bytes());
        }
    };

    // The value is exact if its significant bits fit in the 53-bit mantissa.
    let significant = digits.trim_matches(|c| c == '0' || c == '_');
    let Some((first, last)) = significant
        .chars()
        .next()
        .zip(significant.chars().next_back())
    else {
        return true;
    };
    let (Some(first), Some(last)) = (first.to_digit(16), last.to_digit(16)) else {
        return false;
    };
    let digit_count = significant.bytes().filter(|&b| b != b'_').count() as u32;
    let bit_length = (digit_count - 1) * bits_per_digit + (u32::BITS - first.leading_zeros());
    bit_length - last.trailing_zeros() <= f64::MANTISSA_DIGITS
}

/// Whitespace trimmed by `BigInt(string)`: `WhiteSpace` and `LineTerminator`.
fn is_js_whitespace(c: char) -> bool {
    matches!(
        c,
        '\t' | '\u{b}' | '\u{c}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
                | '\n'
                | '\r'
                | '\u{2028}'
                | '\u{2029}'
    )
}

const UNPARENTHESIZED_NEGATIVE_LITERAL_PARENTS: SyntaxKindSet<JsLanguage> =
    JsParenthesizedExpression::KIND_SET
        .union(JsExpressionStatement::KIND_SET)
        .union(JsInitializerClause::KIND_SET)
        .union(JsCallArgumentList::KIND_SET)
        .union(JsArrayElementList::KIND_SET)
        .union(JsPropertyObjectMember::KIND_SET)
        .union(JsReturnStatement::KIND_SET)
        .union(JsThrowStatement::KIND_SET)
        .union(JsArrowFunctionExpression::KIND_SET);

/// Returns `true` if a negative literal replacing `call` must be parenthesized.
fn needs_parentheses(call: &JsCallExpression) -> bool {
    let Some(parent) = call.syntax().parent() else {
        return false;
    };
    if let Some(member) = JsComputedMemberExpression::cast_ref(&parent) {
        return member
            .member()
            .is_ok_and(|member| member.syntax() != call.syntax());
    }
    !UNPARENTHESIZED_NEGATIVE_LITERAL_PARENTS.matches(parent.kind())
}

/// Returns `true` if `call` is the first token of an expression statement that
/// isn't separated from the previous statement by a semicolon or another token
/// that can't be continued by an expression.
fn starts_unterminated_statement(call: &JsCallExpression) -> bool {
    let Some(first_token) = call.syntax().first_token() else {
        return false;
    };
    let is_statement_start = call
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().first_token())
        .is_some_and(|token| token == first_token);
    if !is_statement_start {
        return false;
    }
    let Some(previous) = first_token.prev_token() else {
        return false;
    };
    !terminates_statement(&previous)
}

const STATEMENT_TERMINATORS: TokenSet<JsSyntaxKind> =
    token_set![T![;], T!['{'], T![:], T![else], T![do]];

const CONTROL_FLOW_STATEMENT_KINDS: SyntaxKindSet<JsLanguage> = JsIfStatement::KIND_SET
    .union(JsWhileStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsWithStatement::KIND_SET);

/// Returns `true` if `token` can't be continued by a following `(` or `-`.
fn terminates_statement(token: &JsSyntaxToken) -> bool {
    if STATEMENT_TERMINATORS.contains(token.kind()) {
        return true;
    }
    match token.kind() {
        T![')'] => token
            .parent()
            .is_some_and(|parent| CONTROL_FLOW_STATEMENT_KINDS.matches(parent.kind())),
        T!['}'] => {
            let Some(mut parent) = token.parent() else {
                return false;
            };
            if parent.kind() == JsSyntaxKind::JS_FUNCTION_BODY
                && let Some(grand_parent) = parent.parent()
            {
                parent = grand_parent;
            }
            !AnyJsExpression::can_cast(parent.kind())
        }
        _ => false,
    }
}

/// Returns `true` if the call contains comments that would be lost by replacing it.
fn has_comments_inside(call: &JsCallExpression) -> bool {
    let syntax = call.syntax();
    let first = syntax.first_token();
    let last = syntax.last_token();
    syntax.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() && first.as_ref() != Some(&token))
            || (token.has_trailing_comments() && last.as_ref() != Some(&token))
    })
}

#[cfg(test)]
mod tests {
    use super::is_exact_number_literal;

    #[test]
    fn exact_number_literals() {
        for (raw, value) in [
            ("0", 0.0),
            ("1", 1.0),
            ("9007199254740992", 9_007_199_254_740_992.0),
            ("9_007_199_254_740_992", 9_007_199_254_740_992.0),
            ("0x20000000000000", 9_007_199_254_740_992.0),
            ("0x1FFFFFFFFFFFFF", 9_007_199_254_740_991.0),
            ("0XFe_fE", 65278.0),
            ("0b11_11", 15.0),
            ("0o777", 511.0),
            ("0x0", 0.0),
            ("0b0000", 0.0),
        ] {
            assert!(is_exact_number_literal(raw, value), "{raw}");
        }
    }

    #[test]
    fn inexact_number_literals() {
        for (raw, value) in [
            ("9007199254740993", 9_007_199_254_740_992.0),
            ("9_007_199_254_740_993", 9_007_199_254_740_992.0),
            ("0x20000000000001", 9_007_199_254_740_992.0),
            ("0x3FFFFFFFFFFFFF", 18_014_398_509_481_984.0),
            ("0777", 511.0),
            ("0888", 888.0),
            ("00", 0.0),
            ("1.0", 1.0),
            ("1e2", 100.0),
            ("1e21", 1e21),
        ] {
            assert!(!is_exact_number_literal(raw, value), "{raw}");
        }
    }
}
