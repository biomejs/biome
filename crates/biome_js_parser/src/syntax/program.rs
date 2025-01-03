//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::stmt::parse_statements;
use crate::prelude::*;
use crate::state::{ChangeParserState, EnableStrictMode};
use crate::syntax::stmt::parse_directives;
use crate::JsParser;
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::ModuleKind;

// test_err js unterminated_unicode_codepoint
// let s = "\u{200";

pub(crate) fn parse(p: &mut JsParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);
    p.eat(JS_SHEBANG);

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
