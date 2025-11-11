use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteName, SvelteNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteName;
impl FormatNodeRule<SvelteName> for FormatSvelteName {
    fn fmt_fields(&self, node: &SvelteName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteNameFields { svelte_ident_token } = node.as_fields();
        write!(f, [svelte_ident_token.format()])
    }
}
