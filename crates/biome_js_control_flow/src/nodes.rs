mod block;
mod bogus;
mod break_stmt;
mod continue_stmt;
mod do_while;
mod for_in;
mod for_of;
mod for_stmt;
mod if_stmt;
mod return_stmt;
mod statement;
mod switch_stmt;
mod throw_stmt;
mod try_catch;
mod variable;
mod while_stmt;

pub(super) use block::*;
pub(super) use bogus::*;
pub(super) use break_stmt::*;
pub(super) use continue_stmt::*;
pub(super) use do_while::*;
pub(super) use for_in::*;
pub(super) use for_of::*;
pub(super) use for_stmt::*;
pub(super) use if_stmt::*;
pub(super) use return_stmt::*;
pub(super) use statement::*;
pub(super) use switch_stmt::*;
pub(super) use throw_stmt::*;
pub(super) use try_catch::*;
pub(super) use variable::*;
pub(super) use while_stmt::*;

pub(super) fn is_truthy_literal(expression: &biome_js_syntax::AnyJsExpression) -> bool {
    use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, numbers::parse_js_number};

    let AnyJsExpression::AnyJsLiteralExpression(literal) = expression.clone().omit_parentheses()
    else {
        return false;
    };

    match literal {
        AnyJsLiteralExpression::JsNumberLiteralExpression(number) => number
            .value_token()
            .ok()
            .and_then(|token| parse_js_number(token.text_trimmed()))
            .is_some_and(|number| number != 0.0 && !number.is_nan()),
        AnyJsLiteralExpression::JsStringLiteralExpression(string) => {
            let Ok(text) = string.inner_string_text() else {
                return false;
            };
            let mut chars = text.chars().peekable();
            while let Some(character) = chars.next() {
                if character != '\\' {
                    return true;
                }
                match chars.next() {
                    Some('\r') => {
                        if chars.peek() == Some(&'\n') {
                            chars.next();
                        }
                    }
                    Some('\n' | '\u{2028}' | '\u{2029}') => {}
                    Some(_) => return true,
                    None => return false,
                }
            }
            false
        }
        AnyJsLiteralExpression::JsRegexLiteralExpression(_) => true,
        _ => literal
            .as_static_value()
            .is_some_and(|value| !value.is_falsy()),
    }
}
