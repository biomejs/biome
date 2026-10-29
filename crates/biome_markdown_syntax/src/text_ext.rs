use crate::{MdEntityReference, MdTextual};
use biome_rowan::SyntaxResult;

impl MdTextual {
    /// Returns textual content with CommonMark backslash escapes resolved.
    ///
    /// Character references are represented by [`MdEntityReference`] nodes and
    /// are therefore not decoded by this method.
    pub fn unescaped_text(&self) -> SyntaxResult<String> {
        Ok(process_escapes(self.value_token()?.text_trimmed()))
    }

    pub fn is_empty_and_not_newline(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text().trim().is_empty() && !self.is_newline()?)
    }

    pub fn is_empty(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text().trim().is_empty() || self.is_newline()?)
    }

    pub fn is_newline(&self) -> SyntaxResult<bool> {
        let token = self.value_token()?;
        Ok(token.text() == "\n" || token.text() == "\r" || token.text() == "\r\n")
    }
}

impl MdEntityReference {
    /// Returns the character represented by this Markdown entity reference.
    ///
    /// Invalid named references remain unchanged. Numeric references use the
    /// HTML replacement rules, including U+FFFD for a null reference.
    pub fn decoded_text(&self) -> SyntaxResult<String> {
        let token = self.value_token()?;
        Ok(decode_entity(token.text()).unwrap_or_else(|| token.text().to_string()))
    }
}

/// Removes backslashes before ASCII punctuation from one textual token.
///
/// Entity references are separate CST nodes, so this pass must not decode text
/// exposed by removing an escape.
fn process_escapes(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut characters = text.chars().peekable();

    while let Some(character) = characters.next() {
        if character == '\\'
            && let Some(&escaped) = characters.peek()
            && is_escapable(escaped)
        {
            result.push(escaped);
            characters.next();
        } else {
            result.push(character);
        }
    }

    result
}

/// Resolves escapes and entities in syntax fields that are not parsed as
/// ordinary inline Markdown, such as fenced-block info strings.
///
/// The single pass prevents an escaped ampersand from exposing an entity that
/// would then be decoded.
pub(crate) fn unescape_markdown(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut index = 0usize;

    while index < text.len() {
        let Some(character) = text[index..].chars().next() else {
            break;
        };
        if character == '\\'
            && let Some(escaped) = text[index + 1..].chars().next()
            && escaped.is_ascii_punctuation()
        {
            output.push(escaped);
            index += 1 + escaped.len_utf8();
            continue;
        }
        if character == '&'
            && let Some(semicolon) = text[index..].find(';')
            && semicolon <= 32
        {
            let end = index + semicolon + 1;
            let entity = &text[index..end];
            if let Some(decoded) = decode_entity(entity) {
                output.push_str(&decoded);
                index = end;
                continue;
            }
        }
        if character != '\r' {
            output.push(character);
        }
        index += character.len_utf8();
    }

    output
}

/// Decodes one complete semicolon-terminated character reference.
///
/// Named references require an exact entity-table match. Numeric references use
/// HTML replacement rules for invalid code points.
fn decode_entity(entity: &str) -> Option<String> {
    if !entity.starts_with('&') || !entity.ends_with(';') {
        return None;
    }
    if let Some(digits) = entity
        .strip_prefix("&#x")
        .or_else(|| entity.strip_prefix("&#X"))
        .and_then(|value| value.strip_suffix(';'))
    {
        if digits.is_empty()
            || digits.len() > 6
            || !digits.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return None;
        }
        return Some(htmlize::unescape(entity).into_owned());
    }
    if let Some(digits) = entity
        .strip_prefix("&#")
        .and_then(|value| value.strip_suffix(';'))
    {
        if digits.is_empty()
            || digits.len() > 7
            || !digits.bytes().all(|byte| byte.is_ascii_digit())
        {
            return None;
        }
        return Some(htmlize::unescape(entity).into_owned());
    }
    let decoded = htmlize::ENTITIES.get(entity.as_bytes())?;
    std::str::from_utf8(decoded).ok().map(ToString::to_string)
}

/// Applies CommonMark whitespace normalization to code-span content.
///
/// Line endings become spaces. One leading and trailing space is removed when
/// both are present and the span contains a non-space character.
pub(crate) fn normalize_code_span(text: &str) -> String {
    let text = text.replace("\r\n", " ").replace(['\r', '\n'], " ");
    if text.starts_with(' ')
        && text.ends_with(' ')
        && text.len() > 2
        && text.chars().any(|character| character != ' ')
    {
        text[1..text.len() - 1].to_string()
    } else {
        text
    }
}

fn is_escapable(character: char) -> bool {
    character.is_ascii_punctuation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_escapes_without_reprocessing_entities() {
        assert_eq!(process_escapes(r"\*text\*"), "*text*");
        assert_eq!(unescape_markdown(r"\&amp;"), "&amp;");
        assert_eq!(unescape_markdown("&#65; &amp;"), "A &");
    }

    #[test]
    fn normalizes_code_span_whitespace() {
        assert_eq!(normalize_code_span(" code "), "code");
        assert_eq!(normalize_code_span("  "), "  ");
        assert_eq!(normalize_code_span("a\r\nb"), "a b");
    }

    #[test]
    fn decodes_character_references() {
        assert_eq!(decode_entity("&amp;"), Some("&".to_string()));
        assert_eq!(decode_entity("&#x41;"), Some("A".to_string()));
        assert_eq!(decode_entity("&not-a-reference;"), None);
        assert_eq!(decode_entity("&notanentity;"), None);
        assert_eq!(decode_entity("&#12foo;"), None);
    }
}
