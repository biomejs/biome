use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsModuleBlock;
use biome_js_syntax::TsModuleBlockFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsModuleBlock;

impl FormatNodeRule<TsModuleBlock> for FormatTsModuleBlock {
    fn fmt_fields(&self, node: &TsModuleBlock, f: &mut JsFormatter) -> FormatResult<()> {
        let TsModuleBlockFields {
            l_curly_token,
            items,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;

        if items.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(f, [block_indent(&items.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(&self, _: &TsModuleBlock, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside `fmt_fields`
        Ok(())
    }
}
