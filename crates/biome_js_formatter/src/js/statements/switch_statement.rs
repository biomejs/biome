use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsSwitchStatement, JsSwitchStatementFields};
use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSwitchStatement;

impl FormatNodeRule<JsSwitchStatement> for FormatJsSwitchStatement {
    fn fmt_fields(&self, node: &JsSwitchStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = node.as_fields();

        let format_cases = format_with(|f| {
            if cases.is_empty() {
                hard_line_break().fmt(f)?;
            } else {
                cases.format().fmt(f)?;
            }

            Ok(())
        });

        write![
            f,
            [
                switch_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&discriminant.format())),
                r_paren_token.format(),
                space(),
                l_curly_token.format(),
                block_indent(&format_cases),
                r_curly_token.format()
            ]
        ]
    }
}
