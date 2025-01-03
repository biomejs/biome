use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritListAccessor, GritListAccessorFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListAccessor;
impl FormatNodeRule<GritListAccessor> for FormatGritListAccessor {
    fn fmt_fields(&self, node: &GritListAccessor, f: &mut GritFormatter) -> FormatResult<()> {
        let GritListAccessorFields {
            l_brack_token,
            index,
            list,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                space(),
                index.format(),
                space(),
                list.format(),
                space(),
                r_brack_token.format()
            ]
        )
    }
}
