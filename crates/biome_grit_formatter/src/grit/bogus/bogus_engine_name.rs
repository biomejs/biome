use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusEngineName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusEngineName;
impl FormatBogusNodeRule<GritBogusEngineName> for FormatGritBogusEngineName {}
