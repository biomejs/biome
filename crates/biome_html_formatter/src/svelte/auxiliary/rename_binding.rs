use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteRenameBinding, SvelteRenameBindingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteRenameBinding;
impl FormatNodeRule<SvelteRenameBinding> for FormatSvelteRenameBinding {
    fn fmt_fields(&self, node: &SvelteRenameBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteRenameBindingFields {
            key,
            colon_token,
            name,
        } = node.as_fields();

        write!(f, [key.format(), colon_token.format(), space(), name.format()])
    }
}
