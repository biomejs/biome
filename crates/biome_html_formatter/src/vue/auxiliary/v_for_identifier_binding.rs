use crate::prelude::*;
use biome_html_syntax::VueVForIdentifierBinding;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForIdentifierBinding;
impl FormatNodeRule<VueVForIdentifierBinding> for FormatVueVForIdentifierBinding {
    fn fmt_fields(
        &self,
        node: &VueVForIdentifierBinding,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        node.name_token().format().fmt(f)
    }
}
