use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularEventBinding, AngularEventBindingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularEventBinding;
impl FormatNodeRule<AngularEventBinding> for FormatAngularEventBinding {
    fn fmt_fields(&self, node: &AngularEventBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AngularEventBindingFields {
            l_paren_token,
            name,
            initializer,
            r_paren_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_paren_token.format(),
                name.format(),
                initializer.format(),
                r_paren_token.format()
            ]
        )
    }
}
