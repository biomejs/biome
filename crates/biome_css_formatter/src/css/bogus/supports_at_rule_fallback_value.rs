use crate::FormatBogusNodeRule;
use biome_css_syntax::CssSupportsAtRuleFallbackValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRuleFallbackValue;
impl FormatBogusNodeRule<CssSupportsAtRuleFallbackValue> for FormatCssSupportsAtRuleFallbackValue {}
