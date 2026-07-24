use crate::prelude::*;
use biome_formatter::write;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, declare_node_union};
use biome_yaml_syntax::{
    AnyYamlBlockHeader, YamlBlockContent, YamlBlockContentFields, YamlBlockHeaderList,
    YamlFoldedScalar, YamlLiteralScalar, YamlSyntaxKind,
};

/// Formats the content of a literal (`|`) or folded (`>`) block scalar.
///
/// The text of the lines is preserved as is, but their indentation is
/// replaced: the original base indentation (the indentation of the first
/// non-blank line, or the one given by an explicit indentation indicator) is
/// stripped from every line and the formatter's own indentation takes its
/// place. Any indentation beyond the base is content and is preserved.
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockContent;

impl FormatNodeRule<YamlBlockContent> for FormatYamlBlockContent {
    fn fmt_fields(&self, node: &YamlBlockContent, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockContentFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let (chomping, indicator) = parent_headers(node);

        let token_text = value_token.text_trimmed();

        let lines = ContentLines::new(token_text);
        let ends_with_break = lines.ends_with_break();
        // The first line of the token is the tail of the header line; the
        // content starts after its line break
        let lines = lines.skip(1);

        let stats = ContentStats::new(lines.clone());

        let kept_lines = match chomping {
            // The line break terminating the last line is printed by the
            // enclosing structure, so the line it opens isn't content
            Chomping::Keep => stats.count.saturating_sub(usize::from(ends_with_break)),
            // Trailing blank lines are dropped
            Chomping::Clip | Chomping::Strip => stats.trimmed_count,
        };

        // The number of block collections the node is nested in, which the
        // absolute indentation of explicitly indented content is computed from
        let ancestors = node
            .syntax()
            .ancestors()
            .skip(1)
            .filter(|ancestor| BLOCK_COLLECTIONS.contains(ancestor.kind()))
            .count();
        let base_indent = match indicator {
            Some(indicator) => indicator.saturating_sub(1).saturating_add(ancestors),
            None => stats.first_indent,
        };

        // FIXME: A non-empty line that is indented less than the base ends
        // the scalar per the spec, but the lexer includes such lines
        // (trailing comment lines, in practice) in the content token.
        // Re-indenting them would promote them to actual scalar content, so
        // the content is kept exactly as is for now. Once the lexer ends the
        // scalar at such lines, this fallback can be removed
        if stats.min_indent < base_indent {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let is_last = closes_last_document(node);
        let content = format_with(|f| {
            let state = std::cell::Cell::new(LineState::default());
            for line in lines.clone().take(kept_lines) {
                write!(
                    f,
                    [FormatContentLine {
                        line,
                        base_indent,
                        state: &state
                    }]
                )?;
            }

            // With keep chomping, the trailing blank lines of the scalar
            // belong to its content. When another node follows, the parser
            // leaves them in the leading trivia of the next token, so they
            // are recovered from there
            if chomping == Chomping::Keep
                && !token_text.is_empty()
                && !ends_with_break
                && let Some(next_token) = value_token.next_token()
            {
                let mut pending: Option<biome_rowan::SyntaxTriviaPiece<_>> = None;
                let mut first = true;
                for piece in next_token.leading_trivia().pieces() {
                    if piece.is_newline() {
                        // The first line break terminates the last content
                        // line of the token, so it doesn't open a blank line
                        if first {
                            first = false;
                        } else {
                            let line = pending.as_ref().map_or("", |piece| piece.text());
                            write!(
                                f,
                                [FormatContentLine {
                                    line,
                                    base_indent,
                                    state: &state
                                }]
                            )?;
                        }
                        pending = None;
                    } else if piece.is_whitespace() {
                        pending = Some(piece);
                    } else {
                        // A comment and anything after it isn't part of the
                        // scalar
                        break;
                    }
                }
            }

            // A keep-chomped scalar that closes the last document ends with
            // a line break of its own, so that the line break the enclosing
            // structure prints turns into a kept trailing blank line:
            //
            // ```yaml
            // a: |+
            //   foo
            //
            // ```
            if chomping == Chomping::Keep && is_last && state.get().any_line {
                write!(f, [text("\n", None)])?;
            }

            Ok(())
        });

        match indicator {
            // Content one level deeper than the parent node
            None => write!(f, [format_replaced(&value_token, &indent(&content))]),
            // An explicit indicator makes the content indentation absolute
            Some(indicator) => {
                let align_spaces =
                    " ".repeat(indicator.saturating_sub(1).saturating_add(ancestors));
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &dedent_to_root(&align(align_spaces, &content))
                    )]
                )
            }
        }
    }
}

