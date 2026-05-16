use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{
    AnyYamlJsonContent, AnyYamlMappingImplicitKey, YamlFlowMapImplicitEntry,
    YamlFlowMapImplicitEntryFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapImplicitEntry;
impl FormatNodeRule<YamlFlowMapImplicitEntry> for FormatYamlFlowMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        if let Some(colon_token) = colon_token {
            if key.as_ref().is_some_and(needs_explicit_key_indicator) {
                write!(f, [token("?"), space(), key.format(), colon_token.format()])?;
            } else {
                write!(f, [key.format(), colon_token.format()])?;
            }

            if value.is_some() {
                write!(f, [space(), value.format()])?;
            } else {
                write!(f, [space()])?;
            }
        } else if key.is_some() {
            write!(f, [token("?"), space(), key.format()])?;
        }

        Ok(())
    }
}

fn needs_explicit_key_indicator(key: &AnyYamlMappingImplicitKey) -> bool {
    match key {
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => matches!(
            node.content(),
            Ok(AnyYamlJsonContent::YamlFlowMapping(_) | AnyYamlJsonContent::YamlFlowSequence(_))
        ),
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(_) => false,
    }
}
