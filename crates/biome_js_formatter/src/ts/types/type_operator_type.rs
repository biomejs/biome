use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{TsTypeOperatorType, TsTypeOperatorTypeFields};

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

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsTypeOperatorType;

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
