use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusFontFeatureValuesItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusFontFeatureValuesItem;
impl FormatBogusNodeRule<CssBogusFontFeatureValuesItem> for FormatCssBogusFontFeatureValuesItem {}
