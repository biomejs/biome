use std::ops::Deref;

use biome_js_syntax::{JsSyntaxNode, JsSyntaxToken};
use biome_rowan::TriviaPieceKind;

/// Represents a normalised JSDoc comment.
///
/// Comments are trimmed and normalised to remove the trivia of the comment.
///
/// ## Example
///
/// Assuming the following JSDoc comment:
///
/// ```ts
/// /**
///  * Magic constant of fooness.
///  *
///  * For if you want more ways to write 1.
///  */
/// export const FOO = 1;
/// ```
///
/// The normalised representation will become:
/// `"Magic constant of fooness.\n\nFor if you want more ways to write 1."`.
///    
/// See https://jsdoc.app/ for the JSDoc reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsdocComment(String);

impl JsdocComment {
    /// Creates a normalised JSDoc comment from the given comment `text`.
    pub fn from_comment_text(text: &str) -> Self {
        debug_assert!(text.starts_with("/**") && text.ends_with("*/"));

        let mut result = text[3..text.len() - 2]
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                trimmed.strip_prefix('*').map_or(trimmed, str::trim_start)
            })
            .fold(String::new(), |mut result, line| {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(line);
                result
            });

        // Trim trailing newlines.
        while result.ends_with('\n') {
            result.truncate(result.len() - 1);
        }

        Self(result)
    }

    /// Returns whether the given text is a valid JSDoc comment.
    ///
    /// JSDoc comments must start with exactly `/**` and with `*/`. Either more
    /// or less asterisks in the opening are ignored.
    pub fn text_is_jsdoc_comment(text: &str) -> bool {
        text.len() >= 6
            && text.starts_with("/**")
            && text.as_bytes().get(3).is_some_and(|c| *c != b'*')
            && text.ends_with("*/")
    }

    /// Iterates all JSDoc comments preceding the node.
    pub fn for_each<F>(node: &JsSyntaxNode, mut func: F)
    where
        F: FnMut(&str),
    {
        if let Some(token) = node.first_token() {
            for trivia in token.leading_trivia().pieces() {
                match trivia.kind() {
                    TriviaPieceKind::MultiLineComment | TriviaPieceKind::SingleLineComment => {
                        let text = trivia.text();
                        if Self::text_is_jsdoc_comment(text) {
                            func(text)
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl AsRef<str> for JsdocComment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for JsdocComment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl TryFrom<JsSyntaxNode> for JsdocComment {
    type Error = ();

    fn try_from(node: JsSyntaxNode) -> Result<Self, Self::Error> {
        Self::try_from(&node)
    }
}

impl TryFrom<&JsSyntaxNode> for JsdocComment {
    type Error = ();

    fn try_from(node: &JsSyntaxNode) -> Result<Self, Self::Error> {
        node.first_token().ok_or(()).and_then(Self::try_from)
    }
}

impl TryFrom<JsSyntaxToken> for JsdocComment {
    type Error = ();
    fn try_from(token: JsSyntaxToken) -> Result<Self, Self::Error> {
        token
            .leading_trivia()
            .pieces()
            .rev()
            .find_map(|trivia| match trivia.kind() {
                TriviaPieceKind::MultiLineComment | TriviaPieceKind::SingleLineComment => {
                    let text = trivia.text();
                    Self::text_is_jsdoc_comment(text).then(|| Self::from_comment_text(text))
                }
                _ => None,
            })
            .ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_is_jsdoc_comment() {
        assert!(JsdocComment::text_is_jsdoc_comment("/** yes */"));
        assert!(JsdocComment::text_is_jsdoc_comment("/**\n* yes\n*/"));
        assert!(JsdocComment::text_is_jsdoc_comment("/**\nyes\n*/"));
        assert!(JsdocComment::text_is_jsdoc_comment("/**\n** yes\n*/"));
        assert!(JsdocComment::text_is_jsdoc_comment("/** */"));

        assert!(!JsdocComment::text_is_jsdoc_comment("/* no */"));
        assert!(!JsdocComment::text_is_jsdoc_comment("/*** no */"));
        assert!(!JsdocComment::text_is_jsdoc_comment("/***/"));
        assert!(!JsdocComment::text_is_jsdoc_comment("/**/"));
    }
}
