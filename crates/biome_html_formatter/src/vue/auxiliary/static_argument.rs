use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueStaticArgument, VueStaticArgumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueStaticArgument;
impl FormatNodeRule<VueStaticArgument> for FormatVueStaticArgument {
    fn fmt_fields(&self, node: &VueStaticArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueStaticArgumentFields { name_token } = node.as_fields();

        write!(f, [name_token.format()])
    }
}
