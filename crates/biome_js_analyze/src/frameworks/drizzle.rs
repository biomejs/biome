use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsStaticMemberExpression, JsSyntaxKind};
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxNode, TokenText};

pub(crate) fn get_identifier_name(expr: &AnyJsExpression) -> Option<TokenText> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            Some(id.name().ok()?.value_token().ok()?.token_text_trimmed())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object = member.object().ok()?;
            let object_name = get_identifier_name(&object)?;
            let member_name = member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed();
            let full_name = format!("{}.{}", object_name, member_name);
            Some(TokenText::new_raw(RawSyntaxKind(0), &full_name))
        }
        _ => None,
    }
}

pub(crate) fn has_where_in_chain(node: &SyntaxNode<biome_js_syntax::JsLanguage>) -> bool {
    let mut current = node.parent();
    loop {
        let Some(parent) = current else { break };

        if let Some(member_expr) = JsStaticMemberExpression::cast_ref(&parent)
            && let Ok(member) = member_expr.member()
            && let Some(name) = member.as_js_name()
            && name
                .value_token()
                .ok()
                .is_some_and(|t| t.token_text_trimmed() == "where")
        {
            // Only count `.where(...)` as a where clause, not bare `.where` property access.
            let is_called = parent
                .parent()
                .is_some_and(|p| JsCallExpression::cast_ref(&p).is_some());
            if is_called {
                return true;
            }
        }

        if matches!(
            parent.kind(),
            JsSyntaxKind::JS_EXPRESSION_STATEMENT | JsSyntaxKind::JS_RETURN_STATEMENT
        ) {
            break;
        }

        current = parent.parent();
    }
    false
}
