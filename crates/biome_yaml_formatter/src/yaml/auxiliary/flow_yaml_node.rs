use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowYamlNode, YamlFlowYamlNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowYamlNode;
impl FormatNodeRule<YamlFlowYamlNode> for FormatYamlFlowYamlNode {
    fn fmt_fields(&self, node: &YamlFlowYamlNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowYamlNodeFields {
            properties,
            content,
        } = node.as_fields();

        write!(f, [properties.format()])?;

        if !properties.is_empty() && content.is_some() {
            write!(f, [space()])?;
        }

        write!(f, [content.format()])
    }
}
