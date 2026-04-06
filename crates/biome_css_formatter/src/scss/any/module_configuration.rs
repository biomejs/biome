//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssModuleConfiguration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssModuleConfiguration;
impl FormatRule<AnyScssModuleConfiguration> for FormatAnyScssModuleConfiguration {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssModuleConfiguration, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssModuleConfiguration::CssBogusParameter(node) => node.format().fmt(f),
            AnyScssModuleConfiguration::ScssModuleConfiguration(node) => node.format().fmt(f),
        }
    }
}
