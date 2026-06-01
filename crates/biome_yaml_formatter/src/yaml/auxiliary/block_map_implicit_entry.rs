use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, YamlBlockMapImplicitEntry,
    YamlBlockMapImplicitEntryFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [key.format(), colon_token.format()])?;

        if let Some(value) = value {
            let has_leading_comments = f.comments().has_leading_comments(value.syntax());
            if has_leading_comments || is_nested_block_collection(&value) {
                return write!(
                    f,
                    [indent(&format_args![hard_line_break(), value.format()])]
                );
            }

            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}

fn is_nested_block_collection(value: &AnyYamlBlockNode) -> bool {
    matches!(
        value,
        AnyYamlBlockNode::YamlBlockInBlockNode(node)
            if matches!(
                node.content(),
                Ok(
                    AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                        | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
                )
            )
    )
}
