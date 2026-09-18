use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockKeepIndicator, YamlBlockKeepIndicatorFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockKeepIndicator;
impl FormatNodeRule<YamlBlockKeepIndicator> for FormatYamlBlockKeepIndicator {
    fn fmt_fields(&self, node: &YamlBlockKeepIndicator, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockKeepIndicatorFields { plus_token } = node.as_fields();
        write!(f, [plus_token.format()])
    }
}
