use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularTwoWayBinding, AngularTwoWayBindingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularTwoWayBinding;
impl FormatNodeRule<AngularTwoWayBinding> for FormatAngularTwoWayBinding {
    fn fmt_fields(&self, node: &AngularTwoWayBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AngularTwoWayBindingFields {
            l_bracket_paren_token,
            name,
            initializer,
            r_bracket_paren_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_bracket_paren_token.format(),
                name.format(),
                initializer.format(),
                r_bracket_paren_token.format()
            ]
        )
    }
}
