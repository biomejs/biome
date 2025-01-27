use crate::JsRuleAction;

use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsObjectMember, JsLiteralMemberName, JsObjectExpression, JsSyntaxKind, JsSyntaxToken,
    TextRange,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult};
use std::str::FromStr;

declare_lint_rule! {
    /// Disallow number literal object member names which are not base10 or uses underscore as separator
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// ({ 0x1: 1 });
    /// ```
    /// ```js,expect_diagnostic
    /// ({ 11_1.11: "ee" });
    /// ```
    /// ```js,expect_diagnostic
    /// ({ 0o1: 1 });
    /// ```
    /// ```js,expect_diagnostic
    /// ({ 1n: 1 });
    /// ```
    /// ```js,expect_diagnostic
    /// ({ 11_1.11: "ee" });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// ({ 0: "zero" });
    /// ({ 122: "integer" });
    /// ({ 1.22: "floating point" });
    /// ({ 3.1e12: "floating point with e" });
    /// ```
    ///
    pub UseSimpleNumberKeys {
        version: "1.0.0",
        name: "useSimpleNumberKeys",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Clone)]
pub enum NumberLiteral {
    Binary {
        node: JsLiteralMemberName,
        value: Box<str>,
        big_int: bool,
    },
    Decimal {
        node: JsLiteralMemberName,
        value: Box<str>,
        big_int: bool,
        underscore: bool,
    },
    Octal {
        node: JsLiteralMemberName,
        value: Box<str>,
        big_int: bool,
    },
    Hexadecimal {
        node: JsLiteralMemberName,
        value: Box<str>,
        big_int: bool,
    },
    FloatingPoint {
        node: JsLiteralMemberName,
        value: Box<str>,
        exponent: bool,
        underscore: bool,
    },
}

pub struct NumberLiteralError;

impl TryFrom<AnyJsObjectMember> for NumberLiteral {
    type Error = NumberLiteralError;

    fn try_from(any_member: AnyJsObjectMember) -> Result<Self, Self::Error> {
        let Some(literal_member_name) = any_member
            .syntax()
            .children()
            .find_map(JsLiteralMemberName::cast)
        else {
            return Err(NumberLiteralError);
        };
        let Ok(token) = literal_member_name.value() else {
            return Err(NumberLiteralError);
        };
        match token.kind() {
            JsSyntaxKind::JS_NUMBER_LITERAL | JsSyntaxKind::JS_BIGINT_LITERAL => {
                let text = token.text_trimmed();
                let mut value = String::new();

                let mut is_first_char_zero: bool = false;
                let mut is_second_char_a_letter: Option<u8> = None;
                let mut contains_dot: bool = false;
                let mut exponent: bool = false;
                let mut largest_digit: u8 = b'0';
                let mut underscore: bool = false;
                let mut big_int: bool = false;

                for (i, b) in text.bytes().enumerate() {
                    match b {
                        b'0' if i == 0 && text.len() > 1 => {
                            is_first_char_zero = true;
                            continue;
                        }
                        b'n' => {
                            big_int = true;
                            break;
                        }
                        b'e' | b'E' => {
                            exponent = true;
                        }
                        b'_' => {
                            underscore = true;
                            continue;
                        }
                        b'.' => {
                            contains_dot = true;
                        }
                        b if i == 1 && b.is_ascii_alphabetic() => {
                            is_second_char_a_letter = Some(b);
                            continue;
                        }
                        _ => {
                            if largest_digit < b {
                                largest_digit = b;
                            }
                        }
                    }
                    value.push(b as char);
                }

                if contains_dot {
                    return Ok(Self::FloatingPoint {
                        node: literal_member_name,
                        value: value.into_boxed_str(),
                        exponent,
                        underscore,
                    });
                };
                if !is_first_char_zero {
                    return Ok(Self::Decimal {
                        node: literal_member_name,
                        value: value.into_boxed_str(),
                        big_int,
                        underscore,
                    });
                };

                match is_second_char_a_letter {
                    Some(b'b' | b'B') => {
                        return Ok(Self::Binary {
                            node: literal_member_name,
                            value: value.into_boxed_str(),
                            big_int,
                        })
                    }
                    Some(b'o' | b'O') => {
                        return Ok(Self::Octal {
                            node: literal_member_name,
                            value: value.into_boxed_str(),
                            big_int,
                        })
                    }
                    Some(b'x' | b'X') => {
                        return Ok(Self::Hexadecimal {
                            node: literal_member_name,
                            value: value.into_boxed_str(),
                            big_int,
                        })
                    }
                    _ => (),
                }

                if largest_digit < b'8' {
                    return Ok(Self::Octal {
                        node: literal_member_name,
                        value: value.into_boxed_str(),
                        big_int,
                    });
                }

                Ok(Self::Decimal {
                    node: literal_member_name,
                    value: value.into_boxed_str(),
                    big_int,
                    underscore,
                })
            }
            _ => Err(NumberLiteralError),
        }
    }
}

