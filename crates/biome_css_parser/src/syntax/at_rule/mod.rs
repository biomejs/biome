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
mod property;
mod scope;
mod starting_style;
mod supports;

use crate::parser::CssParser;
use crate::syntax::at_rule::charset::{is_at_charset_at_rule, parse_charset_at_rule};
use crate::syntax::at_rule::color_profile::{
    is_at_color_profile_at_rule, parse_color_profile_at_rule,
};
use crate::syntax::at_rule::container::{is_at_container_at_rule, parse_container_at_rule};
use crate::syntax::at_rule::counter_style::{
    is_at_counter_style_at_rule, parse_counter_style_at_rule,
};
use crate::syntax::at_rule::document::{is_at_document_at_rule, parse_document_at_rule};
use crate::syntax::at_rule::font_face::{is_at_font_face_at_rule, parse_font_face_at_rule};
use crate::syntax::at_rule::font_feature_values::{
    is_at_font_feature_values_at_rule, parse_font_feature_values_at_rule,
};
use crate::syntax::at_rule::font_palette_values::{
    is_at_font_palette_values_at_rule, parse_font_palette_values_at_rule,
};
use crate::syntax::at_rule::import::{is_at_import_at_rule, parse_import_at_rule};
use crate::syntax::at_rule::keyframes::{is_at_keyframes_at_rule, parse_keyframes_at_rule};
use crate::syntax::at_rule::layer::{is_at_layer_at_rule, parse_layer_at_rule};
use crate::syntax::at_rule::media::{is_at_media_at_rule, parse_media_at_rule};
use crate::syntax::at_rule::namespace::{is_at_namespace_at_rule, parse_namespace_at_rule};
use crate::syntax::at_rule::page::{is_at_page_at_rule, parse_page_at_rule};
use crate::syntax::at_rule::scope::{is_at_scope_at_rule, parse_scope_at_rule};
use crate::syntax::at_rule::starting_style::{
    is_at_starting_style_at_rule, parse_starting_style_at_rule,
};
use crate::syntax::at_rule::supports::{is_at_supports_at_rule, parse_supports_at_rule};
use crate::syntax::parse_error::expected_any_at_rule;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use self::property::{is_at_property_at_rule, parse_property_at_rule};

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

    let kind = if parse_any_at_rule(p)
        .or_add_diagnostic(p, expected_any_at_rule)
        .is_some()
    {
        CSS_AT_RULE
    } else {
        CSS_BOGUS_RULE
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn parse_any_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if is_at_charset_at_rule(p) {
        parse_charset_at_rule(p)
    } else if is_at_color_profile_at_rule(p) {
        parse_color_profile_at_rule(p)
    } else if is_at_counter_style_at_rule(p) {
        parse_counter_style_at_rule(p)
    } else if is_at_container_at_rule(p) {
        parse_container_at_rule(p)
    } else if is_at_font_face_at_rule(p) {
        parse_font_face_at_rule(p)
    } else if is_at_font_feature_values_at_rule(p) {
        parse_font_feature_values_at_rule(p)
    } else if is_at_font_palette_values_at_rule(p) {
        parse_font_palette_values_at_rule(p)
    } else if is_at_media_at_rule(p) {
        parse_media_at_rule(p)
    } else if is_at_keyframes_at_rule(p) {
        parse_keyframes_at_rule(p)
    } else if is_at_page_at_rule(p) {
        parse_page_at_rule(p)
    } else if is_at_layer_at_rule(p) {
        parse_layer_at_rule(p)
    } else if is_at_scope_at_rule(p) {
        parse_scope_at_rule(p)
    } else if is_at_supports_at_rule(p) {
        parse_supports_at_rule(p)
    } else if is_at_import_at_rule(p) {
        parse_import_at_rule(p)
    } else if is_at_namespace_at_rule(p) {
        parse_namespace_at_rule(p)
    } else if is_at_starting_style_at_rule(p) {
        parse_starting_style_at_rule(p)
    } else if is_at_document_at_rule(p) {
        parse_document_at_rule(p)
    } else if is_at_property_at_rule(p) {
        parse_property_at_rule(p)
    } else {
        Absent
    }
}
