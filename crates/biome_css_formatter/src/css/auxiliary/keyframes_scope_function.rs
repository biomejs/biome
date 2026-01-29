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

        let should_insert_space = f.options().delimiter_spacing().value();

        if should_insert_space {
            write!(
                f,
                [
                    scope.format(),
                    l_paren_token.format(),
                    space(),
                    name.format(),
                    space(),
                    r_paren_token.format(),
                ]
            )
        } else {
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
}
