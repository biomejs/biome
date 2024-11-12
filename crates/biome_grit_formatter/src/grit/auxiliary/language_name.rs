use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritLanguageName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageName;
impl FormatNodeRule<GritLanguageName> for FormatGritLanguageName {
    fn fmt_fields(&self, node: &GritLanguageName, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.language_kind().format()])
    }
}
