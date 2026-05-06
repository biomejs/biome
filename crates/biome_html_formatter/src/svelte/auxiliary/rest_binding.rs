use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteRestBinding, SvelteRestBindingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteRestBinding;
impl FormatNodeRule<SvelteRestBinding> for FormatSvelteRestBinding {
    fn fmt_fields(&self, node: &SvelteRestBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteRestBindingFields {
            dotdotdot_token,
            name,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), name.format()])
    }
}
