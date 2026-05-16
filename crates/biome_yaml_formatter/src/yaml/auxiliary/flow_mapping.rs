use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::{YamlFlowMapping, YamlFlowMappingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapping;
impl FormatNodeRule<YamlFlowMapping> for FormatYamlFlowMapping {
    fn fmt_fields(&self, node: &YamlFlowMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowMappingFields {
            l_curly_token,
            entries,
            r_curly_token,
        } = node.as_fields();

        if entries.is_empty() {
            write!(f, [l_curly_token.format(), r_curly_token.format()])
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    group(&format_args![
                        soft_block_indent_with_maybe_space(&entries.format(), true),
                        soft_line_break_or_space(),
                    ]),
                    r_curly_token.format()
                ]
            )
        }
    }
}
