use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVSlotShorthandDirective, VueVSlotShorthandDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVSlotShorthandDirective;
impl FormatNodeRule<VueVSlotShorthandDirective> for FormatVueVSlotShorthandDirective {
    fn fmt_fields(
        &self,
        node: &VueVSlotShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let VueVSlotShorthandDirectiveFields {
            hash_token,
            arg,
            modifiers,
            initializer,
        } = node.as_fields();

        write!(
            f,
            [
                hash_token.format(),
                arg.format(),
                modifiers.format(),
                initializer.format()
            ]
        )
    }
}
