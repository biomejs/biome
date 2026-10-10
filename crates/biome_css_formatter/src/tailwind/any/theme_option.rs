//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwThemeOption;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwThemeOption;
impl FormatRule<AnyTwThemeOption> for FormatAnyTwThemeOption {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwThemeOption, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwThemeOption::CssBogus(node) => node.format().fmt(f),
            AnyTwThemeOption::CssIdentifier(node) => node.format().fmt(f),
            AnyTwThemeOption::TwThemePrefixOption(node) => node.format().fmt(f),
        }
    }
}
