use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockMapImplicitEntry, YamlBlockMapImplicitEntryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockMapImplicitEntryFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [key.format(), colon_token.format()])?;

        if let Some(value) = value {
            write!(f, [space(), value.format()])?;
        }

        Ok(())
    }
}
