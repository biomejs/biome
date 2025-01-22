use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritEngineName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritEngineName;
impl FormatNodeRule<GritEngineName> for FormatGritEngineName {
    fn fmt_fields(&self, node: &GritEngineName, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.engine_kind().format()])
    }
}
