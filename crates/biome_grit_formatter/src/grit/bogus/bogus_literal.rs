use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusLiteral;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLiteral;
impl FormatBogusNodeRule<GritBogusLiteral> for FormatGritBogusLiteral {}