/// The block collections a node can be nested in, each of which adds one
/// level of indentation
const BLOCK_COLLECTIONS: TokenSet<YamlSyntaxKind> = token_set![
    YamlSyntaxKind::YAML_BLOCK_MAPPING,
    YamlSyntaxKind::YAML_BLOCK_SEQUENCE
];

/// What a block scalar does with the line breaks trailing its content,
/// chosen by the chomping indicator in its header.
///
/// See <https://yaml.org/spec/1.2.2/#8112-block-chomping-indicator>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chomping {
    /// Keep the line break ending the last non-empty line, drop the blank
    /// lines after it. The default, used when no indicator is given
    Clip,
    /// Drop the ending line break and the trailing blank lines (`-`)
    Strip,
    /// Keep both the ending line break and the trailing blank lines (`+`)
    Keep,
}

/// What the content lines formatted so far looked like, which the following
/// lines of the same scalar base their formatting on
#[derive(Debug, Clone, Copy, Default)]
struct LineState {
    /// Whether any content line has been formatted yet
    any_line: bool,
    /// Whether the most recently formatted line was blank
    prev_empty: bool,
}

/// Formats one content line, stripped of its base indentation, preceded by
/// the line break that opens it. A blank line prints its line break as
/// literal text so that no indentation ends up on it and consecutive line
/// breaks don't collapse into one.
///
/// The lines of a scalar share one `state` cell, which carries what the
/// previous lines were between the `fmt` calls
struct FormatContentLine<'a> {
    line: &'a str,
    base_indent: usize,
    state: &'a std::cell::Cell<LineState>,
}

impl Format<YamlFormatContext> for FormatContentLine<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut state = self.state.get();
        let line = &self.line[leading_spaces(self.line).min(self.base_indent)..];
        if line.is_empty() {
            write!(f, [text("\n", None)])?;
            state.prev_empty = true;
        } else {
            if state.any_line && state.prev_empty {
                // Ends the blank line; the following line break element then
                // only provides the indentation
                write!(f, [text("\n", None)])?;
            }
            write!(f, [hard_line_break(), text(line, None)])?;
            state.prev_empty = false;
        }
        state.any_line = true;
        self.state.set(state);
        Ok(())
    }
}

declare_node_union! {
    /// A block scalar, the node a `YamlBlockContent` sits in
    AnyYamlBlockScalar = YamlLiteralScalar | YamlFoldedScalar
}

impl AnyYamlBlockScalar {
    fn headers(&self) -> YamlBlockHeaderList {
        match self {
            Self::YamlLiteralScalar(scalar) => scalar.headers(),
            Self::YamlFoldedScalar(scalar) => scalar.headers(),
        }
    }
}

