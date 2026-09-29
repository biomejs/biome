use biome_rowan::{TextRange, TextSize};
use std::borrow::Cow;

/// Replaces blockquote markers inside a Markdown HTML comment with spaces.
///
/// Markdown keeps continuation-line blockquote markers inside HTML block tokens.
/// Replacing only the marker byte preserves all source-relative offsets.
pub fn normalize_quote_prefixes(text: &str, quote_depth: usize) -> Cow<'_, str> {
    if quote_depth == 0 {
        return Cow::Borrowed(text);
    }

    let mut marker_offsets = Vec::new();
    let mut line_offset = 0;
    for line in text.split_inclusive('\n') {
        let bytes = line.as_bytes();
        let mut offset = 0;
        for _ in 0..quote_depth {
            while matches!(bytes.get(offset), Some(b' ' | b'\t')) {
                offset += 1;
            }
            if bytes.get(offset) != Some(&b'>') {
                break;
            }
            marker_offsets.push(line_offset + offset);
            offset += 1;
        }
        line_offset += line.len();
    }

    if marker_offsets.is_empty() {
        return Cow::Borrowed(text);
    }

    let mut bytes = text.as_bytes().to_vec();
    for offset in marker_offsets {
        if let Some(byte) = bytes.get_mut(offset) {
            *byte = b' ';
        }
    }
    match String::from_utf8(bytes) {
        Ok(text) => Cow::Owned(text),
        Err(_) => Cow::Borrowed(text),
    }
}

/// Returns the ranges of all HTML comments when `text` contains only comments
/// and whitespace.
///
/// The ranges are relative to `text`. Unterminated comments and text containing
/// any other HTML are rejected.
pub fn html_comment_ranges(text: &str) -> Option<Vec<TextRange>> {
    let mut ranges = Vec::new();
    let mut offset = 0;

    while offset < text.len() {
        let remaining = text.get(offset..)?;
        let whitespace_len = remaining
            .char_indices()
            .find_map(|(index, character)| (!character.is_whitespace()).then_some(index))
            .unwrap_or(remaining.len());
        offset += whitespace_len;

        if offset == text.len() {
            break;
        }

        let comment = text.get(offset..)?;
        if !comment.starts_with("<!--") {
            return None;
        }
        let end = comment.find("-->")? + 3;
        let start = TextSize::try_from(offset).ok()?;
        offset += end;
        let end = TextSize::try_from(offset).ok()?;
        ranges.push(TextRange::new(start, end));
    }

    (!ranges.is_empty()).then_some(ranges)
}

#[cfg(test)]
mod tests {
    use super::{html_comment_ranges, normalize_quote_prefixes};
    use biome_rowan::TextRange;

    #[test]
    fn recognizes_comment_only_text() {
        assert_eq!(
            html_comment_ranges(" <!-- first -->\n<!-- second --> "),
            Some(vec![
                TextRange::new(1.into(), 15.into()),
                TextRange::new(16.into(), 31.into())
            ])
        );
    }

    #[test]
    fn rejects_mixed_or_unterminated_html() {
        assert_eq!(html_comment_ranges("<!-- comment --> <div>"), None);
        assert_eq!(html_comment_ranges("<!-- comment"), None);
        assert_eq!(html_comment_ranges("  \n"), None);
    }

    #[test]
    fn normalizes_blockquote_prefixes_without_changing_offsets() {
        let text = "<!--\n> biome-ignore format: reason\n> -->";
        let normalized = normalize_quote_prefixes(text, 1);
        assert_eq!(normalized, "<!--\n  biome-ignore format: reason\n  -->");
        assert_eq!(normalized.len(), text.len());
    }
}
