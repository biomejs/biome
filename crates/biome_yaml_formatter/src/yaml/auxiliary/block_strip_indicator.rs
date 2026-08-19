use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockStripIndicator, YamlBlockStripIndicatorFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockStripIndicator;
impl FormatNodeRule<YamlBlockStripIndicator> for FormatYamlBlockStripIndicator {
    fn fmt_fields(
        &self,
        node: &YamlBlockStripIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlBlockStripIndicatorFields { minus_token } = node.as_fields();
        write!(f, [minus_token.format()])
    }
}
