use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForRestBinding, VueVForRestBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForRestBinding;
impl FormatNodeRule<VueVForRestBinding> for FormatVueVForRestBinding {
    fn fmt_fields(&self, node: &VueVForRestBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForRestBindingFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), binding.format()])
    }
}
