//! Helpers for the indented Sass syntax.
//!
//! The indented syntax uses indentation instead of braces to delimit blocks
//! and line breaks instead of semicolons to terminate statements. These
//! helpers compute the indentation of the current line and decide whether an
//! indented list has reached the end of its block.

use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind;
use biome_parser::Parser;
use biome_parser::prelude::TokenSource;

/// Returns the indentation of the line that contains the current token.
///
/// Indentation is the number of leading whitespace characters of the line, not
/// the column of the token. A token can follow a comment on the same line
/// without changing the line indentation.
pub(crate) fn current_line_indentation(p: &CssParser) -> u32 {
    let text = p.source().text();
    let offset = usize::from(p.cur_range().start());
    let line_start = text[..offset].rfind('\n').map_or(0, |index| index + 1);

    text[line_start..offset]
        .chars()
        .take_while(|character| matches!(character, ' ' | '\t'))
        .count() as u32
}

/// Returns `true` when the current token ends the enclosing indented block.
///
/// An indented list ends at the end of the file or when a token starts a line
/// that is less indented than the innermost block. Tokens that continue the
/// current line never end the block.
pub(crate) fn is_at_indented_list_end(p: &mut CssParser) -> bool {
    if p.at(CssSyntaxKind::EOF) {
        return true;
    }

    let Some(&block_indentation) = p.state().sass_indentation.last() else {
        return false;
    };

    p.has_preceding_line_break() && current_line_indentation(p) < block_indentation
}

/// Returns `true` when an indented list has reached its end for the given
/// braced terminator.
///
/// Files without the indented syntax keep using the braced terminator.
///
/// Always inlined so the hot list terminators keep the plain `p.at(end_kind)`
/// check for non-indented files without a call boundary.
#[inline(always)]
pub(crate) fn is_at_list_end_for(p: &mut CssParser, end_kind: CssSyntaxKind) -> bool {
    if p.source_type.is_indented_syntax() {
        is_at_indented_list_end(p)
    } else {
        p.at(end_kind)
    }
}

/// Returns `true` when a line break terminates the current indented statement.
///
/// This is used as a value boundary so a declaration value stops at the end of
/// its line, matching the Sass rule that a line break ends a statement where a
/// statement can end.
pub(crate) fn is_at_indented_statement_end(p: &mut CssParser) -> bool {
    p.has_preceding_line_break()
}

/// Returns `true` when a line break ends the current selector in the indented
/// syntax.
///
/// The selector lexer turns whitespace that spans a line break into trivia, so
/// the selector list ends when the next token starts a new line.
pub(crate) fn is_at_indented_selector_end(p: &mut CssParser) -> bool {
    p.source_type.is_indented_syntax() && p.has_preceding_line_break()
}

/// Returns `true` when the current token starts an indented block body.
///
/// A block body starts on a new line and is indented deeper than the enclosing
/// block. It is used to decide whether a blockless at-rule such as
/// `@include foo` is followed by a block.
pub(crate) fn is_at_indented_block_body(p: &mut CssParser) -> bool {
    if !p.source_type.is_indented_syntax() || !p.has_preceding_line_break() {
        return false;
    }

    let parent_indentation = p.state().sass_indentation.last().copied().unwrap_or(0);
    current_line_indentation(p) > parent_indentation
}
