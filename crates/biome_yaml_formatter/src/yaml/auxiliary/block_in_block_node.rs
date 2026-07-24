use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, YamlBlockInBlockNode, YamlBlockInBlockNodeFields, YamlSyntaxKind,
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
            let has_comments_between = properties
                .iter()
                .last()
                .is_some_and(|property| f.comments().has_trailing_comments(property.syntax()))
                || f.comments().has_leading_comments(content.syntax());
            let is_block_collection = matches!(
                &content,
                AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                    | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
            );
            let at_document_level = node
                .syntax()
                .parent()
                .is_some_and(|parent| parent.kind() == YamlSyntaxKind::YAML_DOCUMENT);

            // A block collection is already placed on an indented line by the
            // enclosing entry, and at the document level the content carries
            // no indentation, so a plain line break suffices for both
            if is_block_collection || (has_comments_between && at_document_level) {
                write!(f, [hard_line_break()])?;
            } else if has_comments_between {
                // A comment between the properties and a block scalar ends
                // the header line, so the scalar moves to its own line,
                // indented past the entry:
                //
                // ```yaml
                // key: !tag # comment
                //   |
                //     content
                // ```
                return write!(
                    f,
                    [indent(&format_args![hard_line_break(), content.format()])]
                );
            } else {
                write!(f, [space()])?;
            }
        }

        write!(f, [content.format()])
    }
}
