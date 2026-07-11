use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowJsonNode, YamlFlowJsonNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowJsonNode;
impl FormatNodeRule<YamlFlowJsonNode> for FormatYamlFlowJsonNode {
    fn fmt_fields(&self, node: &YamlFlowJsonNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowJsonNodeFields {
            properties,
            content,
        } = node.as_fields();

        write!(f, [properties.format()])?;

        if !properties.is_empty() {
            write!(f, [space()])?;
        }

        write!(f, [content.format()])
    }
}
