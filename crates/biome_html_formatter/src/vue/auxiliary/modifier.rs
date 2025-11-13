use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueModifier, VueModifierFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueModifier;
impl FormatNodeRule<VueModifier> for FormatVueModifier {
    fn fmt_fields(&self, node: &VueModifier, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueModifierFields {
            dot_token,
            modifier_token,
        } = node.as_fields();

        write!(f, [dot_token.format(), modifier_token.format()])
    }
}
