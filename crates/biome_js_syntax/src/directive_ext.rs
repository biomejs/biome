use biome_rowan::{SyntaxResult, TokenText};

use crate::{JsDirective, inner_string_text};

impl JsDirective {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::syntax::{JsDirective, JsSyntaxKind::*};
    /// use biome_js_factory::JsSyntaxTreeBuilder;
    /// use biome_rowan::AstNode;
    /// let mut tree_builder = JsSyntaxTreeBuilder::new();
    ///         tree_builder.start_node(JS_DIRECTIVE);
    ///         tree_builder.token(JS_STRING_LITERAL, "\"use strict\"");
    ///         tree_builder.finish_node();
    ///         let node = tree_builder.finish();
    ///         let js_directive = JsDirective::cast(node).unwrap();
    ///         let text = js_directive.inner_string_text().unwrap();
    ///         assert_eq!(text, "use strict")
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

#[cfg(test)]
mod tests {
    use biome_js_factory::JsSyntaxTreeBuilder;
    use biome_js_factory::syntax::{JsDirective, JsSyntaxKind::*};
    use biome_rowan::AstNode;

    #[test]
    fn js_directive_inner_string_text() {
        let tokens = vec!["\"use strict\"", "'use strict'"];
        for token in tokens {
            let mut tree_builder = JsSyntaxTreeBuilder::new();
            tree_builder.start_node(JS_DIRECTIVE);
            tree_builder.token(JS_STRING_LITERAL, token);
            tree_builder.finish_node();

            let node = tree_builder.finish();
            let js_directive = JsDirective::cast(node).unwrap();
            let text = js_directive.inner_string_text().unwrap();
            assert_eq!(text, "use strict")
        }
    }
}
