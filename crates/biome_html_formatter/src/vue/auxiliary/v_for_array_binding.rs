use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForArrayBinding, VueVForArrayBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForArrayBinding;
impl FormatNodeRule<VueVForArrayBinding> for FormatVueVForArrayBinding {
    fn fmt_fields(&self, node: &VueVForArrayBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForArrayBindingFields {
            l_brack_token,
            bindings,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                space(),
                bindings.format(),
                space(),
                r_brack_token.format()
            ]
        )
    }
}
