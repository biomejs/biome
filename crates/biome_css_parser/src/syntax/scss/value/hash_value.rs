use crate::lexer::{CssLexContext, CssReLexContext};
use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use crate::syntax::property::color::{is_at_color, parse_color};
use crate::syntax::scss::{is_at_scss_interpolation, parse_scss_regular_interpolation};
use biome_css_syntax::CssSyntaxKind::{
    CSS_GENERIC_DELIMITER, SCSS_HASH_VALUE, SCSS_INTERPOLATED_VALUE,
    SCSS_INTERPOLATED_VALUE_PART_LIST,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, ParserProgress};

/// Parses a Sass hash string or delegates to hexadecimal-color parsing.
///
/// The caller must enable full SCSS value semantics and handle leading
/// interpolation first. A complete identifier selects the string branch;
/// numeric starts and color spellings retain the raw color parser's behavior.
/// Adjacent interpolation belongs to the same case-sensitive string, while
/// comments and whitespace terminate it.
/// A trailing percent sign remains a distinct delimiter fragment.
///
/// ```scss
/// $selector: #ABCG;
/// $interpolated: #ABC#{G};
/// $numeric_tail: #ABC#{G}1PX;
/// $percent_tail: #ABC#{G}1%;
/// $escaped: #\61 pp;
/// $color: #ABC;
/// ```
///
/// See [Sass unquoted strings](https://sass-lang.com/documentation/values/strings/#other-unquoted-strings)
/// and [interpolation](https://sass-lang.com/documentation/interpolation/).
#[inline]
pub(crate) fn parse_scss_hash_value_or_color(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_color(p) {
        return Absent;
    }

    let Some(scan) = p.source().scss_hash_value_scan() else {
        return parse_color(p);
    };

    let m = p.start();
    p.bump_with_context(T![#], CssLexContext::ScssHashValue(scan));
    p.bump(T![ident]);
    let head = m.complete(p, SCSS_HASH_VALUE);

    let has_gap = |p: &mut CssParser| p.last_end() != Some(p.cur_range().start());
    if has_gap(p) || !is_at_scss_interpolation(p) {
        return Present(head);
    }

    let list = head.precede(p);
    let mut progress = ParserProgress::default();
    while !has_gap(p) {
        progress.assert_progressing(p);
        if is_at_scss_interpolation(p) {
            parse_scss_regular_interpolation(p).ok();
        } else if p.at(T![%])
            && matches!(
                p.nth(1),
                T![EOF]
                    | T![;]
                    | T!['}']
                    | T![')']
                    | T![']']
                    | T![,]
                    | T![:]
                    | T!['{']
                    | T![!]
                    | T![...]
            )
        {
            // Sass permits a trailing `%` after interpolation as a separate
            // value, rather than requiring a modulo operand: `#ABC#{G}1%`.
            let delimiter = p.start();
            p.bump(T![%]);
            delimiter.complete(p, CSS_GENERIC_DELIMITER);
            break;
        } else {
            // Hash-string tails are identifier text even when regular lexing
            // produces a number or dimension, as in `#ABC#{G}1PX`.
            p.re_lex(CssReLexContext::ScssIdentifierContinuation);
            if parse_regular_identifier(p).is_absent() {
                break;
            }
        }
    }

    Present(
        list.complete(p, SCSS_INTERPOLATED_VALUE_PART_LIST)
            .precede(p)
            .complete(p, SCSS_INTERPOLATED_VALUE),
    )
}
