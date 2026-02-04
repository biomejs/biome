//! Top level functions for parsing a script or module, also includes module specific items.

use super::expr::{ExpressionContext, parse_expression};
use super::module::parse_module_body;
use super::stmt::parse_statements;
use crate::JsParser;
use crate::prelude::*;
use crate::state::{ChangeParserState, EnableStrictMode};
use crate::syntax::js_parse_error;
use crate::syntax::stmt::parse_directives;
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::ModuleKind;

// test_err js unterminated_unicode_codepoint
// let s = "\u{200";

pub(crate) fn parse(p: &mut JsParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);
    p.eat(JS_SHEBANG);

    // Handle template expressions (Vue {{ }}, Svelte { }, Astro { })
    // These should be parsed as expressions, not as modules with statements
    if p.source_type().is_template_expression() {
        return parse_template_expression(p, m);
    }

    let (statement_list, strict_snapshot) = parse_directives(p);

    let result = match p.source_type().module_kind() {
        ModuleKind::Script => {
            parse_statements(p, false, statement_list);
            m.complete(p, JS_SCRIPT)
        }
        ModuleKind::Module => {
            parse_module_body(p, statement_list);
            m.complete(
                p,
                if p.source_type().language().is_definition_file() {
                    TS_DECLARATION_MODULE
                } else {
                    JS_MODULE
                },
            )
        }
    };

    if let Some(strict_snapshot) = strict_snapshot {
        EnableStrictMode::restore(p.state_mut(), strict_snapshot);
    }

    result
}

/// Parses template expressions like Vue {{ expr }}, Svelte { expr }, or Astro { expr }.
/// These should always parse as expressions, never as statements.
/// This fixes issues where `{ duration }` was incorrectly parsed as a block statement
/// instead of as an object literal expression.
fn parse_template_expression(p: &mut JsParser, m: Marker) -> CompletedMarker {
    // Parse as a single expression with default context
    // This allows { } to be parsed as object literals, not block statements
    let expr_marker = p.start();
    let expr_result = parse_expression(p, ExpressionContext::default());

    if expr_result.is_absent() {
        p.error(js_parse_error::template_expression_expected_expression(
            p,
            p.cur_range(),
        ));
    }

    // Template expressions should only contain one expression
    // Any trailing tokens are an error
    if !p.at(EOF) {
        p.error(js_parse_error::template_expression_trailing_code(
            p,
            p.cur_range(),
        ));

        // Consume remaining tokens to ensure we reach EOF
        // These will be part of the bogus expression
        while !p.at(EOF) {
            p.bump_any();
        }

        // If we had trailing code, wrap everything in a bogus expression
        if !expr_result.is_absent() {
            expr_marker.complete(p, JS_BOGUS_EXPRESSION);
        } else {
            expr_marker.abandon(p);
        }
    } else {
        // Clean expression, no extra wrapping needed
        expr_marker.abandon(p);
    }

    // Use JS_EXPRESSION_TEMPLATE_ROOT as the root node type
    // This is a specialized root for template expressions that only contains an expression and EOF
    m.complete(p, JS_EXPRESSION_TEMPLATE_ROOT)
}
