//! Helpers for the MDX constructs of the Markdown syntax tree.

use crate::MdxExpressionContent;
use biome_rowan::{SyntaxResult, TextRange, TextSize};

impl MdxExpressionContent {
    /// Returns the ranges of the comments in this expression, when the
    /// expression holds only JavaScript comments and whitespace, such as
    /// `{/* biome-ignore lint: reason */}`. Returns `None` for any other
    /// content.
    pub fn comment_ranges(&self) -> SyntaxResult<Option<Vec<TextRange>>> {
        let token = self.value_token()?;
        let start = token.text_trimmed_range().start();
        Ok(comment_ranges(token.text_trimmed())
            .map(|ranges| ranges.into_iter().map(|range| range + start).collect()))
    }
}

/// Returns the ranges of the comments in `text`, relative to its start, when
/// `text` holds at least one JavaScript comment and nothing else but
/// whitespace.
fn comment_ranges(text: &str) -> Option<Vec<TextRange>> {
    let mut ranges = Vec::new();
    let mut offset = 0;
    loop {
        let rest = &text[offset..];
        let comment = rest.trim_start();
        offset += rest.len() - comment.len();
        if comment.is_empty() {
            break;
        }
        let length = if let Some(body) = comment.strip_prefix("/*") {
            body.find("*/")? + 4
        } else {
            let body = comment.strip_prefix("//")?;
            body.find(['\n', '\r']).map_or(comment.len(), |end| end + 2)
        };
        ranges.push(TextRange::at(
            TextSize::from(offset as u32),
            TextSize::from(length as u32),
        ));
        offset += length;
    }
    (!ranges.is_empty()).then_some(ranges)
}

#[cfg(test)]
mod tests {
    use super::comment_ranges;
    use biome_rowan::TextRange;

    #[test]
    fn comments_and_whitespace_are_comment_only() {
        assert_eq!(
            comment_ranges(" /* a */ // b\n"),
            Some(vec![
                TextRange::new(1.into(), 8.into()),
                TextRange::new(9.into(), 13.into())
            ])
        );
    }

    #[test]
    fn code_or_nothing_is_not_comment_only() {
        assert_eq!(comment_ranges("a /* b */"), None);
        assert_eq!(comment_ranges("/* a */ b"), None);
        assert_eq!(comment_ranges("   "), None);
        assert_eq!(comment_ranges("/* unclosed"), None);
    }
}
