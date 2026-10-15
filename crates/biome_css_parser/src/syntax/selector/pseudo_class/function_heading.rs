use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::selector_lex_context;
use crate::syntax::value::function::is_nth_at_adjacent_l_paren;
use crate::syntax::{is_at_identifier, parse_regular_identifier, parse_regular_number};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T, TextRange, decode_css_identifier};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::token_set;

const HEADING_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{'], T!['}'], T![;]];

#[inline]
pub(crate) fn is_at_pseudo_class_function_heading(p: &mut CssParser) -> bool {
    is_at_identifier(p)
        && decode_css_identifier(p.cur_text()).eq_ignore_ascii_case("heading")
        && is_nth_at_adjacent_l_paren(p, 1)
}

/// Parses a [heading selector](https://drafts.csswg.org/selectors-5/#headings)
/// with a nonempty list of integer tokens, without limiting their values.
///
/// ```css
/// .article :heading(1, 2, 3) { text-wrap: balance; }
/// ```
pub(crate) fn parse_pseudo_class_function_heading(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_heading(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T!['(']);

    let mut levels = HeadingLevelList::default();
    levels.parse_list(p);
    let closed = p.at(T![')']);
    if !closed {
        ParseRecoveryTokenSet::new(CSS_BOGUS, HEADING_RECOVERY_SET)
            .enable_recovery_on_line_break()
            .recover(p)
            .ok();
    }
    let context = selector_lex_context(p);
    p.expect_with_context(T![')'], context);
    let kind = if closed && !levels.invalid {
        CSS_PSEUDO_CLASS_FUNCTION_HEADING
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

#[inline]
fn is_at_heading_level(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && !p.cur_text().contains(['.', 'e', 'E'])
}

fn expected_heading_level(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected an integer heading level.", range)
}

#[derive(Default)]
struct HeadingLevelList {
    invalid: bool,
}

impl ParseSeparatedList for HeadingLevelList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_HEADING_LEVEL_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        if !is_at_heading_level(p) {
            return Absent;
        }
        parse_regular_number(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at_ts(HEADING_RECOVERY_SET)
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        match parsed_element.or_add_diagnostic(p, expected_heading_level) {
            Some(m) => Ok(m),
            None => {
                self.invalid = true;
                Err(RecoveryError::RecoveryDisabled)
            }
        }
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut CssParser) -> bool {
        let present = p.expect(T![,]);
        self.invalid |= !present;
        present
    }

    fn diagnose_missing_element(&mut self, p: &mut CssParser) {
        self.invalid = true;
        p.error(expected_heading_level(p, p.cur_range()));
    }
}
