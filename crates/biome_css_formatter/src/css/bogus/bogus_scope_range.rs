use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusScopeRange;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusScopeRange;
impl FormatBogusNodeRule<CssBogusScopeRange> for FormatCssBogusScopeRange {}
