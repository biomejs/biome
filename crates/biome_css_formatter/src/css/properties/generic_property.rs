use crate::comments::FormatCssLeadingComment;
use crate::prelude::*;
use biome_css_syntax::{CssGenericProperty, CssGenericPropertyFields};
use biome_formatter::{CstFormatContext, FormatRefWithRule, format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericProperty;
impl FormatNodeRule<CssGenericProperty> for FormatCssGenericProperty {
    fn fmt_fields(&self, node: &CssGenericProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssGenericPropertyFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [name.format(), colon_token.format()])?;

        // Format trailing comments inline after the colon
        let comments = f.context().comments().clone();
        let trailing_comments = comments.trailing_comments(node.syntax());

        if !trailing_comments.is_empty() {
            for comment in trailing_comments {
                write!(f, [space()])?;
                let format_comment = FormatRefWithRule::new(comment, FormatCssLeadingComment);
                write!(f, [format_comment])?;
                comment.mark_formatted();
            }
            write!(
                f,
                [indent(&format_args![hard_line_break(), &value.format()])]
            )
        } else {
            write!(f, [space(), value.format()])
        }
    }

    fn fmt_trailing_comments(
        &self,
        _node: &CssGenericProperty,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Trailing comments are formatted inline in fmt_fields
        Ok(())
    }
}
