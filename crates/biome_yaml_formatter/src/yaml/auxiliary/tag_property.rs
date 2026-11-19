use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlTagProperty, YamlTagPropertyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlTagProperty;
impl FormatNodeRule<YamlTagProperty> for FormatYamlTagProperty {
    fn fmt_fields(&self, node: &YamlTagProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlTagPropertyFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
