use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::{
    expected_keyframes_item, expected_keyframes_item_selector,
};
use crate::syntax::block::{parse_declaration_block, ParseBlockBody};
use crate::syntax::css_modules::{
    expected_any_css_module_scope, local_or_global_not_allowed, CSS_MODULES_SCOPE_SET,
};
use crate::syntax::parse_error::expected_non_css_wide_keyword_identifier;
use crate::syntax::value::dimension::{is_at_percentage_dimension, parse_percentage_dimension};
use crate::syntax::{
    is_at_declaration, is_at_identifier, is_at_string, parse_custom_identifier, parse_string,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Checks if the current parser position is at a `@keyframes` at-rule.
///
/// This function determines if the parser is currently positioned at the start of a `@keyframes`
/// rule, which is part of the CSS syntax.
#[inline]
pub(crate) fn is_at_keyframes_at_rule(p: &mut CssParser) -> bool {
    p.at(T![keyframes])
}
/// Parses a `@keyframes` at-rule in CSS.
///
/// This function parses a `@keyframes` rule, which can be scoped locally or globally using the
/// `:local` and `:global` pseudo-classes specific to CSS Modules.
///
/// For more information, see the [CSS Animations Specification](https://drafts.csswg.org/css-animations/#keyframes).
/// # Examples
/// Basic usage in CSS:
/// ```css
/// @keyframes my-animation {
///     from {
///         color: red;
///     }
///     to {
///         color: blue;
///     }
/// }
/// @keyframes :local(my-local-animation) {
///     from {
///         opacity: 0;
///     }
///     to {
///         opacity: 1;
///     }
/// }
/// @keyframes :global "my-global-animation" {
///     from {
///         opacity: 0;
///     }
///     to {
///         opacity: 1;
///     }
/// }
/// ```
#[inline]
pub(crate) fn parse_keyframes_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![keyframes]);

    if is_at_keyframes_scoped_name(p) {
        // is_at_keyframes_scoped_name guaranties that it will parse a keyframes scoped name
        parse_keyframes_scoped_name(p).ok();
    } else {
        parse_keyframes_identifier(p)
            .or_add_diagnostic(p, expected_non_css_wide_keyword_identifier);
    };

    KeyframesBlock.parse_block_body(p);

    Present(m.complete(p, CSS_KEYFRAMES_AT_RULE))
}

/// Checks if the current parser position is at a keyframes scoped name.
///
/// This function determines if the parser is currently positioned at the start of a keyframes scoped name,
/// which is indicated by the presence of a `:` character.
fn is_at_keyframes_scoped_name(p: &mut CssParser) -> bool {
    p.at(T![:])
}

/// Parses a keyframes scoped name in CSS Modules.
///
/// This function parses a keyframes scoped name, which can be either `:local` or `:global`,
/// specific to CSS Modules. If CSS Modules are not enabled, it generates a diagnostic error and skips the scoped name.
fn parse_keyframes_scoped_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_scoped_name(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);

    if p.options().is_css_modules_disabled() {
        // :local and :global are not standard CSS features
        // provide a hint on how to enable parsing of these pseudo-classes
        p.error(local_or_global_not_allowed(p, p.cur_range()));

        // Skip the entire pseudo-class function selector
        // Skip until the next opening curly brace
        while !p.at(T!['{']) {
            p.bump_any();
        }

        return Present(m.complete(p, CSS_BOGUS_KEYFRAMES_NAME));
    }

    let kind = {
        let m = p.start();

        // If we are at an invalid CSS module scope,
        // we generate a diagnostic error and skip the invalid scope.
        if !p.eat_ts(CSS_MODULES_SCOPE_SET) {
            p.error(expected_any_css_module_scope(p, p.cur_range()));
            p.bump_any();
        }

        let kind = if p.eat(T!['(']) {
            CSS_KEYFRAMES_SCOPE_FUNCTION
        } else {
            CSS_KEYFRAMES_SCOPE_PREFIX
        };

        let name = parse_keyframes_identifier(p)
            .or_add_diagnostic(p, expected_non_css_wide_keyword_identifier);

        if kind == CSS_KEYFRAMES_SCOPE_FUNCTION {
            // If we have a function, we expect a closing parenthesis
            p.expect(T![')']);
        }

        m.complete(p, kind);

        if name.is_some() {
            CSS_KEYFRAMES_SCOPED_NAME
        } else {
            // if we have an invalid name return a bogus keyframes name
            CSS_BOGUS_KEYFRAMES_NAME
        }
    };

    Present(m.complete(p, kind))
}

/// Checks if the current parser position is at a keyframes identifier.
///
/// This function determines if the parser is currently positioned at the start of a keyframes identifier,
/// which can be either a standard identifier or a string.
fn is_at_keyframes_identifier(p: &mut CssParser) -> bool {
    is_at_identifier(p) || is_at_string(p)
}

/// Parses a keyframes identifier in CSS.
/// This function parses a keyframes identifier, which can be either a standard identifier or a string.
fn parse_keyframes_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_keyframes_identifier(p) {
        return Absent;
    }

    if is_at_identifier(p) {
        parse_custom_identifier(p, CssLexContext::Regular)
    } else {
        parse_string(p)
    }
}

struct KeyframesBlock;

impl ParseBlockBody for KeyframesBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_KEYFRAMES_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_keyframes_item_selector(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        KeyframesItemList.parse_list(p);
    }
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

    parse_declaration_block(p);

    Present(m.complete(p, CSS_KEYFRAMES_ITEM))
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
        p.at(T!['{'])
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

/// A set of tokens representing the keyframes item selectors `from` and `to`.
const KEYFRAMES_ITEM_SELECTOR_IDENT_SET: TokenSet<CssSyntaxKind> = token_set!(T![from], T![to]);

/// Checks if the current parser position is at a keyframes item selector.
///
/// This function determines if the parser is currently positioned at the start of a keyframes item selector,
/// which can be either `from`, `to`, or a percentage dimension.
fn is_at_keyframes_item_selector(p: &mut CssParser) -> bool {
    p.at_ts(KEYFRAMES_ITEM_SELECTOR_IDENT_SET) || is_at_percentage_dimension(p)
}

/// Parses a keyframes item selector in CSS.
///
/// This function parses a keyframes item selector, which can be either `from`, `to`, or a percentage dimension.
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
