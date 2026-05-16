use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowMapImplicitEntry, YamlFlowMapImplicitEntryFields};
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

        write!(f, [key.format()])?;

        if let Some(colon_token) = colon_token {
            write!(f, [colon_token.format()])?;
        }

        if value.is_some() {
            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}
