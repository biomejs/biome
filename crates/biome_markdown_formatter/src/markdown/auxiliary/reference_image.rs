use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdReferenceImage, MdReferenceImageFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceImage;
impl FormatNodeRule<MdReferenceImage> for FormatMdReferenceImage {
    fn fmt_fields(&self, node: &MdReferenceImage, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdReferenceImageFields {
            excl_token,
            l_brack_token,
            alt,
            r_brack_token,
            label,
        } = node.as_fields();

        write!(
            f,
            [
                excl_token.format(),
                l_brack_token.format(),
                alt.format(),
                r_brack_token.format()
            ]
        )?;

        if let Some(label) = label {
            write!(f, [label.format()])?;
        }

        Ok(())
    }
}
