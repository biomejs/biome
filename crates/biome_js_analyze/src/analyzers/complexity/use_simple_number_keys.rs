use crate::JsRuleAction;

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsObjectMember, JsLiteralMemberName, JsObjectExpression, JsSyntaxKind, JsSyntaxToken,
    TextRange,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult};
use std::str::FromStr;

declare_rule! {
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
    pub(crate) UseSimpleNumberKeys {
        version: "1.0.0",
        name: "useSimpleNumberKeys",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Clone)]
pub enum NumberLiteral {
    Binary {
        node: JsLiteralMemberName,
        value: String,
        big_int: bool,
    },
    Decimal {
        node: JsLiteralMemberName,
        value: String,
        big_int: bool,
        underscore: bool,
    },
    Octal {
        node: JsLiteralMemberName,
        value: String,
        big_int: bool,
    },
    Hexadecimal {
        node: JsLiteralMemberName,
        value: String,
        big_int: bool,
    },
    FloatingPoint {
        node: JsLiteralMemberName,
        value: String,
        exponent: bool,
        underscore: bool,
    },
}

pub struct NumberLiteralError;

impl TryFrom<AnyJsObjectMember> for NumberLiteral {
    type Error = NumberLiteralError;

    fn try_from(any_member: AnyJsObjectMember) -> Result<Self, Self::Error> {
        let Some(literal_member_name_syntax) = any_member
            .syntax()
            .children()
            .find(|x| JsLiteralMemberName::can_cast(x.kind()))
        else {
            return Err(NumberLiteralError);
        };
        let literal_member_name = JsLiteralMemberName::cast(literal_member_name_syntax).unwrap();

        let token = literal_member_name.value().unwrap();
        match token.kind() {
            JsSyntaxKind::JS_NUMBER_LITERAL | JsSyntaxKind::JS_BIGINT_LITERAL => {
                let chars: Vec<char> = token.text_trimmed().chars().collect();
                let mut value = String::new();

                let mut is_first_char_zero: bool = false;
                let mut is_second_char_a_letter: Option<char> = None;
                let mut contains_dot: bool = false;
                let mut exponent: bool = false;
                let mut largest_digit: char = '0';
                let mut underscore: bool = false;
                let mut big_int: bool = false;

                for i in 0..chars.len() {
                    if i == 0 && chars[i] == '0' && chars.len() > 1 {
                        is_first_char_zero = true;
                        continue;
                    }

                    if chars[i] == 'n' {
                        big_int = true;
                        break;
                    }

                    if chars[i] == 'e' || chars[i] == 'E' {
                        exponent = true;
                    }

                    if i == 1 && chars[i].is_alphabetic() && !exponent {
                        is_second_char_a_letter = Some(chars[i]);
                        continue;
                    }

                    if chars[i] == '_' {
                        underscore = true;
                        continue;
                    }

                    if chars[i] == '.' {
                        contains_dot = true;
                    }

                    if largest_digit < chars[i] {
                        largest_digit = chars[i];
                    }

                    value.push(chars[i])
                }

                if contains_dot {
                    return Ok(Self::FloatingPoint {
                        node: literal_member_name,
                        value,
                        exponent,
                        underscore,
                    });
                };
                if !is_first_char_zero {
                    return Ok(Self::Decimal {
                        node: literal_member_name,
                        value,
                        big_int,
                        underscore,
                    });
                };

                match is_second_char_a_letter {
                    Some('b' | 'B') => {
                        return Ok(Self::Binary {
                            node: literal_member_name,
                            value,
                            big_int,
                        })
                    }
                    Some('o' | 'O') => {
                        return Ok(Self::Octal {
                            node: literal_member_name,
                            value,
                            big_int,
                        })
                    }
                    Some('x' | 'X') => {
                        return Ok(Self::Hexadecimal {
                            node: literal_member_name,
                            value,
                            big_int,
                        })
                    }
                    _ => (),
                }

                if largest_digit < '8' {
                    return Ok(Self::Octal {
                        node: literal_member_name,
                        value,
                        big_int,
                    });
                }

                Ok(Self::Decimal {
                    node: literal_member_name,
                    value,
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

    fn value(&self) -> &String {
        match self {
            Self::Decimal { value, .. } => value,
            Self::Binary { value, .. } => value,
            Self::FloatingPoint { value, .. } => value,
            Self::Octal { value, .. } => value,
            Self::Hexadecimal { value, .. } => value,
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
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals: Self::Signals = Vec::new();
        let node = ctx.query();

        for number_literal in node
            .members()
            .into_iter()
            .flatten()
            .filter_map(|member| NumberLiteral::try_from(member).ok())
        {
            match number_literal {
                NumberLiteral::Decimal { big_int: true, .. } => {
                    signals.push(RuleState(WrongNumberLiteralName::BigInt, number_literal))
                }
                NumberLiteral::FloatingPoint {
                    underscore: true, ..
                }
                | NumberLiteral::Decimal {
                    underscore: true, ..
                } => signals.push(RuleState(
                    WrongNumberLiteralName::WithUnderscore,
                    number_literal,
                )),
                NumberLiteral::Binary { .. } => {
                    signals.push(RuleState(WrongNumberLiteralName::Binary, number_literal))
                }
                NumberLiteral::Hexadecimal { .. } => signals.push(RuleState(
                    WrongNumberLiteralName::Hexadecimal,
                    number_literal,
                )),
                NumberLiteral::Octal { .. } => {
                    signals.push(RuleState(WrongNumberLiteralName::Octal, number_literal))
                }
                _ => (),
            }
        }

        signals
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

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message,
            mutation,
        })
    }
}
