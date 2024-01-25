use super::UtilityClassSortingOptions;
use biome_js_syntax::{
    JsCallArguments, JsCallExpression, JsStringLiteralExpression, JsTemplateChunkElement,
    JsxAttribute, JsxString,
};
use biome_rowan::{declare_node_union, AstNode, TokenText};

fn get_callee_name(call_expression: &JsCallExpression) -> Option<TokenText> {
    call_expression
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .name()
        .ok()
}

fn is_call_expression_of_target_function(
    call_expression: &JsCallExpression,
    options: &UtilityClassSortingOptions,
) -> bool {
    get_callee_name(call_expression).is_some_and(|name| options.has_function(name.text()))
}

fn get_attribute_name(attribute: &JsxAttribute) -> Option<TokenText> {
    Some(
        attribute
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

declare_node_union! {
    /// A string literal, JSX string, or template chunk representing a CSS class string.
    pub AnyClassStringLike = JsStringLiteralExpression | JsxString | JsTemplateChunkElement
}

impl AnyClassStringLike {
    pub(crate) fn should_visit(&self, options: &UtilityClassSortingOptions) -> Option<bool> {
        match self {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let mut in_arguments = false;
                let mut in_function = false;
                for ancestor in string_literal.syntax().ancestors().skip(1) {
                    if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
                        let attribute_name = get_attribute_name(&jsx_attribute)?;
                        if options.has_attribute(attribute_name.text()) {
                            return Some(true);
                        }
                    }

                    if let Some(call_expression) = JsCallExpression::cast_ref(&ancestor) {
                        in_function =
                            is_call_expression_of_target_function(&call_expression, options);
                    }

                    if JsCallArguments::can_cast(ancestor.kind()) {
                        in_arguments = true;
                    }

                    if in_function && in_arguments {
                        return Some(true);
                    }
                }

                None
            }
            AnyClassStringLike::JsxString(jsx_string) => {
                let jsx_attribute = jsx_string
                    .syntax()
                    .ancestors()
                    .skip(1)
                    .find_map(JsxAttribute::cast)?;
                let name = get_attribute_name(&jsx_attribute)?;
                if options.has_attribute(name.text()) {
                    return Some(true);
                }

                None
            }
            AnyClassStringLike::JsTemplateChunkElement(_) => None,
        }
    }

    pub fn value(&self) -> Option<TokenText> {
        match &self {
            AnyClassStringLike::JsStringLiteralExpression(node) => node.inner_string_text().ok(),
            AnyClassStringLike::JsxString(node) => node.inner_string_text().ok(),
            AnyClassStringLike::JsTemplateChunkElement(template_chunk) => {
                Some(template_chunk.template_chunk_token().ok()?.token_text())
            }
        }
    }
}
