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

        if let Some(content) = content {
            if !properties.is_empty() {
                if get_lines_before(content.syntax()) > 0 {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [space()])?;
                }
            }

            write!(f, [content.format()])?;
        }

        Ok(())
    }
}
