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

        if !own_properties.is_empty() {
            write!(f, [own_properties, space()])?;
        }

        write!(f, [content.format()])
    }
}
