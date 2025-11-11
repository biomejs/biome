use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueDirectiveArgument, VueDirectiveArgumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDirectiveArgument;
impl FormatNodeRule<VueDirectiveArgument> for FormatVueDirectiveArgument {
    fn fmt_fields(&self, node: &VueDirectiveArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueDirectiveArgumentFields { colon_token, arg } = node.as_fields();

        write!(f, [colon_token.format(), arg.format()])
    }
}
