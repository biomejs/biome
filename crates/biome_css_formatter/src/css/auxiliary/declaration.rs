use crate::prelude::*;
use biome_css_syntax::{CssDeclaration, CssDeclarationFields};
use biome_formatter::trivia::format_dangling_comments;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclaration;
impl FormatNodeRule<CssDeclaration> for FormatCssDeclaration {
    fn fmt_fields(&self, node: &CssDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDeclarationFields {
            property,
            important,
        } = node.as_fields();

        write!(f, [property.format()])?;

        if let Some(important) = important {
            if f.comments().has_dangling_comments(node.syntax()) {
                write!(
                    f,
                    [
                        space(),
                        format_dangling_comments(node.syntax()).with_block_comments_on_line()
                    ]
                )?;
            }

            write!(f, [space(), important.format()])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        node: &CssDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if node.important().is_some() {
            // `fmt_fields` prints value/important comments at their syntax boundary.
            Ok(())
        } else {
            write!(f, [format_dangling_comments(node.syntax())])
        }
    }
}
