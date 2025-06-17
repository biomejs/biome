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

        write!(f, [bom_token.format()])?;
        let mut has_header = false;

        if let Some(version) = version {
            write!(f, [version.format(), hard_line_break()])?;
            has_header = true;
        }

        if let Some(language) = language {
            write!(f, [language.format(), hard_line_break()])?;
            has_header = true;
        }

        if has_header {
            write!(f, [empty_line()])?;
        }

        write!(
            f,
            [
                definitions.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}
