use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsTupleType, TsTupleTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTupleType;

impl FormatNodeRule<TsTupleType> for FormatTsTupleType {
    fn fmt_fields(&self, node: &TsTupleType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTupleTypeFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(f, [l_brack_token.format(),])?;

        if elements.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(f, [group(&soft_block_indent(&elements.format())),])?;
        }

        write!(f, [r_brack_token.format(),])
    }

    fn needs_parentheses(&self, item: &TsTupleType) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(&self, _: &TsTupleType, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}

impl NeedsParentheses for TsTupleType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
