use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockMapping, YamlBlockMappingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapping;
impl FormatNodeRule<YamlBlockMapping> for FormatYamlBlockMapping {
    fn fmt_fields(&self, node: &YamlBlockMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockMappingFields {
            mapping_start_token: _,
            entries,
            mapping_end_token: _,
        } = node.as_fields();

        write!(f, [entries.format()])
    }
}
