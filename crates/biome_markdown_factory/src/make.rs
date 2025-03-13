use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxToken};

pub use crate::generated::node_factory::*;

/// Create a textual token
pub fn textual(text: &str) -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL, text, [], [])
}

/// Create a string token
pub fn string(text: &str) -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MD_STRING_LITERAL, text, [], [])
}

/// Create a hash token for headers
pub fn hash() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::HASH, "#", [], [])
}

/// Create a backtick token
pub fn backtick() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::BACKTICK, "`", [], [])
}

/// Create a star token for emphasis
pub fn star() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::STAR, "*", [], [])
}

/// Create an underscore token for emphasis
pub fn underscore() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::UNDERSCORE, "_", [], [])
}

/// Create a left bracket token
pub fn l_brack() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::L_BRACK, "[", [], [])
}

/// Create a right bracket token
pub fn r_brack() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::R_BRACK, "]", [], [])
}

/// Create a left parenthesis token
pub fn l_paren() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::L_PAREN, "(", [], [])
}

/// Create a right parenthesis token
pub fn r_paren() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::R_PAREN, ")", [], [])
}

/// Create a bang token for images
pub fn bang() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::BANG, "!", [], [])
}

/// Create a minus token for thematic breaks
pub fn minus() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MINUS, "-", [], [])
}

/// Create a thematic break token
pub fn thematic_break() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MD_THEMATIC_BREAK_LITERAL, "---", [], [])
}

/// Create a newline token
pub fn newline() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::NEWLINE, "\n", [], [])
}

/// Create a whitespace token
pub fn whitespace(text: &str) -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::WHITESPACE, text, [], [])
}

/// Create a tab token
pub fn tab() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::TAB, "\t", [], [])
}

/// Create an indent chunk token for indented code blocks
pub fn indent_chunk() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MD_INDENT_CHUNK_LITERAL, "    ", [], [])
}

/// Create a hard line break token
pub fn hard_line_break() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::MD_HARD_LINE_LITERAL, "  \n", [], [])
}

/// Create a greater than token for blockquotes
pub fn greater_than() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::R_ANGLE, ">", [], [])
}

/// Create a plus token for unordered lists
pub fn plus() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::PLUS, "+", [], [])
}

/// Create a digit token for ordered lists
pub fn digit(text: &str) -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::DIGIT, text, [], [])
}

/// Create a period token for ordered lists
pub fn period() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::PERIOD, ".", [], [])
}

/// Create a pipe token for tables
pub fn pipe() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::PIPE, "|", [], [])
}

/// Create a colon token for table alignment
pub fn colon() -> MarkdownSyntaxToken {
    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::COLON, ":", [], [])
}
