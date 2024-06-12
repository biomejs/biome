use crate::prelude::*;
use biome_css_syntax::{CssKeyframesScopeFunction, CssKeyframesScopeFunctionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesScopeFunction;
impl FormatNodeRule<CssKeyframesScopeFunction> for FormatCssKeyframesScopeFunction {
    fn fmt_fields(
        &self,
        node: &CssKeyframesScopeFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssKeyframesScopeFunctionFields {
            scope,
            l_paren_token,
            name,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                scope.format(),
                l_paren_token.format(),
                name.format(),
                r_paren_token.format(),
            ]
        )
    }
}
