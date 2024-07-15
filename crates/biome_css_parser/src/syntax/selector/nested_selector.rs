use crate::parser::CssParser;
use crate::syntax::selector::selector_lex_context;
use biome_css_syntax::CssSyntaxKind::{CSS_NESTED_SELECTOR, CSS_NESTED_SELECTOR_LIST};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::Parser;

pub(crate) struct NestedSelectorList;
impl ParseNodeList for NestedSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_NESTED_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_nested_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !p.at(T![&])
    }

    fn recover(&mut self, _p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element {
            Absent => Err(RecoveryError::AlreadyRecovered),
            Present(m) => Ok(m),
        }
    }
}

/// Checks if the parser is currently positioned at the start of a nested selector.
///
/// In CSS, nested selectors often start with an `&` character, which refers to the parent selector.
/// This function checks if the current token in the parser matches `&`.
#[inline]
fn is_at_nested_selector(p: &mut CssParser) -> bool {
    p.at(T![&])
}

/// Parses a nested selector from the current position in the CSS parser.
#[inline]
fn parse_nested_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_nested_selector(p) {
        return Absent;
    }

    let m = p.start();
    let context = selector_lex_context(p);
    p.bump_with_context(T![&], context);
    Present(m.complete(p, CSS_NESTED_SELECTOR))
}
