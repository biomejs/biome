use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritRoot, GritRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRoot;

impl FormatNodeRule<GritRoot> for FormatGritRoot {
    fn fmt_fields(&self, node: &GritRoot, f: &mut GritFormatter) -> FormatResult<()> {
        let GritRootFields {
            bom_token,
            version,
            language,
            definitions,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                bom_token.format(),
                version.format(),
                language.format(),
                definitions.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}
