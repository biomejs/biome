use super::*;

pub(super) fn convert_directive(ctx: &ConvertCtx<'_>, directive: JsDirective) -> Result<Directive> {
    let token = directive
        .value_token()
        .map_err(|_| missing("JsDirective", "value_token"))?;
    let value = inner_string_text(&token).to_string();
    Ok(Directive {
        base: ctx.base(directive.syntax().text_trimmed_range()),
        value: DirectiveLiteral {
            base: ctx.base(token.text_trimmed_range()),
            value,
        },
    })
}
