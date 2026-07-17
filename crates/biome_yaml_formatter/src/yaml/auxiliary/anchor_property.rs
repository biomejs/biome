use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlAnchorProperty, YamlAnchorPropertyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlAnchorProperty;
impl FormatNodeRule<YamlAnchorProperty> for FormatYamlAnchorProperty {
    fn fmt_fields(&self, node: &YamlAnchorProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlAnchorPropertyFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
