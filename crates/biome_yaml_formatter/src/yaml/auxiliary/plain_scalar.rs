use crate::prelude::*;
use crate::FormatYamlSyntaxToken;
use biome_formatter::trivia::FormatToken;
use biome_yaml_syntax::{YamlPlainScalar, YamlPlainScalarFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPlainScalar;
impl FormatNodeRule<YamlPlainScalar> for FormatYamlPlainScalar {
    fn fmt_fields(&self, node: &YamlPlainScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlPlainScalarFields { value_token } = node.as_fields();
        let value_token = value_token?;
        let value_text = value_token.text();
        let trimmed_value = value_text
            .trim_start_matches([' ', '\t', '\n', '\r'])
            .trim_end_matches([' ', '\t']);

        FormatYamlSyntaxToken.format_replaced(
            &value_token,
            &text(trimmed_value, value_token.text_range().start()),
            f,
        )
    }
}
