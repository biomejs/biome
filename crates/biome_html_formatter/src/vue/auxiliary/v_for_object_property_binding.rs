use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForObjectPropertyBinding, VueVForObjectPropertyBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForObjectPropertyBinding;
impl FormatNodeRule<VueVForObjectPropertyBinding> for FormatVueVForObjectPropertyBinding {
    fn fmt_fields(
        &self,
        node: &VueVForObjectPropertyBinding,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let VueVForObjectPropertyBindingFields {
            property,
            colon_token,
            binding,
        } = node.as_fields();

        write!(
            f,
            [
                property.format(),
                colon_token.format(),
                space(),
                binding.format()
            ]
        )
    }
}
