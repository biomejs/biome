use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusVersion;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusVersion;
impl FormatBogusNodeRule<GritBogusVersion> for FormatGritBogusVersion {}
