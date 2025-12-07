use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVBindShorthandDirective, VueVBindShorthandDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVBindShorthandDirective;
impl FormatNodeRule<VueVBindShorthandDirective> for FormatVueVBindShorthandDirective {
    fn fmt_fields(
        &self,
        node: &VueVBindShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let VueVBindShorthandDirectiveFields {
            arg,
            modifiers,
            initializer,
        } = node.as_fields();

        write!(f, [arg.format(), modifiers.format(), initializer.format()])
    }
}
