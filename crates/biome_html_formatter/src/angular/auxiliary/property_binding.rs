use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularPropertyBinding, AngularPropertyBindingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularPropertyBinding;
impl FormatNodeRule<AngularPropertyBinding> for FormatAngularPropertyBinding {
    fn fmt_fields(&self, node: &AngularPropertyBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AngularPropertyBindingFields {
            l_brack_token,
            name,
            initializer,
            r_brack_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_brack_token.format(),
                name.format(),
                initializer.format(),
                r_brack_token.format()
            ]
        )
    }
}
