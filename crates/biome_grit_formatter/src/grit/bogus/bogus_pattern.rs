use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusPattern;
impl FormatBogusNodeRule<GritBogusPattern> for FormatGritBogusPattern {}
