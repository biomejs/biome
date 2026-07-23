use biome_js_syntax::AnyJsExpression;

/// Recursively strip TypeScript type assertions and parentheses to get the underlying expression.
pub fn unwrap_typescript_expression(expr: AnyJsExpression) -> AnyJsExpression {
    let expr = expr.omit_parentheses();
    match &expr {
        AnyJsExpression::TsAsExpression(ts_as) => {
            if let Ok(inner) = ts_as.expression() {
                return unwrap_typescript_expression(inner);
            }
        }
        AnyJsExpression::TsSatisfiesExpression(ts_sat) => {
            if let Ok(inner) = ts_sat.expression() {
                return unwrap_typescript_expression(inner);
            }
        }
        AnyJsExpression::TsTypeAssertionExpression(ts_ta) => {
            if let Ok(inner) = ts_ta.expression() {
                return unwrap_typescript_expression(inner);
            }
        }
        AnyJsExpression::TsNonNullAssertionExpression(ts_nn) => {
            if let Ok(inner) = ts_nn.expression() {
                return unwrap_typescript_expression(inner);
            }
        }
        _ => {}
    }
    expr
}
