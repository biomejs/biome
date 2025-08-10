use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusTwUtilityValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusTwUtilityValue;
impl FormatBogusNodeRule<CssBogusTwUtilityValue> for FormatCssBogusTwUtilityValue {}
