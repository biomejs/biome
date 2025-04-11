use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsNumberLiteralExpression, numbers::split_into_radix_and_number};
use biome_rowan::AstNode;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of numeric separators in numeric literals.
    ///
    /// Enforces a convention of grouping digits using [numeric separators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Numeric_separators).
    /// Long numbers can become difficult to read, so separating groups of digits with an underscore (`_`) improves code clarity. This rule also enforces proper usage of the numeric separator, by checking if the groups of digits are of the correct size.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub UseNumericSeparators {
        version: "next",
        name: "useNumericSeparators",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("numeric-separators-style"), RuleSource::Clippy("unreadable_literal")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNumericSeparators {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let raw = token.text_trimmed();
        let num = parse_numeric_literal(token.text_trimmed());

        let (min_digits, chunk_size) = match num.radix {
            2 => (0, 4),
            8 => (0, 4),
            10 => (5, 3),
            16 => (0, 2),
            _ => unreachable!(),
        };

        let expected = format_numeric_literal(&num, min_digits, chunk_size);

        if raw == expected {
            None
        } else {
            if raw.contains('_') {
                // Already contains separators, but not in the right places.
                Some(State::InconsistentDigitGrouping)
            } else {
                // Missing separators.
                Some(State::UnreadableLiteral)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match state {
            State::UnreadableLiteral => Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("Long numeric literal missing separators"),
            )),
            State::InconsistentDigitGrouping => Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("Inconsistent digit grouping in numeric literal"),
            )),
        }
    }

    fn action(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        // let token = ctx.query().value_token().ok()?;

        None
    }
}

pub enum State {
    InconsistentDigitGrouping,
    UnreadableLiteral,
}

/// Add chunk separators to a number string.
/// 1234567890 -> 1_234_567_890
fn add_chunk_separators(num: &str, chunk_size: usize) -> String {
    let reverse_digits = num.chars().rev().collect::<Vec<_>>();
    let chunks = reverse_digits
        // TODO: rchunks?
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>();

    chunks.join("_").chars().rev().collect()
}

struct NumericLiteral {
    sign: Option<char>,
    radix: u8,
    number: String,
    fractional: Option<String>,
}

fn parse_numeric_literal(raw: &str) -> NumericLiteral {
    let (sign, num) = if raw.starts_with('-') {
        (Some('-'), &raw[1..])
    } else if raw.starts_with('+') {
        (Some('+'), &raw[1..])
    } else {
        (None, raw)
    };
    let (radix, num) = split_into_radix_and_number(num);
    let (number, fractional) = num.split_once('.').unwrap_or((&num, ""));

    NumericLiteral {
        sign,
        radix,
        number: number.to_owned(),
        fractional: if fractional.is_empty() {
            None
        } else {
            Some(fractional.to_owned())
        },
    }
}

fn format_numeric_literal(
    numberic_literal: &NumericLiteral,
    min_digits: usize,
    chunk_size: usize,
) -> String {
    let NumericLiteral {
        sign,
        number,
        fractional,
        ..
    } = numberic_literal;

    let number = if number.len() < min_digits {
        number.to_owned()
    } else {
        add_chunk_separators(&number, chunk_size)
    };

    let fractional = if let Some(fractional) = fractional {
        if fractional.len() < min_digits {
            None
        } else {
            Some(add_chunk_separators(&fractional, chunk_size))
        }
    } else {
        None
    };

    format!(
        "{}{}{}",
        sign.unwrap_or_default(),
        number,
        fractional.map(|f| format!(".{}", f)).unwrap_or_default()
    )
}

#[test]
fn test() {
    assert_eq!(format_numeric_literal("10", 3), "10");
    assert_eq!(format_numeric_literal("100", 3), "100");
    assert_eq!(format_numeric_literal("1000", 3), "1_000");
    assert_eq!(format_numeric_literal("10000", 3), "10_000");
    assert_eq!(format_numeric_literal("100000", 3), "100_000");
    assert_eq!(format_numeric_literal("1000000", 3), "1_000_000");
    assert_eq!(format_numeric_literal("1234567890", 3), "1_234_567_890");
    assert_eq!(
        format_numeric_literal("12345678901234567890", 3),
        "12_345_678_901_234_567_890"
    );

    assert_eq!(format_numeric_literal("-10_000", 3), "-10_000");

    assert_eq!(format_numeric_literal("1_000_000_000", 3), "1_000_000_000");

    assert_eq!(format_numeric_literal("9999.99", 3), "9_999.99");
}
