use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusSyntax;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSyntax;
impl FormatBogusNodeRule<CssBogusSyntax> for FormatCssBogusSyntax {}
