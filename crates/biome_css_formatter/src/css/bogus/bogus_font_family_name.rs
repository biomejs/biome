use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusFontFamilyName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusFontFamilyName;
impl FormatBogusNodeRule<CssBogusFontFamilyName> for FormatCssBogusFontFamilyName {}
