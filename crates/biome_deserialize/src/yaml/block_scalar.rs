//! The content of [block scalars](https://yaml.org/spec/1.2.2/#81-block-scalar-styles): literal
//! (`|`) and folded (`>`).
use biome_rowan::{AstNode, Direction};
use biome_yaml_syntax::{
    AnyYamlBlockHeader, AnyYamlBlockScalar, YamlBlockMapImplicitEntry, YamlSyntaxKind,
    YamlSyntaxNode, YamlSyntaxToken,
};
use std::iter::repeat_n;

/// What happens to the final line break and the trailing empty lines of a block scalar.
///
/// See <https://yaml.org/spec/1.2.2/#8112-block-chomping-indicator>.
#[derive(Clone, Copy)]
enum Chomping {
    /// `-`: they're removed.
    Strip,
    /// The default: only the final line break is kept.
    Clip,
    /// `+`: they're kept.
    Keep,
}

/// Returns the content of a literal or folded block scalar.
///
/// See <https://yaml.org/spec/1.2.2/#81-block-scalar-styles>.
pub(super) fn block_scalar_value(scalar: &AnyYamlBlockScalar) -> Option<String> {
    let (content, is_folded) = match scalar {
        AnyYamlBlockScalar::YamlLiteralScalar(scalar) => (scalar.content(), false),
        AnyYamlBlockScalar::YamlFoldedScalar(scalar) => (scalar.content(), true),
    };
    let token = content.ok()?.value_token().ok()?;

    let mut chomping = Chomping::Clip;
    let mut indentation_indicator: Option<usize> = None;
    for header in scalar.headers() {
        match header {
            AnyYamlBlockHeader::YamlBlockStripIndicator(_) => chomping = Chomping::Strip,
            AnyYamlBlockHeader::YamlBlockKeepIndicator(_) => chomping = Chomping::Keep,
            AnyYamlBlockHeader::YamlIndentationIndicator(indicator) => {
                indentation_indicator = indicator
                    .indentation_indicator_token()
                    .ok()?
                    .text_trimmed()
                    .parse()
                    .ok();
            }
            AnyYamlBlockHeader::YamlBogusBlockHeader(_) => {}
        }
    }

    // The lexer consumes the trivia that follows the scalar at once, so its trailing blank
    // lines are in the leading trivia of a single token. It can be a token without text that
    // ends a block collection.
    let next_token = std::iter::successors(token.next_token(), YamlSyntaxToken::next_token)
        .find(|token| !token.text().is_empty());
    let lines = BlockScalarLines::new(
        token.text_trimmed(),
        next_token.as_ref().map_or("", leading_blank_lines),
    );
    let indent = match indentation_indicator {
        Some(indicator) => parent_indentation(scalar.syntax()) + indicator,
        None => lines.detect_indentation(),
    };

    let mut value = String::new();
    // The empty lines since the previous line with content
    let mut empty_lines = 0;
    // The line breaks since the last line with content, which chomping applies to
    let mut trailing_line_breaks = 0;
    // Whether the previous line with content is more indented, or `None` before the first one
    let mut is_previous_more_indented = None;
    let mut lines = lines.iter().peekable();
    while let Some(line) = lines.next() {
        let has_line_break = lines.peek().is_some();
        let spaces = line.len() - line.trim_start_matches(' ').len();
        let content = &line[spaces.min(indent)..];
        if spaces < indent || content.is_empty() {
            // A less indented line with content, such as a comment, ends the scalar. The lexer
            // only ends the scalars that are inside a block collection there.
            if !content.trim_start_matches('\t').is_empty() {
                break;
            }
            empty_lines += 1;
            trailing_line_breaks += usize::from(has_line_break);
            continue;
        }
        let is_more_indented = content.starts_with([' ', '\t']);
        let line_feeds = match is_previous_more_indented {
            None => empty_lines,
            // Folding turns a line break between lines that aren't more indented into a space,
            // unless empty lines follow it, which become line feeds.
            // See <https://yaml.org/spec/1.2.2/#813-folded-style>.
            Some(false) if is_folded && !is_more_indented => {
                if empty_lines == 0 {
                    value.push(' ');
                }
                empty_lines
            }
            Some(_) => empty_lines + 1,
        };
        value.extend(repeat_n('\n', line_feeds));
        value.push_str(content);
        empty_lines = 0;
        trailing_line_breaks = usize::from(has_line_break);
        is_previous_more_indented = Some(is_more_indented);
    }

    match chomping {
        Chomping::Strip => {}
        Chomping::Clip => {
            if is_previous_more_indented.is_some() && trailing_line_breaks > 0 {
                value.push('\n');
            }
        }
        Chomping::Keep => value.extend(repeat_n('\n', trailing_line_breaks)),
    }
    Some(value)
}

