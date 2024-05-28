use crate::prelude::*;
use biome_css_syntax::{CssKeyframesScopedName, CssKeyframesScopedNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesScopedName;
impl FormatNodeRule<CssKeyframesScopedName> for FormatCssKeyframesScopedName {
    fn fmt_fields(&self, node: &CssKeyframesScopedName, f: &mut CssFormatter) -> FormatResult<()> {
        let CssKeyframesScopedNameFields { colon_token, scope } = node.as_fields();

        write!(f, [colon_token.format(), scope.format(),])
    }
}
