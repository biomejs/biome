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
                // A line break between the properties and the content is
                // kept: the parser attaches the properties of a following
                // block collection to its first key, so joining the lines
                // would move them onto the key for real:
                //
                // ```yaml
                // - !circle
                //   center: 1
                // ```
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
