use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::{is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Checks if the parser is currently at an unknown CSS at-rule.
///
/// An unknown CSS at-rule is identified by being an identifier followed by optional parameters,
/// such as `@my-custom-rule param;` or `@unknown-rule { /* CSS content */ }`.
#[inline]
pub(crate) fn is_at_unknown_at_rule(p: &mut CssParser) -> bool {
    is_at_identifier(p)
}

/// Represents an unknown or unsupported CSS at-rule during parsing.
///
/// When encountered during parsing, `CssUnknownAtRule` serves as a fallback mechanism,
/// allowing the parser to continue processing the stylesheet by treating the unsupported rule
/// as part of an unformed tree. This enables graceful handling of CSS constructs that are not
/// supported by the parser, ensuring that the stylesheet can still be processed without errors.
#[inline]
pub(crate) fn parse_unknown_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unknown_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok(); // we've checked that the next token is an identifier

    {
        let m = p.start();

        // Skip all tokens until the end of the property value or the next property.
        // EOF indicates the end of the file.
        // '{' indicates the start of a block.
        // ';' indicates the end of the property value.
        while !(p.at(EOF) || p.at(T!['{']) || p.at(T![;])) {
            p.bump_any();
        }

        m.complete(p, CSS_UNKNOWN_AT_RULE_COMPONENT_LIST);
    }

    let kind = if p.at(T!['{']) {
        parse_declaration_or_rule_list_block(p);
        CSS_UNKNOWN_BLOCK_AT_RULE
    } else {
        p.expect(T![;]);
        CSS_UNKNOWN_VALUE_AT_RULE
    };

    Present(m.complete(p, kind))
}
