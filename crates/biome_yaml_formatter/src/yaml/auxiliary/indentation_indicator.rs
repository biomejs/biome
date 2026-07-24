use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlIndentationIndicator, YamlIndentationIndicatorFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlIndentationIndicator;
impl FormatNodeRule<YamlIndentationIndicator> for FormatYamlIndentationIndicator {
    fn fmt_fields(
        &self,
        node: &YamlIndentationIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        let YamlIndentationIndicatorFields {
            indentation_indicator_token,
        } = node.as_fields();
        write!(f, [indentation_indicator_token.format()])
    }
}
