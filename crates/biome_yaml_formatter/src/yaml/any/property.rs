//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlProperty;
impl FormatRule<AnyYamlProperty> for FormatAnyYamlProperty {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlProperty::YamlAnchorProperty(node) => node.format().fmt(f),
            AnyYamlProperty::YamlTagProperty(node) => node.format().fmt(f),
        }
    }
}
