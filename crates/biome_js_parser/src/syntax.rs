//! The Js syntax itself and parser functions.
//!
//! The actual parsing is done in these modules.
//! Every single function is public, this is to allow people to
//! use the parser for their specific needs, for example, parsing
//! only an expression.
//!
//! Functions emit markers, see `CompletedMarker` and `Marker` docs for more info.

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
