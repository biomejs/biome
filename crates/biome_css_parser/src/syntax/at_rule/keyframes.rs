use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::{
    expected_keyframes_item, expected_keyframes_item_selector,
};
use crate::syntax::blocks::{parse_block_body, parse_declaration_list_block};
use crate::syntax::parse_error::{expected_block, expected_non_css_wide_keyword_identifier};
use crate::syntax::value::dimension::{is_at_percentage_dimension, parse_percentage_dimension};
use crate::syntax::{
    is_at_declaration, is_at_identifier, parse_custom_identifier, parse_string, BODY_RECOVERY_SET,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_keyframes_at_rule(p: &mut CssParser) -> bool {
    p.at(T![keyframes])
}

#[inline]
pub(crate) fn parse_keyframes_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![keyframes]);

    let name = if is_at_identifier(p) {
        parse_custom_identifier(p, CssLexContext::Regular)
    } else {
        parse_string(p)
    };

    let kind = if name
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, KEYFRAMES_NAME_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_non_css_wide_keyword_identifier,
        )
        .is_ok()
    {
        CSS_KEYFRAMES_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    if parse_keyframes_block(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_block,
        )
        .is_err()
    {
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    Present(m.complete(p, kind))
}
const KEYFRAMES_NAME_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

#[inline]
fn parse_keyframes_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = parse_block_body(p, |p| {
        KeyframesItemList.parse_list(p);
    });

    Present(m.complete(p, CSS_KEYFRAMES_BLOCK))
}

struct KeyframesItemListParseRecovery;

impl ParseRecovery for KeyframesItemListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_KEYFRAMES_ITEM;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || is_at_keyframes_item_selector(p)
    }
}

struct KeyframesItemList;

impl ParseNodeList for KeyframesItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_KEYFRAMES_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_keyframes_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &KeyframesItemListParseRecovery, expected_keyframes_item)
    }
}

struct KeyframesItemBlockParseRecovery;

impl ParseRecovery for KeyframesItemBlockParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_BLOCK;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // We need to recover the contents of an invalid block for the following cases:
        // We have a declaration block, but it lacks a '{' token:
        //    @keyframes name {
        //      from
        //          color: red;
        //      } <----- here it's a recover point for the keyframes item block.
        //   }
        //
        //    @keyframes name {
        //      from
        //          color: red;
        //      to <----- here it's a recover point for the next keyframes item. {
        //          color: blue;
        //      }
        //   }
        p.at(T!['}']) || is_at_keyframes_item_selector(p)
    }
}

#[inline]
fn parse_keyframes_item(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    KeyframesSelectorList.parse_list(p);
    // `parse_list` will take care of recovering invalid selectors, but if
    // _none_ are present, we still want to add a diagnostic to explain the
    // error while continuing the rest of the parse, since we know that the
    // following content should still be a declaration list block.
    if p.cur_range().start() == m.start() {
        p.error(expected_keyframes_item_selector(p, p.cur_range()))
    }

    let declaration_block = parse_declaration_list_block(p).or_recover(
        p,
        &KeyframesItemBlockParseRecovery,
        expected_block,
    );

    let kind = match declaration_block {
        Ok(block) => {
            if block.kind(p) == CSS_BOGUS_BLOCK {
                // If we have recovered a bogus block, we need to ensure that we consume the closing '}' token.
                // Otherwise, the parser will interpret the '}' token as the end of the keyframe block:
                //    @keyframes name { <----- the keyframe block
                //      from
                //          color: red;
                //      } <----- here it's a recover point
                //   }
                p.eat(T!['}']);
                CSS_BOGUS_KEYFRAMES_ITEM
            } else {
                CSS_KEYFRAMES_ITEM
            }
        }
        Err(_) => CSS_BOGUS_KEYFRAMES_ITEM,
    };

    Present(m.complete(p, kind))
}

struct KeyframesSelectorListParseRecovery;

impl ParseRecovery for KeyframesSelectorListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SELECTOR;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // If we have an invalid selector, we need to skip tokens until we find a recover point:
        // @keyframes name1 {
        //  from someivalidselector { <---- a recover point
        //    color: red;
        //  }
        // 	from someivalidselector to <---- a recover point {
        // 		color: red;
        // 	}
        // 	to
        // 	    a recover point ----> color: blue;
        // 	}
        // 	to
        // 	   color: blue;
        // 	} <----- a recover point
        // }
        is_at_keyframes_item_selector(p) || is_at_keyframes_selector_list_end(p)
    }
}

struct KeyframesSelectorList;

impl ParseSeparatedList for KeyframesSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_KEYFRAMES_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_keyframes_item_selector(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_keyframes_selector_list_end(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &KeyframesSelectorListParseRecovery,
            expected_keyframes_item_selector,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn is_at_keyframes_selector_list_end(p: &mut CssParser) -> bool {
    // We check if the next element is a separator or a keyframes selector.
    // It allows us to have a better recovery for the following case:
    // @keyframes name1 {
    // 	from   <----- here we miss a '{', but we can try to assume that we can parse a declaration block.
    // 		color: red;
    // 	}
    // 	from   <----- here we miss a '{', but we can try to assume that we can parse a declaration block.
    // 	}
    // }
    p.at(T!['{']) || is_at_declaration(p) || p.at(T!['}'])
}

const KEYFRAMES_ITEM_SELECTOR_IDENT_SET: TokenSet<CssSyntaxKind> = token_set!(T![from], T![to]);

fn is_at_keyframes_item_selector(p: &mut CssParser) -> bool {
    p.at_ts(KEYFRAMES_ITEM_SELECTOR_IDENT_SET) || is_at_percentage_dimension(p)
}
#[inline]
fn parse_keyframes_item_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_item_selector(p) {
        return Absent;
    }

    let m = p.start();

    let kind = if is_at_percentage_dimension(p) {
        parse_percentage_dimension(p).ok();
        CSS_KEYFRAMES_PERCENTAGE_SELECTOR
    } else {
        p.bump_ts(KEYFRAMES_ITEM_SELECTOR_IDENT_SET);
        CSS_KEYFRAMES_IDENT_SELECTOR
    };

    Present(m.complete(p, kind))
}
