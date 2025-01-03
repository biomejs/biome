mod charset;
mod color_profile;
mod container;
mod counter_style;
mod document;
mod feature;
mod font_face;
mod font_feature_values;
mod font_palette_values;
mod import;
mod keyframes;
mod layer;
mod media;
mod namespace;
mod page;
mod parse_error;
mod position_try;
mod property;
mod scope;
mod starting_style;
mod supports;
mod unknown;
mod value;
mod view_transition;

use crate::parser::CssParser;
use crate::syntax::at_rule::charset::parse_charset_at_rule;
use crate::syntax::at_rule::color_profile::parse_color_profile_at_rule;
use crate::syntax::at_rule::container::parse_container_at_rule;
use crate::syntax::at_rule::counter_style::parse_counter_style_at_rule;
use crate::syntax::at_rule::document::parse_document_at_rule;
use crate::syntax::at_rule::font_face::parse_font_face_at_rule;
use crate::syntax::at_rule::font_feature_values::parse_font_feature_values_at_rule;
use crate::syntax::at_rule::font_palette_values::parse_font_palette_values_at_rule;
use crate::syntax::at_rule::import::parse_import_at_rule;
use crate::syntax::at_rule::keyframes::parse_keyframes_at_rule;
use crate::syntax::at_rule::layer::parse_layer_at_rule;
use crate::syntax::at_rule::media::parse_media_at_rule;
use crate::syntax::at_rule::namespace::parse_namespace_at_rule;
use crate::syntax::at_rule::page::parse_page_at_rule;
use crate::syntax::at_rule::position_try::parse_position_try_at_rule;
use crate::syntax::at_rule::property::parse_property_at_rule;
use crate::syntax::at_rule::scope::parse_scope_at_rule;
use crate::syntax::at_rule::starting_style::parse_starting_style_at_rule;
use crate::syntax::at_rule::supports::parse_supports_at_rule;
use crate::syntax::at_rule::unknown::{is_at_unknown_at_rule, parse_unknown_at_rule};
use crate::syntax::at_rule::value::parse_value_at_rule;
use crate::syntax::at_rule::view_transition::parse_view_transition_at_rule;
use crate::syntax::parse_error::expected_any_at_rule;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_at_rule(p: &mut CssParser) -> bool {
    p.at(T![@])
}

#[inline]
pub(crate) fn parse_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![@]);

    // Show the error under the token next to the '@'
    let range = p.cur_range();

    let kind = match parse_any_at_rule(p) {
        Present(_) => CSS_AT_RULE,
        Absent => {
            p.error(expected_any_at_rule(p, range));
            CSS_BOGUS_RULE
        }
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn parse_any_at_rule(p: &mut CssParser) -> ParsedSyntax {
    match p.cur() {
        T![charset] => parse_charset_at_rule(p),
        T![color_profile] => parse_color_profile_at_rule(p),
        T![counter_style] => parse_counter_style_at_rule(p),
        T![container] => parse_container_at_rule(p),
        T![font_face] => parse_font_face_at_rule(p),
        T![font_feature_values] => parse_font_feature_values_at_rule(p),
        T![font_palette_values] => parse_font_palette_values_at_rule(p),
        T![media] => parse_media_at_rule(p),
        T![keyframes] => parse_keyframes_at_rule(p),
        T![page] => parse_page_at_rule(p),
        T![layer] => parse_layer_at_rule(p),
        T![scope] => parse_scope_at_rule(p),
        T![supports] => parse_supports_at_rule(p),
        T![import] => parse_import_at_rule(p),
        T![namespace] => parse_namespace_at_rule(p),
        T![starting_style] => parse_starting_style_at_rule(p),
        T![document] => parse_document_at_rule(p),
        T![property] => parse_property_at_rule(p),
        T![value] => parse_value_at_rule(p),
        T![position_try] => parse_position_try_at_rule(p),
        T![view_transition] => parse_view_transition_at_rule(p),
        _ if is_at_unknown_at_rule(p) => parse_unknown_at_rule(p),
        _ => Absent,
    }
}
