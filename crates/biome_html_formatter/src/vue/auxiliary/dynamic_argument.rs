use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueDynamicArgument, VueDynamicArgumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueDynamicArgument;
impl FormatNodeRule<VueDynamicArgument> for FormatVueDynamicArgument {
    fn fmt_fields(&self, node: &VueDynamicArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueDynamicArgumentFields {
            l_brack_token,
            name_token,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                name_token.format(),
                r_brack_token.format()
            ]
        )
    }
}
