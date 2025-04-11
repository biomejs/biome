use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    JsNumberLiteralExpression, JsSyntaxToken, numbers::split_into_radix_and_number,
};
use biome_rowan::{AstNode, BatchMutationExt};
use serde::{Deserialize, Serialize};

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
    /// var a = 1234567890;
    /// a = -999_99;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1_234_567_890;
    /// a = -99_999;
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
    type Options = UseNumericSeparatorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let raw = token.text_trimmed();

        let num = NumericLiteral::parse(raw);
        let expected = num.format(ctx.options().clone());

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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = ctx.query().value_token().ok()?;
        let num = NumericLiteral::parse(token.text_trimmed());

        let new_token =
            JsSyntaxToken::new_detached(token.kind(), &num.format(ctx.options().clone()), [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                State::UnreadableLiteral => markup! { "Add numeric separators" },
                State::InconsistentDigitGrouping => markup! { "Fix numeric separator grouping" },
            },
            mutation,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Deserializable, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNumericSeparatorsOptions {
    /// Binary numeric literal options.
    #[serde(default)]
    pub binary: BaseOptions,
    /// Octal numeric literal options.
    #[serde(default)]
    pub octal: BaseOptions,
    /// Decimal numeric literal options.
    #[serde(default = "decimal_default")]
    pub decimal: BaseOptions,
    /// Hexadecimal numeric literal options.
    #[serde(default = "hexadecimal_default")]
    pub hexadecimal: BaseOptions,
}

impl Default for UseNumericSeparatorsOptions {
    fn default() -> Self {
        Self {
            binary: BaseOptions::default(),
            octal: BaseOptions::default(),
            decimal: decimal_default(),
            hexadecimal: hexadecimal_default(),
        }
    }
}

fn decimal_default() -> BaseOptions {
    BaseOptions {
        min_digits: 5,
        chunk_size: 3,
    }
}

fn hexadecimal_default() -> BaseOptions {
    BaseOptions {
        min_digits: 0,
        chunk_size: 2,
    }
}

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BaseOptions {
    /// The minimum number of digits before a separator is required.
    pub min_digits: usize,
    /// The size of each chunk.
    pub chunk_size: usize,
}

impl Default for BaseOptions {
    fn default() -> Self {
        Self {
            min_digits: 0,
            chunk_size: 4,
        }
    }
}

pub enum State {
    InconsistentDigitGrouping,
    UnreadableLiteral,
}

/// Add chunk separators to a number string, starting from the right.
/// The "uneven" chunk is added to the left of the first separator.
/// 1234567890 -> 1_234_567_890
fn add_chunk_separators_from_right(num: &str, chunk_size: usize) -> String {
    num.chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .rev()
        .collect()
}

/// Add chunk separators to a number string, starting from the left. Used for fractional parts.
/// The "uneven" chunk is added to the right of the last separator.
/// 12345654321 -> 123_456_543_21
fn add_chunk_separators_from_left(num: &str, chunk_size: usize) -> String {
    num.chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("_")
}

#[derive(Debug)]
struct NumericLiteral {
    sign: Option<char>,
    radix: u8,
    number: String,
    fractional: Option<String>,
}

impl NumericLiteral {
    fn parse(raw: &str) -> Self {
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

    fn format(&self, options: UseNumericSeparatorsOptions) -> String {
        let NumericLiteral {
            sign,
            radix,
            number,
            fractional,
        } = self;

        let BaseOptions {
            min_digits,
            chunk_size,
        } = match radix {
            2 => options.binary,
            8 => options.octal,
            10 => options.decimal,
            16 => options.hexadecimal,
            _ => unreachable!(),
        };

        let number = if number.len() < min_digits {
            number.to_owned()
        } else {
            add_chunk_separators_from_right(&number, chunk_size)
        };

        let fractional = if let Some(fractional) = fractional {
            if fractional.len() < min_digits {
                Some(fractional.to_owned())
            } else {
                Some(add_chunk_separators_from_left(&fractional, chunk_size))
            }
        } else {
            None
        };

        format!(
            "{}{}{}",
            if let Some(sign) = sign {
                sign.to_string()
            } else {
                String::new()
            },
            number,
            fractional.map(|f| format!(".{}", f)).unwrap_or_default()
        )
    }
}

#[test]
fn test() {
    fn format_numeric_literal(num: &str) -> String {
        let options = UseNumericSeparatorsOptions::default();
        let num = NumericLiteral::parse(num);
        num.format(options)
    }

    // Decimals with less than 5 digits are not formatted with separators with the default options.
    assert_eq!(format_numeric_literal("10"), "10");
    assert_eq!(format_numeric_literal("100"), "100");
    assert_eq!(format_numeric_literal("1000"), "1000");
    // Decimals with 5 digits or more are formatted with separators.
    assert_eq!(format_numeric_literal("10000"), "10_000");
    assert_eq!(format_numeric_literal("100000"), "100_000");
    assert_eq!(format_numeric_literal("1000000"), "1_000_000");
    assert_eq!(format_numeric_literal("1234567890"), "1_234_567_890");
    assert_eq!(
        format_numeric_literal("12345678901234567890"),
        "12_345_678_901_234_567_890"
    );

    assert_eq!(format_numeric_literal("-10_000"), "-10_000");

    assert_eq!(format_numeric_literal("1_000_000_000"), "1_000_000_000");

    assert_eq!(format_numeric_literal("99999.99"), "99_999.99");

    assert_eq!(format_numeric_literal("1.23456789"), "1.234_567_89");
}
