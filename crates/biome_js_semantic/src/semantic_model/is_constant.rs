use biome_js_syntax::{AnyJsExpression, JsSyntaxKind};
use biome_rowan::AstNode;

pub fn is_constant(expr: &AnyJsExpression) -> bool {
    for node in expr.syntax().descendants() {
        if matches!(
            node.kind(),
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
        ) {
            return false;
        }
        if matches!(node.kind(), JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION)
            && !node
                .children_with_tokens()
                .nth(1)
                .is_some_and(|child| matches!(child.kind(), JsSyntaxKind::EQ))
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{JsFileSource, JsIdentifierBinding, JsVariableDeclarator};

    use crate::{SemanticModelOptions, semantic_model};

    fn assert_is_const(code: &str, is_const: bool) {
        use biome_rowan::AstNode;
        use biome_rowan::SyntaxNodeCast;
        let r = biome_js_parser::parse(code, JsFileSource::js_module(), JsParserOptions::default());
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let a_reference = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.to_trimmed_string() == "a")
            .unwrap();
        let declarator = a_reference.parent::<JsVariableDeclarator>().unwrap();
        let initializer = declarator.initializer().unwrap();
        let expr = initializer.expression().ok().unwrap();

        assert_eq!(model.is_constant(&expr), is_const, "{code}");
    }

    #[test]
    pub fn ok_semantic_model_is_constant() {
        assert_is_const("const a = 1;", true);
        assert_is_const("const a = 1 + 1;", true);
        assert_is_const("const a = \"a\";", true);
        assert_is_const("const a = b = 1;", true);

        assert_is_const("const a = 1 + f();", false);
        assert_is_const("const a = `${a}`;", false);
        assert_is_const("const a = b = 1 + f();", false);
    }

    #[test]
    pub fn ok_semantic_model_is_constant_with_pre_post_updates() {
        assert_is_const("const a = b++;", false);
        assert_is_const("const a = b--;", false);
        assert_is_const("const a = ++b;", false);
        assert_is_const("const a = --b;", false);
    }

    #[test]
    pub fn ok_semantic_model_is_constant_with_assignments() {
        assert_is_const("const a = (b = 1);", true);
        assert_is_const("const a = (b += 1);", false);
        assert_is_const("const a = (b -= 1);", false);
        assert_is_const("const a = (b *= 1);", false);
        assert_is_const("const a = (b /= 1);", false);
        assert_is_const("const a = (b %= 1);", false);
        assert_is_const("const a = (b **= 1);", false);
        assert_is_const("const a = (b &= 1);", false);
        assert_is_const("const a = (b |= 1);", false);
        assert_is_const("const a = (b ^= 1);", false);
        assert_is_const("const a = (b <<= 1);", false);
        assert_is_const("const a = (b >>= 1);", false);
        assert_is_const("const a = (b >>>= 1);", false);
        assert_is_const("const a = (b &&= 1);", false);
        assert_is_const("const a = (b ||= 1);", false);
        assert_is_const("const a = (b ??= 1);", false);
    }
}
