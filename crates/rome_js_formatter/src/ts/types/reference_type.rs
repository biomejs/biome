use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsReferenceType, TsReferenceTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsReferenceType;

impl FormatNodeRule<TsReferenceType> for FormatTsReferenceType {
    fn fmt_fields(&self, node: &TsReferenceType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReferenceTypeFields {
            name,
            type_arguments,
        } = node.as_fields();

        write![f, [name.format(), type_arguments.format()]]
    }

    fn needs_parentheses(&self, item: &TsReferenceType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsReferenceType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
