use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsTypeofType, TsTypeofTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeofType;

impl FormatNodeRule<TsTypeofType> for FormatTsTypeofType {
    fn fmt_fields(&self, node: &TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeofTypeFields {
            type_arguments,
            typeof_token,
            expression_name,
        } = node.as_fields();

        write![
            f,
            [
                typeof_token.format(),
                space(),
                expression_name.format(),
                type_arguments.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeofType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeofType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
