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
            eof_token,
        } = node.as_fields();

        if let Some(bom) = bom_token {
            bom.format().fmt(f)?;
        }

        if let Some(frontmatter) = frontmatter {
            write!(f, [frontmatter.format(), hard_line_break()])?;
        }

        if let Some(directive) = directive {
            directive.format().fmt(f)?;
        }

        html.format().fmt(f)?;

        write!(f, [hard_line_break(), format_removed(&eof_token?)])?;

        Ok(())
    }
}
