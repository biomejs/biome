use crate::prelude::*;

use crate::parentheses::{operator_type_or_higher_needs_parens, NeedsParentheses};
use biome_rowan::AstNode;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsTypeOperatorType, TsTypeOperatorTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeOperatorType;

impl FormatNodeRule<TsTypeOperatorType> for FormatTsTypeOperatorType {
    fn fmt_fields(&self, node: &TsTypeOperatorType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeOperatorTypeFields { operator_token, ty } = node.as_fields();

        write![f, [operator_token.format(), space(), ty.format()]]
    }

    fn needs_parentheses(&self, item: &TsTypeOperatorType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeOperatorType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        operator_type_or_higher_needs_parens(self.syntax(), parent)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::TsTypeOperatorType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("let s: (unique symbol)[] = symbol();", TsTypeOperatorType);

        assert_needs_parentheses!("let s: unique (unique symbol);", TsTypeOperatorType[1]);
        assert_not_needs_parentheses!("let s: unique (unique symbol);", TsTypeOperatorType[0]);

        assert_needs_parentheses!("let s: [number, ...(unique symbol)]", TsTypeOperatorType);
        assert_needs_parentheses!("let s: [(unique symbol)?]", TsTypeOperatorType);

        assert_needs_parentheses!("let s: (unique symbol)[a]", TsTypeOperatorType);
        assert_not_needs_parentheses!("let s: a[(unique symbol)]", TsTypeOperatorType);
    }
}
