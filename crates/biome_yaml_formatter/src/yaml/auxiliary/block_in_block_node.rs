use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, YamlBlockInBlockNode, YamlBlockInBlockNodeFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockInBlockNode;
impl FormatNodeRule<YamlBlockInBlockNode> for FormatYamlBlockInBlockNode {
    fn fmt_fields(&self, node: &YamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockInBlockNodeFields {
            properties,
            content,
        } = node.as_fields();

        write!(f, [properties.format()])?;

        // The parser can produce a node that has properties but no content and
        // no diagnostic, e.g. for `--- !!str` with its scalar on the next line
        let Ok(content) = content else {
            return Ok(());
        };

        if !properties.is_empty() {
            if matches!(
                &content,
                AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                    | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
            ) {
                write!(f, [hard_line_break()])?;
            } else {
                write!(f, [space()])?;
            }
        }

        write!(f, [content.format()])
    }
}
