use crate::prelude::*;
use crate::utils::FormatProperties;
use biome_formatter::write;
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
        let own_properties = FormatProperties::own(&properties, skipped);
        let has_own_properties = !own_properties.is_empty();

        write!(f, [own_properties])?;

        if let Some(content) = content {
            // The content joins the properties' line even when the source
            // had a line break between them: `!!str\nfoo` becomes
            // `!!str foo`
            if has_own_properties {
                write!(f, [space()])?;
            }

            write!(f, [content.format()])?;
        }

        Ok(())
    }
}
