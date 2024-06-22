use super::GritTargetLanguageImpl;
use crate::grit_target_node::GritTargetSyntaxKind;
use biome_js_syntax::JsSyntaxKind;
use biome_parser::{token_set, TokenSet};

const COMMENT_KINDS: TokenSet<JsSyntaxKind> =
    token_set![JsSyntaxKind::COMMENT, JsSyntaxKind::MULTILINE_COMMENT];

#[derive(Clone, Debug)]
pub struct JsTargetLanguage;

impl GritTargetLanguageImpl for JsTargetLanguage {
    type Kind = JsSyntaxKind;

    fn language_name(&self) -> &'static str {
        "JavaScript"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("import ", " from 'GRIT_PACKAGE';"),
            ("GRIT_VALUE ", " GRIT_VALUE"),
            ("class GRIT_CLASS ", " {}"),
            ("class GRIT_CLASS { ", " GRIT_PROP = 'GRIT_VALUE'; }"),
            ("", "  function GRIT_FUNCTION() {}"),
            ("GRIT_OBJ = { ", " }"),
            ("class GRIT_CLASS { ", " }"),
            ("GRIT_VAR = ", ""),
            ("<f>", "</f>"),
            ("<f ", " />"),
            ("function GRIT_FN(", ") {}"),
            ("var ", ";"),
            ("", " class GRIT_CLASS {}"),
            ("function GRIT_FN(GRIT_ARG:", ") { }"),
            ("import { ", " } from 'GRIT_PACKAGE'"),
            ("function GRIT_FN(GRIT_ARG", ") { }"),
            ("GRIT_FN<{ ", " }>();"),
        ]
    }

    fn is_comment_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_js_kind()
            .map_or(false, |kind| COMMENT_KINDS.contains(kind))
    }

    fn metavariable_kind() -> Self::Kind {
        JsSyntaxKind::JS_GRIT_METAVARIABLE
    }

    fn is_alternative_metavariable_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_js_kind().map_or(false, |kind| {
            kind == JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST
                || kind == JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST
        })
    }
}
