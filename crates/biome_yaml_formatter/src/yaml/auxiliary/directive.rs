use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::{TextRange, TextSize};
use biome_yaml_syntax::{YamlDirective, YamlDirectiveFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDirective;
impl FormatNodeRule<YamlDirective> for FormatYamlDirective {
    fn fmt_fields(&self, node: &YamlDirective, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlDirectiveFields { value_token } = node.as_fields();
        let value_token = value_token?;

        // Normalize the directive parameters with single spaces
        let value = value_token.text_trimmed();
        write!(
            f,
            [format_replaced(
                &value_token,
                &format_with(|f| {
                    let start = value_token.text_trimmed_range().start();
                    let mut join = f.join_with(space());
                    let mut rest = value;
                    loop {
                        rest = rest.trim_start();
                        if rest.is_empty() {
                            break;
                        }
                        let offset = value.len() - rest.len();
                        let word_len = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
                        let range = TextRange::at(
                            start + TextSize::from(offset as u32),
                            TextSize::from(word_len as u32),
                        );
                        join.entry(&located_token_text(&value_token, range));
                        rest = &rest[word_len..];
                    }
                    join.finish()
                })
            )]
        )
    }
}
