use super::*;

pub(super) fn convert_literal(
    ctx: &ConvertCtx<'_>,
    literal: AnyJsLiteralExpression,
) -> Result<Expression> {
    match literal {
        AnyJsLiteralExpression::JsStringLiteralExpression(literal) => {
            let token = literal
                .value_token()
                .map_err(|_| missing("JsStringLiteralExpression", "value_token"))?;
            let value = inner_string_text(&token).to_string();
            Ok(Expression::StringLiteral(StringLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
                value,
            }))
        }
        AnyJsLiteralExpression::JsNumberLiteralExpression(literal) => {
            let token = literal
                .value_token()
                .map_err(|_| missing("JsNumberLiteralExpression", "value_token"))?;
            let value =
                token
                    .text_trimmed()
                    .parse()
                    .map_err(|_| ReactCompilerError::InvalidLiteral {
                        range: token.text_trimmed_range(),
                        reason: "number literal is not supported yet",
                    })?;
            Ok(Expression::NumericLiteral(NumericLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
                value,
            }))
        }
        AnyJsLiteralExpression::JsBooleanLiteralExpression(literal) => {
            let token = literal
                .value_token()
                .map_err(|_| missing("JsBooleanLiteralExpression", "value_token"))?;
            Ok(Expression::BooleanLiteral(BooleanLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
                value: token.text_trimmed() == "true",
            }))
        }
        AnyJsLiteralExpression::JsNullLiteralExpression(literal) => {
            Ok(Expression::NullLiteral(NullLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
            }))
        }
        AnyJsLiteralExpression::JsBigintLiteralExpression(literal) => {
            let token = literal
                .value_token()
                .map_err(|_| missing("JsBigintLiteralExpression", "value_token"))?;
            Ok(Expression::BigIntLiteral(BigIntLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
                value: token.text_trimmed().trim_end_matches('n').to_string(),
            }))
        }
        AnyJsLiteralExpression::JsRegexLiteralExpression(literal) => {
            let token = literal
                .value_token()
                .map_err(|_| missing("JsRegexLiteralExpression", "value_token"))?;
            let text = token.text_trimmed();
            let (pattern, flags) = text
                .strip_prefix('/')
                .and_then(|text| text.rsplit_once('/'))
                .ok_or(ReactCompilerError::InvalidLiteral {
                    range: token.text_trimmed_range(),
                    reason: "regular expression literal is malformed",
                })?;
            Ok(Expression::RegExpLiteral(RegExpLiteral {
                base: ctx.base(literal.syntax().text_trimmed_range()),
                pattern: pattern.to_string(),
                flags: flags.to_string(),
            }))
        }
    }
}
