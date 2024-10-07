use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlDirective, HtmlDirectiveFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDirective;
impl FormatNodeRule<HtmlDirective> for FormatHtmlDirective {
    fn fmt_fields(&self, node: &HtmlDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlDirectiveFields {
            l_angle_token,
            excl_token,
            doctype_token,
            html_token,
            quirk_token,
            public_id_token,
            system_id_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                excl_token.format(),
                doctype_token.format(),
            ]
        )?;
        if let Some(html) = html_token {
            write!(f, [space()])?;
            html.format().fmt(f)?;
        }
        if let Some(quirk) = quirk_token {
            write!(f, [space()])?;
            quirk.format().fmt(f)?;
        }
        if let Some(public_id) = public_id_token {
            write!(f, [space()])?;
            public_id.format().fmt(f)?;
        }
        if let Some(system_id) = system_id_token {
            write!(f, [space()])?;
            system_id.format().fmt(f)?;
        }
        write!(f, [r_angle_token.format(), hard_line_break()])?;
        Ok(())
    }
}
