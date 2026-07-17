use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockMapping, YamlBlockMappingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapping;
impl FormatNodeRule<YamlBlockMapping> for FormatYamlBlockMapping {
    fn fmt_fields(&self, node: &YamlBlockMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockMappingFields {
            mapping_start_token,
            entries,
            mapping_end_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_removed(&mapping_start_token?), // always empty, ignored
                entries.format(),
                format_removed(&mapping_end_token?), // always empty, ignored
            ]
        )
    }
}
