use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowMapExplicitEntry, YamlFlowMapExplicitEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapExplicitEntry;
impl FormatNodeRule<YamlFlowMapExplicitEntry> for FormatYamlFlowMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlFlowMapExplicitEntryFields {
            question_mark_token,
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [question_mark_token.format(), space(), key.format()])?;

        if let Some(colon_token) = colon_token {
            write!(f, [colon_token.format()])?;
        }

        if value.is_some() {
            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}
