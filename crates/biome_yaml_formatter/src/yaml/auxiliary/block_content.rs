use crate::content_lines::ContentLines;
use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_parser::{TokenSet, token_set};
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlBlockHeader, AnyYamlBlockScalar, YamlBlockContent, YamlBlockContentFields,
    YamlSyntaxKind,
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

        // The number of block collections the node is nested in, which the
        // absolute indentation of explicitly indented content is computed from
        let ancestors = node
            .syntax()
            .ancestors()
            .skip(1)
            .filter(|ancestor| BLOCK_COLLECTIONS.contains(ancestor.kind()))
            .count();
        // An explicit indicator makes the content indentation absolute
        let explicit_indent =
            indicator.map(|indicator| indicator.saturating_sub(1).saturating_add(ancestors));

        let stats = ContentStats::new(lines.clone(), explicit_indent);
        let base_indent = explicit_indent.unwrap_or(stats.first_indent);
        let scalar_end = stats.scalar_end;

        let kept_lines = match (chomping, scalar_end) {
            // With a trailing comment region the token continues past the
            // content, whose trailing blank lines all belong to it
            (Chomping::Keep, Some(scalar_end)) => scalar_end,
            // The line break terminating the last line is printed by the
            // enclosing structure, so the line it opens isn't content
            (Chomping::Keep, None) => stats
                .line_count
                .saturating_sub(usize::from(ends_with_break)),
            // Trailing blank lines are dropped
            (Chomping::Clip | Chomping::Strip, _) => stats.trimmed_count,
        };

        let is_last = closes_last_document(node);
        let state = std::cell::Cell::new(LineState::default());
        let content = format_with(|f| {
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
                && scalar_end.is_none()
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
            //
            // A comment following the scalar prints the blank line above
            // itself instead, so the scalar adds none of its own
            let comments_follow = value_token.next_token().is_some_and(|next| {
                next.leading_trivia()
                    .pieces()
                    .any(|piece| piece.is_comments())
            });
            if chomping == Chomping::Keep
                && scalar_end.is_none()
                && is_last
                && !comments_follow
                && state.get().any_line
            {
                write!(f, [text("\n", None)])?;
            }

            // An empty keep-chomped scalar that closes the last document
            // still owns the blank lines after its header; the parser left
            // them in the leading trivia of the next token. Every break is
            // printed here: after a literal line break the printer drops
            // the line break the enclosing structure emits, so it can't
            // supply the last one:
            //
            // ```yaml
            // keep: |+
            //
            // ```
            if chomping == Chomping::Keep
                && scalar_end.is_none()
                && token_text.is_empty()
                && is_last
                && let Some(next_token) = value_token.next_token()
            {
                let breaks = next_token
                    .leading_trivia()
                    .pieces()
                    .take_while(|piece| piece.is_newline() || piece.is_whitespace())
                    .filter(|piece| piece.is_newline())
                    .count();
                for _ in 1..=breaks {
                    write!(f, [text("\n", None)])?;
                }
            }

            Ok(())
        });

        // The blank lines the chomping dropped and the trailing comment
        // lines after them, printed after the scalar at the document root,
        // where the comments live:
        //
        // ```yaml
        // strip: |-
        //   # text
        //
        // # comment
        // ```
        let trailing = format_with(|f| {
            if scalar_end.is_none() {
                return Ok(());
            }
            for line in lines.clone().skip(kept_lines) {
                write!(
                    f,
                    [FormatContentLine {
                        line,
                        // Strips all the indentation, putting the comments
                        // at column zero
                        base_indent: usize::MAX,
                        state: &state
                    }]
                )?;
            }
            Ok(())
        });

        match explicit_indent {
            // Content one level deeper than the parent node
            None => write!(
                f,
                [format_replaced(
                    &value_token,
                    &format_args![indent(&content), dedent_to_root(&trailing)]
                )]
            ),
            // An explicit indicator makes the content indentation absolute
            Some(explicit_indent) => {
                let align_spaces = " ".repeat(explicit_indent);
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &format_args![
                            dedent_to_root(&align(align_spaces, &content)),
                            dedent_to_root(&trailing)
                        ]
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

/// The aggregates of the content lines that the formatting is derived from,
/// all gathered in a single pass over the lines
struct ContentStats {
    /// The number of lines
    line_count: usize,
    /// The number of content lines that remain after dropping the trailing
    /// empty lines: those holding only whitespace up to the base indentation
    trimmed_count: usize,
    /// The leading spaces of the first non-blank line, `usize::MAX` when
    /// every line is blank
    first_indent: usize,
    /// The index of the first non-blank line indented less than the base,
    /// which ends the scalar per the spec. The lexer includes such lines
    /// (trailing comment lines, in practice) in the content token, so
    /// everything from this line on is not content
    scalar_end: Option<usize>,
}

impl ContentStats {
    fn new<'a>(lines: impl Iterator<Item = &'a str>, explicit_indent: Option<usize>) -> Self {
        let mut stats = Self {
            line_count: 0,
            trimmed_count: 0,
            first_indent: usize::MAX,
            scalar_end: None,
        };
        // The base indentation the lines are measured against: the explicit
        // one, or the first non-blank line's once that line is reached. The
        // blank lines before it can't end the scalar, and whether they are
        // empty can't matter: the non-blank line after them always sits past
        // them in `trimmed_count`
        let mut base_indent = explicit_indent;
        for line in lines {
            stats.line_count += 1;
            // The trailing comment region reaches to the end of the token;
            // only the total line count is still of interest
            if stats.scalar_end.is_some() {
                continue;
            }
            let spaces = leading_spaces(line);
            if spaces < line.len() {
                if stats.first_indent == usize::MAX {
                    stats.first_indent = spaces;
                }
                let base_indent = *base_indent.get_or_insert(spaces);
                if spaces < base_indent {
                    stats.scalar_end = Some(stats.line_count.saturating_sub(1));
                } else {
                    stats.trimmed_count = stats.line_count;
                }
            } else if base_indent.is_some_and(|base_indent| spaces > base_indent) {
                // A line holding only whitespace up to the base indentation
                // is an empty line; more whitespace than that is content
                stats.trimmed_count = stats.line_count;
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
