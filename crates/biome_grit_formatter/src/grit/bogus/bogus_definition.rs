use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusDefinition;
impl FormatBogusNodeRule<GritBogusDefinition> for FormatGritBogusDefinition {}
