use crate::comments::{FormatCommentsSlice, FormatMiddleComments};
use crate::prelude::*;
use crate::utils::FormatProperties;
use biome_formatter::write;
use biome_rowan::AstNodeList;
use biome_yaml_syntax::{AnyYamlMappingImplicitKey, YamlFlowYamlNode, YamlFlowYamlNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowYamlNode;
impl FormatNodeRule<YamlFlowYamlNode> for FormatYamlFlowYamlNode {
    fn fmt_fields(&self, node: &YamlFlowYamlNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowYamlNodeFields {
            properties,
            content,
        } = node.as_fields();

        // Properties the parser flattened onto this node but that belong to
        // the enclosing block mapping are printed by that mapping's node
        // instead. Printing them here would move them onto the key for real:
        //
        // ```yaml
        // - !circle
        //   center: 1
        // ```
        //
        // must not become `- !circle center: 1`, where the tag applies to
        // the key instead of the mapping
        let skipped = AnyYamlMappingImplicitKey::YamlFlowYamlNode(node.clone())
            .enclosing_mapping_property_count();
        let own_properties = properties.iter().skip(skipped);
        let has_own_properties = own_properties.clone().next().is_some();

        write!(f, [FormatProperties(own_properties)])?;

        let has_middle_comments = f.comments().has_dangling_comments(node.syntax());

        if let Some(content) = &content {
            // A middle comment ends the properties' line, so the content
            // moves to the next one:
            //
            // ```yaml
            // !!str # comment
            // hello
            // ```
            //
            // Without comments the content joins the properties' line even
            // when the source had a line break between them: `!!str\nfoo`
            // becomes `!!str foo`
            if has_middle_comments {
                write!(
                    f,
                    [FormatMiddleComments::new(node.syntax()), hard_line_break()]
                )?;
            } else if has_own_properties {
                write!(f, [space()])?;
            }

            write!(f, [content.format()])?;
        } else if has_own_properties {
            // The space that would separate the content is printed even
            // though there is none. Before a line break it is dropped, but
            // in a flow collection it survives, matching Prettier:
            //
            // ```yaml
            // { foo: !!str , !!str : bar }
            // ```
            write!(f, [space()])?;
        }

        if content.is_none() && has_middle_comments {
            // With no content to move, the comments stay on the line
            let comments = f.comments().clone();
            write!(
                f,
                [FormatCommentsSlice {
                    comments: comments.dangling_comments(node.syntax()),
                    inline_first: true
                }]
            )?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowYamlNode,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments are the node's middle comments, formatted
        // by `FormatMiddleComments` (or `FormatCommentsSlice` when the node
        // has no content), which `fmt_fields` calls between the properties
        // and the content
        Ok(())
    }
}
