use super::pseudo_class_nth::is_nth_at_scss_pseudo_class_nth_value_with_interpolation;
use crate::parser::CssParser;
use crate::syntax::scss::is_at_scss_interpolation;
use crate::syntax::selector::relative_selector::{
    RelativeSelectorList, is_at_relative_selector_combinator,
};
use crate::syntax::selector::{
    PSEUDO_CLASS_NTH_SIGN_SET, PseudoValueList, SELECTOR_FUNCTION_RECOVERY_SET, SelectorList,
    is_at_pseudo_class_nth_argument, is_at_pseudo_value, parse_pseudo_class_nth_selector,
};
use biome_css_syntax::CssSyntaxKind::{
    IDENT, SCSS_INTERPOLATED_PSEUDO_CLASS_NTH_ARGUMENTS,
    SCSS_INTERPOLATED_PSEUDO_CLASS_RELATIVE_SELECTOR_ARGUMENTS,
    SCSS_INTERPOLATED_PSEUDO_CLASS_SELECTOR_ARGUMENTS,
    SCSS_INTERPOLATED_PSEUDO_CLASS_VALUE_ARGUMENTS,
    SCSS_INTERPOLATED_PSEUDO_ELEMENT_SELECTOR_ARGUMENTS,
    SCSS_INTERPOLATED_PSEUDO_ELEMENT_VALUE_ARGUMENTS,
};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, token_set};

#[inline]
pub(crate) fn is_at_scss_interpolated_pseudo_class_function_arguments(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_pseudo_relative_selector_arguments(p)
        || is_at_scss_interpolated_pseudo_nth_arguments(p)
        || is_at_scss_interpolated_pseudo_selector_arguments(p)
        || is_at_scss_interpolated_pseudo_value_arguments(p)
}

/// Parses typed arguments for an interpolated pseudo-class function.
///
/// Examples:
/// ```scss
/// :#{$name}(> img) {}
/// :#{$name}(.item) {}
/// :#{$name}(2n + #{$offset}) {}
/// :#{$name}(en, #{$locale}) {}
/// ```
///
/// The resolved pseudo name is unknown, so this chooses the most specific
/// reusable list grammar from the current argument head.
///
/// Docs: https://sass-lang.com/documentation/interpolation/
pub(crate) fn parse_scss_interpolated_pseudo_class_function_arguments(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_class_function_arguments(p) {
        return Absent;
    }

    if is_at_scss_interpolated_pseudo_relative_selector_arguments(p) {
        return parse_scss_interpolated_pseudo_class_relative_selector_arguments(p);
    }

    if is_at_scss_interpolated_pseudo_nth_arguments(p) {
        return parse_scss_interpolated_pseudo_class_nth_arguments(p);
    }

    if is_at_scss_interpolated_pseudo_selector_arguments(p) {
        return parse_scss_interpolated_pseudo_class_selector_arguments(p);
    }

    if is_at_scss_interpolated_pseudo_value_arguments(p) {
        return parse_scss_interpolated_pseudo_class_value_arguments(p);
    }

    Absent
}

/// Parses `:#{$name}(> img)` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_class_relative_selector_arguments(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_relative_selector_arguments(p) {
        return Absent;
    }

    let m = p.start();
    RelativeSelectorList::new(T![')'])
        .disable_recovery()
        .parse_list(p);
    Present(m.complete(
        p,
        SCSS_INTERPOLATED_PSEUDO_CLASS_RELATIVE_SELECTOR_ARGUMENTS,
    ))
}

/// Parses `:#{$name}(2n + #{$offset})` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_class_nth_arguments(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_nth_arguments(p) {
        return Absent;
    }

    let m = p.start();
    let Present(_) = parse_pseudo_class_nth_selector(p) else {
        return Absent;
    };

    Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_CLASS_NTH_ARGUMENTS))
}

/// Parses `:#{$name}(.item, [hidden])` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_class_selector_arguments(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_selector_arguments(p) {
        return Absent;
    }

    let m = p.start();
    SelectorList::default()
        .with_end_kind_ts(token_set!(T![')']))
        .with_recovery_ts(SELECTOR_FUNCTION_RECOVERY_SET)
        .parse_list(p);
    Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_CLASS_SELECTOR_ARGUMENTS))
}

