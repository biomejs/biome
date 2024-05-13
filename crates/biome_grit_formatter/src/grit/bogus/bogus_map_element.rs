use crate::FormatBogusNodeRule;
use biome_grit_syntax::GritBogusMapElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusMapElement;
impl FormatBogusNodeRule<GritBogusMapElement> for FormatGritBogusMapElement {}
