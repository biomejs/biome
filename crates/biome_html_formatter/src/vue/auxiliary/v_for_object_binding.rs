use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForObjectBinding, VueVForObjectBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForObjectBinding;
impl FormatNodeRule<VueVForObjectBinding> for FormatVueVForObjectBinding {
    fn fmt_fields(&self, node: &VueVForObjectBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForObjectBindingFields {
            l_curly_token,
            bindings,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                space(),
                bindings.format(),
                space(),
                r_curly_token.format()
            ]
        )
    }
}
