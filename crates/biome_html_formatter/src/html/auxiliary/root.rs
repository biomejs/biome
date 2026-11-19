use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlRoot, HtmlRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlRoot;
impl FormatNodeRule<HtmlRoot> for FormatHtmlRoot {
    fn fmt_fields(&self, node: &HtmlRoot, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlRootFields {
            html,
            bom_token,
            directive,
            frontmatter,
            processing_instruction,
            eof_token,
        } = node.as_fields();

        if let Some(bom) = bom_token {
            bom.format().fmt(f)?;
        }

        if let Some(frontmatter) = frontmatter {
            write!(f, [frontmatter.format(), empty_line()])?;
        }

        if let Some(processing_instruction) = processing_instruction {
            write!(
                f,
                [group(&processing_instruction.format()), hard_line_break()]
            )?;
        }

        if let Some(directive) = directive {
            directive.format().fmt(f)?;
        }

        html.format().fmt(f)?;

        write!(f, [format_trailing_comments(node.syntax())])?;

        write!(f, [format_removed(&eof_token?)])?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &HtmlRoot, _: &mut HtmlFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`.
        Ok(())
    }
}
