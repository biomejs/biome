use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdLinkReferenceDefinition, MdLinkReferenceDefinitionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkReferenceDefinition;
impl FormatNodeRule<MdLinkReferenceDefinition> for FormatMdLinkReferenceDefinition {
    fn fmt_fields(
        &self,
        node: &MdLinkReferenceDefinition,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdLinkReferenceDefinitionFields {
            indent,
            l_brack_token,
            label,
            r_brack_token,
            colon_token,
            destination,
            title,
        } = node.as_fields();

        write!(
            f,
            [
                indent.format(),
                l_brack_token.format(),
                label.format(),
                r_brack_token.format(),
                colon_token.format(),
                space(),
                destination.format(),
            ]
        )?;

        if let Some(title) = title {
            write!(f, [space(), title.format()])?;
        }

        Ok(())
    }
}