/// Parses `:#{$name}(en, #{$locale})` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_class_value_arguments(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_value_arguments(p) {
        return Absent;
    }

    let m = p.start();
    PseudoValueList.parse_list(p);
    Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_CLASS_VALUE_ARGUMENTS))
}

#[inline]
pub(crate) fn is_at_scss_interpolated_pseudo_element_function_arguments(p: &mut CssParser) -> bool {
    is_at_scss_interpolated_pseudo_selector_arguments(p)
        || is_at_scss_interpolated_pseudo_value_arguments(p)
}

/// Parses typed arguments for an interpolated pseudo-element function.
///
/// Examples:
/// ```scss
/// ::#{$name}(.item) {}
/// ::#{$name}(#{$part}) {}
/// ```
///
/// The resolved pseudo-element name is unknown, so this chooses the most
/// specific reusable list grammar from the current argument head.
///
/// Docs: https://sass-lang.com/documentation/interpolation/
pub(crate) fn parse_scss_interpolated_pseudo_element_function_arguments(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_element_function_arguments(p) {
        return Absent;
    }

    if is_at_scss_interpolated_pseudo_selector_arguments(p) {
        return parse_scss_interpolated_pseudo_element_selector_arguments(p);
    }

    if is_at_scss_interpolated_pseudo_value_arguments(p) {
        return parse_scss_interpolated_pseudo_element_value_arguments(p);
    }

    Absent
}

/// Parses `::#{$name}(.item, [hidden])` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_element_selector_arguments(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_selector_arguments(p) {
        return Absent;
    }

    let m = p.start();
    SelectorList::default()
        .with_end_kind_ts(token_set!(T![')']))
        .with_recovery_ts(SELECTOR_FUNCTION_RECOVERY_SET)
        .parse_list(p);
    Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_ELEMENT_SELECTOR_ARGUMENTS))
}

/// Parses `::#{$name}(#{$part})` arguments.
#[inline]
fn parse_scss_interpolated_pseudo_element_value_arguments(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_pseudo_value_arguments(p) {
        return Absent;
    }

    let m = p.start();
    PseudoValueList.parse_list(p);
    Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_ELEMENT_VALUE_ARGUMENTS))
}

#[inline]
fn is_at_scss_interpolated_pseudo_relative_selector_arguments(p: &mut CssParser) -> bool {
    // `:#{$name}(+ .item)` is relative selector args; `:#{$name}(+2n)` is nth args.
    is_at_relative_selector_combinator(p) && !is_at_scss_interpolated_pseudo_nth_arguments(p)
}

#[inline]
fn is_at_scss_interpolated_pseudo_nth_arguments(p: &mut CssParser) -> bool {
    let n = if p.at_ts(PSEUDO_CLASS_NTH_SIGN_SET) {
        1
    } else {
        0
    };

    is_nth_at_scss_pseudo_class_nth_value_with_interpolation(p, n)
        || is_at_pseudo_class_nth_argument(p)
}

#[inline]
fn is_at_scss_interpolated_pseudo_selector_arguments(p: &mut CssParser) -> bool {
    // `:#{$name}(#id)` is selector args; `::#{$name}(#{$part})` is value args.
    let is_at_id_selector = p.at(T![#]) && !is_at_scss_interpolation(p);

    p.at_ts(token_set![T![.], T![:], T![::], T!['['], T![*], T![|]]) || is_at_id_selector
}

#[inline]
fn is_at_scss_interpolated_pseudo_value_arguments(p: &mut CssParser) -> bool {
    is_at_pseudo_value(p) && !is_at_scss_interpolated_pseudo_nested_function_value(p)
}

#[inline]
fn is_at_scss_interpolated_pseudo_nested_function_value(p: &mut CssParser) -> bool {
    // `:#{$name}(foo(bar))` is not a pseudo value; leave `foo(` for `)` recovery.
    p.at(IDENT) && p.nth_at(1, T!['('])
}
