use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusContainer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusContainer;
impl FormatBogusNodeRule<GritBogusContainer> for FormatGritBogusContainer {}
