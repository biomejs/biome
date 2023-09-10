use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsxSpreadAttribute, JsxSpreadAttributeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSpreadAttribute;

impl FormatNodeRule<JsxSpreadAttribute> for FormatJsxSpreadAttribute {
    fn fmt_fields(&self, node: &JsxSpreadAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = node.as_fields();

        let argument = argument?;
        let format_inner =
            format_with(|f| write!(f, [dotdotdot_token.format(), argument.format(),]));

        write!(f, [l_curly_token.format()])?;

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            write!(f, [soft_block_indent(&format_inner)])?;
        } else {
            write!(f, [format_inner])?;
        }

        write![f, [r_curly_token.format()]]
    }
}
