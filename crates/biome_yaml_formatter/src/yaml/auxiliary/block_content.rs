use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlBlockHeader, YamlBlockContent, YamlBlockContentFields, YamlBlockHeaderList,
    YamlSyntaxKind, YamlSyntaxNode,
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

        let (chomping, indicator) = parent_headers(node.syntax());

        let token_text = value_token.text_trimmed();

        // The first line of the token is the tail of the header line; the
        // content starts after its line break
        let body = line_break(token_text).map(|(index, len)| &token_text[index + len..]);
        let lines = || {
            let mut rest = body;
            std::iter::from_fn(move || {
                let text = rest?;
                match line_break(text) {
                    Some((index, len)) => {
                        rest = Some(&text[index + len..]);
                        Some(&text[..index])
                    }
                    None => {
                        rest = None;
                        Some(text)
                    }
                }
            })
        };
        let ends_with_break = token_text.ends_with(['\n', '\r']);

        let ancestors = collection_ancestor_count(node.syntax());
        let base_indent = match indicator {
            Some(indicator) => (indicator - 1).saturating_add(ancestors),
            None => lines()
                .find_map(|line| {
                    let spaces = leading_spaces(line);
                    (spaces < line.len()).then_some(spaces)
                })
                .unwrap_or(usize::MAX),
        };

        // A non-empty line that is indented less than the base ends the
        // scalar per the spec, but the lexer includes such lines (trailing
        // comment lines, in practice) in the content token. Everything from
        // the first such line on is not content; it is rendered after the
        // scalar, dedented to the document root
        let scalar_end = lines().position(|line| {
            let spaces = leading_spaces(line);
            spaces < line.len() && spaces < base_indent
        });
        let content_lines = scalar_end.unwrap_or(usize::MAX);

        // A line holding only whitespace up to the base indentation is an
        // empty line, not content
        let is_empty_line = |line: &str| {
            let spaces = leading_spaces(line);
            spaces == line.len() && spaces <= base_indent
        };

        let kept_lines = match chomping {
            // With a trailing comment region the token continues past the
            // content, whose trailing blank lines all belong to it
            Chomping::Keep if scalar_end.is_some() => content_lines,
            // The line break terminating the last line is printed by the
            // enclosing structure, so the line it opens isn't content
            Chomping::Keep => lines().count().saturating_sub(usize::from(ends_with_break)),
            // Trailing blank lines are dropped
            Chomping::Clip | Chomping::Strip => lines()
                .take(content_lines)
                .enumerate()
                .filter(|(_, line)| !is_empty_line(line))
                .last()
                .map_or(0, |(index, _)| index + 1),
        };

        let is_last = is_last_descendant(node.syntax());
        let state = std::cell::Cell::new(LineState::default());
        let content = format_with(|f| {
            for line in lines().take(kept_lines) {
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
            for line in lines().skip(kept_lines) {
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

        match indicator {
            // Content one level deeper than the parent node
            None => write!(
                f,
                [format_replaced(
                    &value_token,
                    &format_args![indent(&content), dedent_to_root(&trailing)]
                )]
            ),
            // An explicit indicator makes the content indentation absolute
            Some(indicator) => {
                let align_spaces = " ".repeat((indicator - 1).saturating_add(ancestors));
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chomping {
    Clip,
    Strip,
    Keep,
}

#[derive(Debug, Clone, Copy, Default)]
struct LineState {
    any_line: bool,
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
fn parent_headers(node: &YamlSyntaxNode) -> (Chomping, Option<usize>) {
    let mut chomping = Chomping::Clip;
    let mut indicator = None;

    let headers = node
        .parent()
        .into_iter()
        .flat_map(|parent| parent.children())
        .find_map(YamlBlockHeaderList::cast);
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

fn leading_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}

/// The position and byte length of the first line break in `text`, treating
/// `\r\n`, `\n`, and a lone `\r` each as one break
fn line_break(text: &str) -> Option<(usize, usize)> {
    // A `\r\n` pair is always entered at the `\r`, never in the middle,
    // since the search matches whichever of the two bytes comes first
    let index = text.find(['\n', '\r'])?;
    let bytes = text.as_bytes();
    let len = match bytes[index] {
        b'\r' if bytes.get(index + 1) == Some(&b'\n') => 2,
        _ => 1,
    };
    Some((index, len))
}

/// The number of block collections the node is nested in, which the
/// absolute indentation of explicitly indented content is computed from
fn collection_ancestor_count(node: &YamlSyntaxNode) -> usize {
    node.ancestors()
        .filter(|ancestor| {
            matches!(
                ancestor.kind(),
                YamlSyntaxKind::YAML_BLOCK_MAPPING | YamlSyntaxKind::YAML_BLOCK_SEQUENCE
            )
        })
        .count()
}

/// Whether no node follows this one in the stream, i.e. the node closes the
/// last document
fn is_last_descendant(node: &YamlSyntaxNode) -> bool {
    let mut current = node.clone();
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
