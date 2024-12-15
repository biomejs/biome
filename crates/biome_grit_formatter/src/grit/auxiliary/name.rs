use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritName;
impl FormatNodeRule<GritName> for FormatGritName {
    fn fmt_fields(&self, node: &GritName, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.value_token().format()])
    }
}
