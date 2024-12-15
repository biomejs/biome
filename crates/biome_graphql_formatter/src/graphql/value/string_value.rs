use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlStringValue, GraphqlStringValueFields, TextLen};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlStringValue;
impl FormatNodeRule<GraphqlStringValue> for FormatGraphqlStringValue {
    fn fmt_fields(&self, node: &GraphqlStringValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlStringValueFields {
            graphql_string_literal_token,
        } = node.as_fields();

        if node.is_block() {
            let token = graphql_string_literal_token?;
            let text_trimmed = token.text_trimmed();
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
                join.entry(&text("\"\"\""));
                join.entry(&hard_line_break());

                let mut start = token.text_trimmed_range().start();
                for line in trimmed_content.lines() {
                    if line.is_empty() || is_blank(line) {
                        // if the line is empty,
                        // write an empty line because two hardline breaks don't work
                        join.entry(&empty_line());
                        continue;
                    }
                    // Write the line with the minimum indentation level removed
                    // SAFETY: min_indent is always less than or equal to the length of the line
                    join.entry(&dynamic_text(&line[min_indent..], start));
                    start += line.text_len();

                    if line.is_empty() {
                        join.entry(&empty_line());
                    } else {
                        // Write a hard line break after each line
                        join.entry(&hard_line_break());
                    }
                }

                join.entry(&hard_line_break());
                // Write the closing triple quotes
                join.entry(&text("\"\"\""));
                join.finish()
            });

            write!(f, [format_replaced(&token, &content)])
        } else {
            write![f, [graphql_string_literal_token.format()]]
        }
    }
}

fn is_blank(line: &str) -> bool {
    line.bytes().all(|byte| byte.is_ascii_whitespace())
}
