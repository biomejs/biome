use crate::FormatBogusNodeRule;
use biome_css_syntax::CssUnknownAtRuleParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownAtRuleParameterList;
impl FormatBogusNodeRule<CssUnknownAtRuleParameterList> for FormatCssUnknownAtRuleParameterList {}
