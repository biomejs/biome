use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritNodeLike, GritNodeLikeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNodeLike;
impl FormatNodeRule<GritNodeLike> for FormatGritNodeLike {
    fn fmt_fields(&self, node: &GritNodeLike, f: &mut GritFormatter) -> FormatResult<()> {
        let GritNodeLikeFields {
            l_paren_token,
            name,
            r_paren_token,
            named_args,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                named_args.format(),
                r_paren_token.format()
            ]
        )
    }
}
