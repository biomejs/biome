use crate::comments::{FormatCommentsSlice, FormatMiddleComments};
use crate::prelude::*;
use crate::utils::FormatProperties;
use biome_formatter::write;
use biome_rowan::AstNodeList;
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

        // The properties of a block mapping that the parser attached to the
        // mapping's first key are printed here, up on the mapping's own line
        let pulled_up = node
            .properties_on_first_key()
            .into_iter()
            .flat_map(|(key_properties, count)| key_properties.iter().take(count));
        let effective_properties = properties.iter().chain(pulled_up);
        let has_properties = effective_properties.clone().next().is_some();

        write!(f, [FormatProperties(effective_properties)])?;

        let has_middle_comments = f.comments().has_dangling_comments(node.syntax());

        // The parser can produce a node that has properties but no content and
        // no diagnostic, e.g. for `--- !!str` with its scalar on the next line
        let Ok(content) = content else {
            // With no content to move, the comments stay on the line
            if has_middle_comments {
                let comments = f.comments().clone();
                write!(
                    f,
                    [FormatCommentsSlice {
                        comments: comments.dangling_comments(node.syntax()),
                        inline_first: true
                    }]
                )?;
            }
            return Ok(());
        };

        if has_properties {
            let is_block_collection = matches!(
                &content,
                AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                    | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
            );
            let parent_kind = node.syntax().parent().map(|parent| parent.kind());

            // The middle comments and the content below the properties: a
            // single comment joins the properties' line, a group goes onto
            // its own lines above the content
            let body = format_with(|f| {
                if has_middle_comments {
                    write!(f, [FormatMiddleComments::new(node.syntax())])?;
                }
                write!(f, [hard_line_break(), content.format()])
            });

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
                    return write!(f, [indent(&body)]);
                }
                return write!(f, [body]);
            } else if has_middle_comments {
                if parent_kind == Some(YamlSyntaxKind::YAML_DOCUMENT) {
                    // At the document level the scalar's header carries no
                    // indentation
                    return write!(f, [body]);
                }
                // A comment between the properties and a block scalar ends
                // the header line, so the scalar moves to its own line,
                // indented past the entry:
                //
                // ```yaml
                // key: !tag # comment
                //   |
                //     content
                // ```
                return write!(f, [indent(&body)]);
            } else {
                write!(f, [space()])?;
            }
        }

        write!(f, [content.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlBlockInBlockNode,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments are the node's middle comments, formatted
        // by `FormatMiddleComments` (or `FormatCommentsSlice` when the node
        // has no content), which `fmt_fields` calls between the properties
        // and the content
        Ok(())
    }
}
