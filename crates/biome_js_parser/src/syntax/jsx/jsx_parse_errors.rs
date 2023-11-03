use crate::prelude::*;
use crate::JsParser;
use biome_js_syntax::TextRange;
use biome_parser::diagnostic::{expected_any, expected_node};

pub(crate) fn jsx_expected_attribute(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("JSX attribute", range, p)
}

pub(crate) fn jsx_expected_attribute_value(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("JSX attribute value", range, p)
}

pub(crate) fn jsx_expected_children(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["JSX Expression", "Element", "text"], range, p)
}

pub(crate) fn jsx_expected_closing_tag(
    p: &JsParser,
    opening_name: &str,
    opening_range: TextRange,
    closing_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Expected corresponding JSX closing tag for '{opening_name}'."),
        opening_range,
    )
    .with_detail(opening_range, "Opening tag")
    .with_detail(closing_range, "closing tag")
}
