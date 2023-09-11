use crate::prelude::*;
use biome_formatter::{format_args, write};

use biome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumDeclaration;

impl FormatNodeRule<TsEnumDeclaration> for FormatTsEnumDeclaration {
    fn fmt_fields(&self, node: &TsEnumDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        if let Some(const_token) = const_token {
            write!(f, [const_token.format(), space()])?;
        }

        write!(
            f,
            [
                enum_token.format(),
                space(),
                id.format(),
                space(),
                l_curly_token.format(),
            ]
        )?;

        if members.is_empty() {
            write!(
                f,
                [group(&format_args![
                    format_dangling_comments(node.syntax()),
                    soft_line_break()
                ])]
            )?;
        } else {
            write!(f, [block_indent(&members.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &TsEnumDeclaration,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
