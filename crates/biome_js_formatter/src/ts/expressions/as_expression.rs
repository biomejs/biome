use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    AnyJsComputedMember, AnyJsExpression, AnyTsType, JsLanguage, JsSyntaxKind, JsSyntaxNode,
    TsAsExpression,
};
use biome_rowan::{SyntaxResult, SyntaxToken};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsExpression;

impl FormatNodeRule<TsAsExpression> for FormatTsAsExpression {
    fn fmt_fields(&self, node: &TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        format_as_or_satisfies_expression(
            f,
            node.syntax(),
            node.expression(),
            node.as_token()?,
            node.ty()?,
        )
    }

    fn needs_parentheses(&self, item: &TsAsExpression) -> bool {
        item.needs_parentheses()
    }
}

pub(crate) fn format_as_or_satisfies_expression(
    f: &mut Formatter<JsFormatContext>,
    node: &JsSyntaxNode,
    expression: SyntaxResult<AnyJsExpression>,
    operation_token: SyntaxToken<JsLanguage>,
    ty: AnyTsType,
) -> FormatResult<()> {
    let format_inner = format_with(|f| {
        write!(f, [expression.format(), space(), operation_token.format()])?;

        if f.comments().has_leading_own_line_comment(ty.syntax()) {
            write!(f, [indent(&format_args![hard_line_break(), &ty.format()])])
        } else {
            write!(f, [space(), ty.format()])
        }
    });

    let is_callee_or_object = node.parent().is_some_and(|parent| {
        match parent.kind() {
            // Callee
            JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            // Static member
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => true,
            _ => AnyJsComputedMember::cast(parent)
                .and_then(|member| member.object().ok())
                .is_some_and(|object| object.syntax() == node),
        }
    });

    if is_callee_or_object {
        write!(f, [group(&soft_block_indent(&format_inner))])
    } else {
        write!(f, [format_inner])
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsFileSource, TsAsExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 as number ? true : false", TsAsExpression);
        assert_needs_parentheses!("cond ? x as number : false", TsAsExpression);
        assert_needs_parentheses!("cond ? true : x as number", TsAsExpression);

        assert_needs_parentheses!("class X extends (B as number) {}", TsAsExpression);

        assert_needs_parentheses!("(x as Function)()", TsAsExpression);
        assert_needs_parentheses!("(x as Function)?.()", TsAsExpression);
        assert_needs_parentheses!("new (x as Function)()", TsAsExpression);

        assert_needs_parentheses!("<number>(x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)`template`", TsAsExpression);
        assert_needs_parentheses!("!(x as any)", TsAsExpression);
        assert_needs_parentheses!("[...(x as any)]", TsAsExpression);
        assert_needs_parentheses!("({...(x as any)})", TsAsExpression);
        assert_needs_parentheses!(
            "<test {...(x as any)} />",
            TsAsExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x as any)}</test>",
            TsAsExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!("await (x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)!", TsAsExpression);

        assert_needs_parentheses!("(x as any).member", TsAsExpression);
        assert_needs_parentheses!("(x as any)[member]", TsAsExpression);
        assert_not_needs_parentheses!("object[x as any]", TsAsExpression);

        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[1]);

        assert_not_needs_parentheses!("x as number as string", TsAsExpression[1]);

        // default-exported function expressions require parentheses, otherwise
        // the end of the function ends the export declaration, and the `as`
        // gets treated as a new statement.
        assert_needs_parentheses!(
            "export default (function foo(){} as typeof console.log)",
            TsAsExpression
        );
        assert_not_needs_parentheses!("export default foo as bar", TsAsExpression);
    }
}
