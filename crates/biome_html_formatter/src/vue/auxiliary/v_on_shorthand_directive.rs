use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVOnShorthandDirective, VueVOnShorthandDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVOnShorthandDirective;
impl FormatNodeRule<VueVOnShorthandDirective> for FormatVueVOnShorthandDirective {
    fn fmt_fields(
        &self,
        node: &VueVOnShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let VueVOnShorthandDirectiveFields {
            at_token,
            arg,
            modifiers,
            initializer,
        } = node.as_fields();

        write!(
            f,
            [
                at_token.format(),
                arg.format(),
                modifiers.format(),
                initializer.format()
            ]
        )
    }
}
