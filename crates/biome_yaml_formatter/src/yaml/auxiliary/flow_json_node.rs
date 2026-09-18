use crate::comments::FormatMiddleComments;
use crate::prelude::*;
use crate::utils::FormatProperties;
use biome_formatter::write;
use biome_yaml_syntax::{AnyYamlMappingImplicitKey, YamlFlowJsonNode, YamlFlowJsonNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowJsonNode;
impl FormatNodeRule<YamlFlowJsonNode> for FormatYamlFlowJsonNode {
    fn fmt_fields(&self, node: &YamlFlowJsonNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowJsonNodeFields {
            properties,
            content,
        } = node.as_fields();

        // Properties the parser flattened onto this node but that belong to
        // the enclosing block mapping are printed by that mapping's node
        // instead, so they stay on the mapping's line rather than moving
        // onto this key:
        //
        // ```yaml
        // ---
        // &mapping
        // &key [&item a, b, c]: value
        // ```
        let skipped = AnyYamlMappingImplicitKey::YamlFlowJsonNode(node.clone())
            .enclosing_mapping_property_count();
        let own_properties = FormatProperties::own(&properties, skipped);
        let has_own_properties = !own_properties.is_empty();

        write!(f, [own_properties])?;

        // A middle comment ends the properties' line, so the content moves
        // to the next one
        if f.comments().has_dangling_comments(node.syntax()) {
            write!(
                f,
                [FormatMiddleComments::new(node.syntax()), hard_line_break()]
            )?;
        } else if has_own_properties {
            write!(f, [space()])?;
        }

        write!(f, [content.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlFlowJsonNode,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comments are the node's middle comments, formatted
        // by `FormatMiddleComments`, which `fmt_fields` calls between the
        // properties and the content
        Ok(())
    }
}
