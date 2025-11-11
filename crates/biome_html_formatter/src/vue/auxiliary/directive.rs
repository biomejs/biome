use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueDirective, VueDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDirective;
impl FormatNodeRule<VueDirective> for FormatVueDirective {
    fn fmt_fields(&self, node: &VueDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueDirectiveFields {
            name_token,
            arg,
            modifiers,
            initializer,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                arg.format(),
                modifiers.format(),
                initializer.format()
            ]
        )
    }
}