impl NumberLiteral {
    fn token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::Decimal { node, .. } => node.value(),
            Self::Binary { node, .. } => node.value(),
            Self::FloatingPoint { node, .. } => node.value(),
            Self::Octal { node, .. } => node.value(),
            Self::Hexadecimal { node, .. } => node.value(),
        }
    }

    fn range(&self) -> TextRange {
        match self {
            Self::Decimal { node, .. } => node.range(),
            Self::Binary { node, .. } => node.range(),
            Self::FloatingPoint { node, .. } => node.range(),
            Self::Octal { node, .. } => node.range(),
            Self::Hexadecimal { node, .. } => node.range(),
        }
    }

    fn value(&self) -> &str {
        match self {
            Self::Decimal { value, .. } => value.as_ref(),
            Self::Binary { value, .. } => value.as_ref(),
            Self::FloatingPoint { value, .. } => value.as_ref(),
            Self::Octal { value, .. } => value.as_ref(),
            Self::Hexadecimal { value, .. } => value.as_ref(),
        }
    }
}

impl NumberLiteral {
    fn to_base_ten(&self) -> Option<f64> {
        match self {
            Self::Binary { value, .. } => i64::from_str_radix(value, 2).map(|num| num as f64).ok(),
            Self::Decimal { value, .. } | Self::FloatingPoint { value, .. } => {
                f64::from_str(value).ok()
            }
            Self::Octal { value, .. } => i64::from_str_radix(value, 7).map(|num| num as f64).ok(),
            Self::Hexadecimal { value, .. } => {
                i64::from_str_radix(value, 16).map(|num| num as f64).ok()
            }
        }
    }
}

enum WrongNumberLiteralName {
    Binary,
    Hexadecimal,
    Octal,
    BigInt,
    WithUnderscore,
}
pub struct RuleState(WrongNumberLiteralName, NumberLiteral);

impl Rule for UseSimpleNumberKeys {
    type Query = Ast<JsObjectExpression>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut result = Vec::new();
        let node = ctx.query();
        for number_literal in node
            .members()
            .into_iter()
            .flatten()
            .filter_map(|member| NumberLiteral::try_from(member).ok())
        {
            match number_literal {
                NumberLiteral::Decimal { big_int: true, .. } => {
                    result.push(RuleState(WrongNumberLiteralName::BigInt, number_literal))
                }
                NumberLiteral::FloatingPoint {
                    underscore: true, ..
                }
                | NumberLiteral::Decimal {
                    underscore: true, ..
                } => result.push(RuleState(
                    WrongNumberLiteralName::WithUnderscore,
                    number_literal,
                )),
                NumberLiteral::Binary { .. } => {
                    result.push(RuleState(WrongNumberLiteralName::Binary, number_literal))
                }
                NumberLiteral::Hexadecimal { .. } => result.push(RuleState(
                    WrongNumberLiteralName::Hexadecimal,
                    number_literal,
                )),
                NumberLiteral::Octal { .. } => {
                    result.push(RuleState(WrongNumberLiteralName::Octal, number_literal))
                }
                _ => (),
            }
        }
        result.into_boxed_slice()
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        RuleState(reason, literal): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let title = match reason {
            WrongNumberLiteralName::BigInt => "Bigint is not allowed here.",
            WrongNumberLiteralName::WithUnderscore => {
                "Number literal with underscore is not allowed here."
            }
            WrongNumberLiteralName::Binary => "Binary number literal in is not allowed here.",
            WrongNumberLiteralName::Hexadecimal => {
                "Hexadecimal number literal is not allowed here."
            }
            WrongNumberLiteralName::Octal => "Octal number literal is not allowed here.",
        };

        let diagnostic = RuleDiagnostic::new(rule_category!(), literal.range(), title.to_string());

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        RuleState(reason, literal): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let token = literal.token().ok()?;
        let token_text = token.text_trimmed().to_string();

        let message = match reason {
            WrongNumberLiteralName::Binary
            | WrongNumberLiteralName::Octal
            | WrongNumberLiteralName::Hexadecimal => {
                let text = literal.to_base_ten()?;
                mutation.replace_token(token, make::js_number_literal(text));
                markup! ("Replace "{ token_text } " with "{text.to_string()}).to_owned()
            }
            WrongNumberLiteralName::WithUnderscore | WrongNumberLiteralName::BigInt => {
                let text = literal.value();
                mutation.replace_token(token, make::js_number_literal(text));
                markup! ("Replace "{ token_text } " with "{text}).to_owned()
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}
