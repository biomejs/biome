use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusTailwindUtilityValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusTailwindUtilityValue;
impl FormatBogusNodeRule<CssBogusTailwindUtilityValue> for FormatCssBogusTailwindUtilityValue {}
