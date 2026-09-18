use crate::prelude::*;

use crate::utils::format_node_without_comments::FormatAnyJsExpressionWithoutComments;
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
        let delimiter_spacing = f.options().delimiter_spacing().value();

        let format_inner = format_with(|f| {
            if f.comments().is_suppressed(argument.syntax()) {
                write!(
                    f,
                    [
                        dotdotdot_token.format(),
                        argument.format(),
                        line_suffix_boundary()
                    ]
                )
            } else {
                write!(
                    f,
                    [
                        format_leading_comments(argument.syntax()),
                        dotdotdot_token.format(),
                    ]
                )?;
                FormatAnyJsExpressionWithoutComments.fmt(&argument, f)?;
                write!(
                    f,
                    [
                        format_dangling_comments(argument.syntax()).with_soft_block_indent(),
                        format_trailing_comments(argument.syntax()),
                        line_suffix_boundary()
                    ]
                )
            }
        });

        write!(f, [l_curly_token.format()])?;

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            if delimiter_spacing {
                // With delimiter spacing, try to fit on one line with spaces
                write!(
                    f,
                    [group(&soft_block_indent_with_maybe_space(
                        &format_inner,
                        true
                    ))]
                )?;
            } else {
                // Without delimiter spacing, always break to multiple lines
                write!(f, [soft_block_indent(&format_inner)])?;
            }
        } else if delimiter_spacing {
            write!(f, [space(), format_inner, space()])?;
        } else {
            write!(f, [format_inner])?;
        }

        write!(f, [r_curly_token.format()])
    }
}
