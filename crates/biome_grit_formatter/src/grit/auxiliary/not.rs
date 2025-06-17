use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritNot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNot;
impl FormatNodeRule<GritNot> for FormatGritNot {
    fn fmt_fields(&self, node: &GritNot, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.token().format(), space()])
    }
}
