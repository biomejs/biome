use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritList, GritListFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritList;
impl FormatNodeRule<GritList> for FormatGritList {
    fn fmt_fields(&self, node: &GritList, f: &mut GritFormatter) -> FormatResult<()> {
        let GritListFields {
            l_brack_token,
            name,
            patterns,
            r_brack_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_brack_token.format(),
                name.format(),
                patterns.format(),
                r_brack_token.format(),
            ]
        )
    }
}
