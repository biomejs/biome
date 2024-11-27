use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritVersion, GritVersionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVersion;
impl FormatNodeRule<GritVersion> for FormatGritVersion {
    fn fmt_fields(&self, node: &GritVersion, f: &mut GritFormatter) -> FormatResult<()> {
        let GritVersionFields {
            engine_token,
            biome_token,
            l_paren_token,
            version,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                engine_token.format(),
                space(),
                biome_token.format(),
                l_paren_token.format(),
                version.format(),
                r_paren_token.format()
            ]
        )
    }
}
