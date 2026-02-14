use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;

/// Parse entity or numeric character reference per CommonMark §6.2.
///
/// Grammar: MdEntityReference = value: 'md_entity_literal'
///
/// Valid patterns:
/// - Named entity: `&name;` where name is 2-31 alphanumeric chars starting with letter
/// - Decimal numeric: `&#digits;` where digits is 1-7 decimal digits
/// - Hexadecimal: `&#xhex;` or `&#Xhex;` where hex is 1-6 hex digits
///
/// The lexer has already validated and tokenized valid entity references as
/// MD_ENTITY_LITERAL tokens. Invalid patterns remain as textual.
pub(crate) fn parse_entity_reference(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(MD_ENTITY_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(MD_ENTITY_LITERAL);
    Present(m.complete(p, MD_ENTITY_REFERENCE))
}
