use crate::FormatBogusNodeRule;
use biome_css_syntax::CssValueAtRuleGenericValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleGenericValue;
impl FormatBogusNodeRule<CssValueAtRuleGenericValue> for FormatCssValueAtRuleGenericValue {}
