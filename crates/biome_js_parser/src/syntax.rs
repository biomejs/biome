//! The Js syntax itself and parser functions.
//!
//! The actual parsing is done in these modules.
//! Every single function is public, this is to allow people to
//! use the parser for their specific needs, for example, parsing
//! only an expression.
//!
//! Functions emit markers, see `CompletedMarker` and `Marker` docs for more info.

use crate::{JsParser, prelude::*};
use biome_js_syntax::JsSyntaxKind::EOF;
use biome_languages::JsFileSource;

/// A JavaScript grammar production used to validate names embedded in host syntax.
pub enum ModuleNameSyntax {
    BindingPattern,
    BindingIdentifier,
    ReferenceIdentifier,
}

impl ModuleNameSyntax {
    /// Requires a complete match under the strict-mode rules of a module.
    pub fn accepts(&self, source: &str) -> bool {
        let mut parser = JsParser::new(source, JsFileSource::js_module(), Default::default());
        if matches!(self, Self::BindingPattern | Self::BindingIdentifier) {
            parser.state_mut().duplicate_binding_parent = Some("let");
        }
        let parsed = match self {
            Self::ReferenceIdentifier => expr::parse_reference_identifier(&mut parser),
            Self::BindingIdentifier => binding::parse_identifier_binding(&mut parser),
            Self::BindingPattern => {
                binding::parse_binding_pattern(&mut parser, expr::ExpressionContext::default())
            }
        };
        parsed.is_present() && parser.at(EOF) && parser.finish().2.is_empty()
    }
}

mod assignment;
mod auxiliary;
mod binding;
mod class;
pub mod expr;
mod function;
mod js_parse_error;
mod jsx;
mod metavariable;
mod module;
mod object;
mod pattern;
pub mod program;
mod stmt;
mod typescript;
