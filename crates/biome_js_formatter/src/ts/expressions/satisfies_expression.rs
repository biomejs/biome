use crate::prelude::*;

use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsSatisfiesExpression;

use super::as_expression::format_as_or_satisfies_expression;

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesExpression;

impl FormatNodeRule<TsSatisfiesExpression> for FormatTsSatisfiesExpression {
    fn fmt_fields(&self, node: &TsSatisfiesExpression, f: &mut JsFormatter) -> FormatResult<()> {
        format_as_or_satisfies_expression(
            f,
            node.syntax(),
            node.expression(),
            node.satisfies_token()?,
            node.ty()?,
        )
    }

    fn needs_parentheses(&self, item: &TsSatisfiesExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsFileSource, TsSatisfiesExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 satisfies number ? true : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? x satisfies number : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? true : x satisfies number", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "class X extends (B satisfies number) {}",
            TsSatisfiesExpression
        );

        assert_needs_parentheses!("(x satisfies Function)()", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies Function)?.()", TsSatisfiesExpression);
        assert_needs_parentheses!("new (x satisfies Function)()", TsSatisfiesExpression);

        assert_needs_parentheses!("<number>(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)`template`", TsSatisfiesExpression);
        assert_needs_parentheses!("!(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("[...(x satisfies any)]", TsSatisfiesExpression);
        assert_needs_parentheses!("({...(x satisfies any)})", TsSatisfiesExpression);
        assert_needs_parentheses!(
            "<test {...(x satisfies any)} />",
            TsSatisfiesExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x satisfies any)}</test>",
            TsSatisfiesExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!("await (x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)!", TsSatisfiesExpression);

        assert_needs_parentheses!("(x satisfies any).member", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)[member]", TsSatisfiesExpression);
        assert_not_needs_parentheses!("object[x satisfies any]", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_not_needs_parentheses!(
            "x satisfies number satisfies string",
            TsSatisfiesExpression[1]
        );
    }
}
