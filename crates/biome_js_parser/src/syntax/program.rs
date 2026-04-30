//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::stmt::parse_statements;
use crate::JsParser;
use crate::prelude::*;
use crate::state::{ChangeParserState, EnableStrictMode, SignatureFlags};
use crate::syntax::binding::parse_binding;
use crate::syntax::expr::{ExpressionContext, parse_expression};
use crate::syntax::function::{ParameterContext, parse_parameter_list};
use crate::syntax::js_parse_error;
use crate::syntax::stmt::parse_directives;
use crate::syntax::typescript::TypeContext;
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
    if p.source_type()
        .as_embedding_kind()
        .is_svelte_function_signature()
    {
        return parse_snippet_signature(p, m);
    }
    // Parse as a single expression with default context
    // This allows { } to be parsed as object literals, not block statements
    let expr_marker = p.start();
    let expr_result = parse_expression(p, ExpressionContext::default());

    // Check if we got a valid expression
    let has_expression = !expr_result.is_absent();

    if !has_expression {
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
        while !p.at(EOF) {
            p.bump_any();
        }

        // Wrap everything in a bogus expression if we had parse errors
        expr_marker.complete(p, JS_BOGUS_EXPRESSION);
    } else if !has_expression {
        // No expression and at EOF, create an empty bogus expression
        expr_marker.complete(p, JS_BOGUS_EXPRESSION);
    } else {
        // Valid expression, no wrapping needed
        expr_marker.abandon(p);
    }

    // Always complete as JS_EXPRESSION_TEMPLATE_ROOT
    // The expression child might be bogus, but the root should always be this type
    m.complete(p, JS_EXPRESSION_TEMPLATE_ROOT)
}

/// Parses a Svelte snippet declaration: `add(a: any, b: float)`.
/// Produces a JsIdentifierBinding for the name and JsParameters for the
/// parameter list, wrapped in a JsSnippetSignatureTemplateRoot.
fn parse_snippet_signature(p: &mut JsParser, m: Marker) -> CompletedMarker {
    parse_binding(p).or_add_diagnostic(p, js_parse_error::expected_binding);

    // These are not mandatory
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        TypeContext::default(),
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, js_parse_error::expected_class_parameters);

    if !p.at(EOF) {
        p.error(js_parse_error::template_expression_trailing_code(
            p,
            p.cur_range(),
        ));
        while !p.at(EOF) {
            p.bump_any();
        }
    }

    m.complete(p, JS_SVELTE_SNIPPET_ROOT)
}