/// Reads the chomping behavior and the explicit indentation indicator from
/// the headers of the enclosing block scalar
fn parent_headers(node: &YamlBlockContent) -> (Chomping, Option<usize>) {
    let mut chomping = Chomping::Clip;
    let mut indicator = None;

    let headers = node
        .syntax()
        .parent()
        .and_then(AnyYamlBlockScalar::cast)
        .map(|scalar| scalar.headers());
    for header in headers.into_iter().flatten() {
        match header {
            AnyYamlBlockHeader::YamlBlockStripIndicator(_) => chomping = Chomping::Strip,
            AnyYamlBlockHeader::YamlBlockKeepIndicator(_) => chomping = Chomping::Keep,
            AnyYamlBlockHeader::YamlIndentationIndicator(header) => {
                indicator = header
                    .indentation_indicator_token()
                    .ok()
                    .and_then(|token| token.text_trimmed().parse::<usize>().ok())
                    .filter(|indicator| *indicator > 0);
            }
            AnyYamlBlockHeader::YamlBogusBlockHeader(_) => {}
        }
    }

    (chomping, indicator)
}

/// The number of leading space characters of the line.
///
/// YAML indentation consists exclusively of spaces; tabs are forbidden (rule
/// `s-indent`, section 6.1 of the spec), so a leading tab is scalar content
fn leading_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}

/// Iterator over the lines of a block scalar token, splitting at `\r\n`,
/// `\n`, and lone `\r` line breaks alike.
///
/// Text that ends with a line break yields a final empty line, which is how
/// it is distinguished from text that ends mid-line
#[derive(Debug, Clone)]
struct ContentLines<'a> {
    rest: Option<&'a str>,
}

impl<'a> ContentLines<'a> {
    fn new(text: &'a str) -> Self {
        Self { rest: Some(text) }
    }

    /// Whether the remaining text ends with a line break, i.e. whether the
    /// last line the iterator yields is an empty one split off by that break
    fn ends_with_break(&self) -> bool {
        self.rest.is_some_and(|text| text.ends_with(['\n', '\r']))
    }
}

impl<'a> Iterator for ContentLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.rest.take()?;
        // A `\r\n` pair is always entered at the `\r`, never in the middle,
        // since the search matches whichever of the two bytes comes first
        match text.find(['\n', '\r']) {
            Some(index) => {
                let bytes = text.as_bytes();
                let break_len = match bytes[index] {
                    b'\r' if bytes.get(index + 1) == Some(&b'\n') => 2,
                    _ => 1,
                };
                self.rest = Some(&text[index + break_len..]);
                Some(&text[..index])
            }
            None => Some(text),
        }
    }
}

/// The aggregates of the content lines that the formatting is derived from,
/// all gathered in a single pass over the lines
struct ContentStats {
    /// The number of lines
    count: usize,
    /// The number of lines that remain after dropping the trailing empty
    /// lines
    trimmed_count: usize,
    /// The leading spaces of the first non-blank line, `usize::MAX` when
    /// every line is blank
    first_indent: usize,
    /// The smallest leading spaces of any non-blank line, `usize::MAX` when
    /// every line is blank
    min_indent: usize,
}

impl ContentStats {
    fn new<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut stats = Self {
            count: 0,
            trimmed_count: 0,
            first_indent: usize::MAX,
            min_indent: usize::MAX,
        };
        for line in lines {
            stats.count += 1;
            if !line.is_empty() {
                stats.trimmed_count = stats.count;
            }
            let spaces = leading_spaces(line);
            if spaces < line.len() {
                if stats.first_indent == usize::MAX {
                    stats.first_indent = spaces;
                }
                stats.min_indent = stats.min_indent.min(spaces);
            }
        }
        stats
    }
}

/// Whether nothing follows the node in the stream, i.e. the node closes the
/// last document.
///
/// That is the case exactly when no ancestor, walking up to the root, has a
/// sibling after it — any such sibling would put content after the node
fn closes_last_document(node: &YamlBlockContent) -> bool {
    let mut current = node.syntax().clone();
    loop {
        if current.next_sibling().is_some() {
            return false;
        }
        match current.parent() {
            Some(parent) if parent.kind() != YamlSyntaxKind::YAML_ROOT => current = parent,
            _ => return true,
        }
    }
}
