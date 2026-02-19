use crate::FormatGraphqlSyntaxToken;
use crate::prelude::*;
use biome_formatter::trivia::FormatToken;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlStringValue, GraphqlStringValueFields};
use biome_rowan::TextSize;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlStringValue;
impl FormatNodeRule<GraphqlStringValue> for FormatGraphqlStringValue {
    fn fmt_fields(&self, node: &GraphqlStringValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlStringValueFields {
            graphql_string_literal_token,
        } = node.as_fields();

        if node.is_block() {
            let string_token = graphql_string_literal_token?;
            let text_trimmed = string_token.text_trimmed();
            // Extract the content of the block string
            // by removing the triple quotes
            let raw_content = &text_trimmed[3..text_trimmed.len() - 3];

            // Trim leading newline characters from the content
            // We can't use trim here because it will remove any whitespace
            // But leading whitespace is significant in block strings to calculate the indentation level
            // SAFETY:
            // We check that the string starts with triple quotes and the parser guarantees that ends with triple quotes.
            let trimmed_content = raw_content.trim_start_matches(['\n', '\r']).trim_end();

            // Find the minimum indentation level of non-empty lines
            let min_indent = trimmed_content
                .lines()
                .filter(|line| !line.trim().is_empty()) // Ignore empty lines
                .map(|line| line.bytes().take_while(|b| b.is_ascii_whitespace()).count())
                .min()
                .unwrap_or(0);

            let content = format_with(|f| {
                let mut join = f.join();
                // Write the opening triple quotes
                join.entry(&token("\"\"\""));
                join.entry(&hard_line_break());

                let start_offset = string_token.text_trimmed_range().start();
                // raw_content starts 3 bytes in (past """); count only leading \n/\r stripped
                // to compute the correct base offset for source position mapping.
                let leading_stripped =
                    raw_content.len() - raw_content.trim_start_matches(['\n', '\r']).len();
                let base_offset = start_offset + TextSize::from((3 + leading_stripped) as u32);
                let mut current_pos: u32 = 0;

                for line in trimmed_content.lines() {
                    if is_blank(line) {
                        // if the line is empty,
                        // write an empty line because two hardline breaks don't work
                        join.entry(&empty_line());
                    } else {
                        // Write the line with the minimum indentation level removed
                        // SAFETY: min_indent is always less than or equal to the length of the line
                        let start = base_offset + TextSize::from(current_pos + min_indent as u32);
                        join.entry(&text(&line[min_indent..], start));
                        // Write a hard line break after each line
                        join.entry(&hard_line_break());
                    }

                    // Update the position for the next line
                    // We need to account for the line length plus the newline character
                    current_pos += line.len() as u32;
                    // Skip the newline character (\n or \r\n)
                    if current_pos < trimmed_content.len() as u32 {
                        if trimmed_content.as_bytes().get(current_pos as usize) == Some(&b'\r') {
                            current_pos += 1;
                        }
                        if trimmed_content.as_bytes().get(current_pos as usize) == Some(&b'\n') {
                            current_pos += 1;
                        }
                    }
                }

                join.entry(&hard_line_break());
                // Write the closing triple quotes
                join.entry(&token("\"\"\""));
                join.finish()
            });

            FormatGraphqlSyntaxToken.format_replaced(&string_token, &content, f)
        } else {
            write![f, [graphql_string_literal_token.format()]]
        }
    }
}

fn is_blank(line: &str) -> bool {
    line.bytes().all(|byte| byte.is_ascii_whitespace())
}
