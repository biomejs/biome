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
            let parent_kind = node.syntax().parent().map(|parent| parent.kind());

            if is_block_collection {
                // The properties stay on the line the node started on and
                // the collection goes below, indented past a mapping key:
                //
                // ```yaml
                // key: &anchor
                //   a: 1
                // ```
                //
                // At the document level and in a sequence entry the
                // collection needs no indentation of its own: it sits at
                // column zero, or is aligned by the enclosing entry
                if parent_kind == Some(YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY) {
                    return write!(
                        f,
                        [indent(&format_args![hard_line_break(), content.format()])]
                    );
                }
                write!(f, [hard_line_break()])?;
            } else if has_comments_between {
                if parent_kind == Some(YamlSyntaxKind::YAML_DOCUMENT) {
                    // At the document level the scalar's header carries no
                    // indentation
                    write!(f, [hard_line_break()])?;
                } else {
                    // A comment between the properties and a block scalar
                    // ends the header line, so the scalar moves to its own
                    // line, indented past the entry:
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
                }
            } else {
                write!(f, [space()])?;
            }
        }

        write!(f, [content.format()])
    }
}