/// The lines of a block scalar, which the lexer splits between the content token and the
/// trivia that follows it.
///
/// Every line is followed by a line break, except the last line of the source.
#[derive(Clone, Copy)]
struct BlockScalarLines<'a> {
    /// The lines of the content token.
    content: &'a str,
    /// The blank lines that follow the content token, which start with the line break of its
    /// last line. They matter for the [Chomping::Keep] indicator, and they can be more indented
    /// than the content.
    trailing: &'a str,
}

impl<'a> BlockScalarLines<'a> {
    fn new(content: &'a str, trailing: &'a str) -> Self {
        // The line break that ends the header starts the content token, unless it's empty
        if content.is_empty() {
            Self {
                content: without_line_break(trailing),
                trailing: "",
            }
        } else {
            Self {
                content: without_line_break(content),
                trailing,
            }
        }
    }

    fn iter(self) -> impl Iterator<Item = &'a str> {
        split_lines(self.content).chain(split_lines(self.trailing).skip(1))
    }

    /// Returns the number of leading spaces of the first line with content or, if there is
    /// none, the length of the longest line.
    ///
    /// See <https://yaml.org/spec/1.2.2/#8111-block-indentation-indicator>.
    fn detect_indentation(self) -> usize {
        self.iter()
            .find_map(|line| {
                let content = line.trim_start_matches(' ');
                (!content.is_empty()).then_some(line.len() - content.len())
            })
            .unwrap_or_else(|| self.iter().map(str::len).max().unwrap_or(0))
    }
}

/// Returns `text` without its leading line break.
fn without_line_break(text: &str) -> &str {
    text.strip_prefix("\r\n")
        .or_else(|| text.strip_prefix(['\n', '\r']))
        .unwrap_or(text)
}

/// Returns the blank lines of the leading trivia of `token`, up to the last line break before
/// a comment.
fn leading_blank_lines(token: &YamlSyntaxToken) -> &str {
    let leading_trivia_len = token.text_trimmed_range().start() - token.text_range().start();
    let trivia = &token.text()[..usize::from(leading_trivia_len)];
    // Comments are the only trivia that contain `#`
    let blank_lines = &trivia[..trivia.find('#').unwrap_or(trivia.len())];
    // Leaves out the indentation of the next line
    &blank_lines[..blank_lines.rfind(['\n', '\r']).map_or(0, |index| index + 1)]
}

/// Splits `text` at its line breaks.
fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = Some(text);
    std::iter::from_fn(move || {
        let text = rest?;
        match text.find(['\n', '\r']) {
            Some(index) => {
                let line_break_len = if text[index..].starts_with("\r\n") {
                    2
                } else {
                    1
                };
                rest = Some(&text[index + line_break_len..]);
                Some(&text[..index])
            }
            None => {
                rest = None;
                Some(text)
            }
        }
    })
}

/// Returns the indentation of the block mapping or sequence that contains `node`,
/// which is 0 at the top level.
fn parent_indentation(node: &YamlSyntaxNode) -> usize {
    let Some(entry) = node.ancestors().find(|ancestor| {
        matches!(
            ancestor.kind(),
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY
                | YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_ENTRY
                | YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY
        )
    }) else {
        return 0;
    };
    let first_token =
        match YamlBlockMapImplicitEntry::cast_ref(&entry).and_then(|entry| entry.key()) {
            // The parser attaches the properties of a block mapping, which can be on previous lines,
            // to its first key. Each property is a single token.
            Some(key) => key
                .syntax()
                .descendants_tokens(Direction::Next)
                .nth(key.enclosing_mapping_property_count()),
            None => entry.first_token(),
        };
    first_token.map_or(0, |token| column(&token))
}

/// Returns the column, in bytes, at which `token` starts.
fn column(token: &YamlSyntaxToken) -> usize {
    /// Returns the length of `text` after its last line break, or `Err` with the length of
    /// `text` if it has none.
    fn len_after_line_break(text: &str) -> Result<usize, usize> {
        text.rfind(['\n', '\r'])
            .map(|index| text.len() - index - 1)
            .ok_or(text.len())
    }

    let mut column = match len_after_line_break(token.leading_trivia().text()) {
        Ok(len) => return len,
        Err(len) => len,
    };
    let mut previous = token.prev_token();
    while let Some(token) = previous {
        match len_after_line_break(token.text()) {
            Ok(len) => return column + len,
            Err(len) => column += len,
        }
        previous = token.prev_token();
    }
    column
}
