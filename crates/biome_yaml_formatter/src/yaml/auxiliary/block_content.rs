use crate::prelude::*;
use biome_formatter::write;
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

        let kept_lines = match chomping {
            // The line break terminating the last line is printed by the
            // enclosing structure, so the line it opens isn't content
            Chomping::Keep => lines().count().saturating_sub(usize::from(ends_with_break)),
            // Trailing blank lines are dropped
            Chomping::Clip | Chomping::Strip => lines()
                .enumerate()
                .filter(|(_, line)| !line.is_empty())
                .last()
                .map_or(0, |(index, _)| index + 1),
        };

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

        // FIXME: A non-empty line that is indented less than the base ends
        // the scalar per the spec, but the lexer includes such lines
        // (trailing comment lines, in practice) in the content token.
        // Re-indenting them would promote them to actual scalar content, so
        // the content is kept exactly as is for now. Once the lexer ends the
        // scalar at such lines, this fallback can be removed
        if lines().any(|line| {
            let spaces = leading_spaces(line);
            spaces < line.len() && spaces < base_indent
        }) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        let is_last = is_last_descendant(node.syntax());
        let content = format_with(|f| {
            let state = std::cell::Cell::new(LineState::default());
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
                let align_spaces = " ".repeat((indicator - 1).saturating_add(ancestors));
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
