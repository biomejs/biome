//! Shared utilities for working with CSS class strings in JSX/JS.
//!
//! This module provides the `AnyClassStringLike` union type that represents
//! various AST nodes that can contain CSS class strings (string literals,
//! JSX strings, template chunks, etc.).

use biome_js_syntax::{
    AnyJsExpression, JsCallArguments, JsCallExpression, JsLiteralMemberName,
    JsStringLiteralExpression, JsSyntaxNode, JsTemplateChunkElement, JsTemplateExpression,
    JsxAttribute, JsxString,
};
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::no_duplicate_classes::NoDuplicateClassesOptions;
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;

/// Trait for option types that specify which class attributes and functions to check.
pub trait ClassStringOptions {
    fn has_attribute(&self, name: &str) -> bool;
    fn has_function(&self, name: &str) -> bool;
    fn match_function(&self, name: &str) -> bool;
}

impl ClassStringOptions for UseSortedClassesOptions {
    fn has_attribute(&self, name: &str) -> bool {
        self.has_attribute(name)
    }
    fn has_function(&self, name: &str) -> bool {
        self.has_function(name)
    }
    fn match_function(&self, name: &str) -> bool {
        self.match_function(name)
    }
}

impl ClassStringOptions for NoDuplicateClassesOptions {
    fn has_attribute(&self, name: &str) -> bool {
        (**self).has_attribute(name)
    }
    fn has_function(&self, name: &str) -> bool {
        (**self).has_function(name)
    }
    fn match_function(&self, name: &str) -> bool {
        (**self).match_function(name)
    }
}

const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

impl ClassStringOptions for NoTailwindArbitraryValueOptions {
    fn has_attribute(&self, name: &str) -> bool {
        CLASS_ATTRIBUTES.contains(&name)
            || self.attributes.iter().flatten().any(|v| v.as_ref() == name)
    }
    fn has_function(&self, name: &str) -> bool {
        self.functions.iter().flatten().any(|v| v.as_ref() == name)
    }
    fn match_function(&self, name: &str) -> bool {
        self.functions.iter().flatten().any(|matcher| {
            let mut matcher_parts = matcher.split('.');
            let mut name_parts = name.split('.');
            let all_parts_match = matcher_parts
                .by_ref()
                .zip(name_parts.by_ref())
                .all(|(m, p)| m == "*" || m == p);
            all_parts_match && matcher_parts.next().is_none() && name_parts.next().is_none()
        })
    }
}

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
    options: &impl ClassStringOptions,
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
    pub AnyClassStringLike = JsStringLiteralExpression | JsxString | JsTemplateChunkElement | JsLiteralMemberName
}

fn inspect_string_literal(node: &JsSyntaxNode, options: &impl ClassStringOptions) -> Option<bool> {
    let mut in_arguments = false;
    let mut in_function = false;
    for ancestor in node.ancestors().skip(1) {
        if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
            let attribute_name = get_attribute_name(&jsx_attribute)?;
            if options.has_attribute(attribute_name.text()) {
                return Some(true);
            }
        }

        if let Some(call_expression) = JsCallExpression::cast_ref(&ancestor) {
            in_function = is_call_expression_of_target_function(&call_expression, options);
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

impl AnyClassStringLike {
    /// Check if this node should be visited based on the given options.
    ///
    /// Returns `Some(true)` if the node is in a context that should be checked
    /// (e.g., a `class` or `className` attribute, or inside a utility function call).
    /// Returns `None` if the node should be skipped.
    pub fn should_visit(&self, options: &impl ClassStringOptions) -> Option<bool> {
        match self {
            Self::JsStringLiteralExpression(string_literal) => {
                inspect_string_literal(string_literal.syntax(), options)
            }
            Self::JsLiteralMemberName(literal_name) => {
                inspect_string_literal(literal_name.syntax(), options)
            }
            Self::JsxString(jsx_string) => {
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
            Self::JsTemplateChunkElement(template) => {
                for ancestor in template.syntax().ancestors().skip(1) {
                    if let Some(template_expression) = JsTemplateExpression::cast_ref(&ancestor) {
                        if let Some(AnyJsExpression::JsIdentifierExpression(tag)) =
                            template_expression.tag()
                        {
                            let name = tag.name().ok()?.name().ok()?;
                            if options.has_function(name.text()) {
                                return Some(true);
                            }
                        }
                        if let Some(AnyJsExpression::JsStaticMemberExpression(tag)) =
                            template_expression.tag()
                            && options.match_function(&tag.syntax().text_trimmed().to_string())
                        {
                            return Some(true);
                        }
                    } else if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
                        let attribute_name = get_attribute_name(&jsx_attribute)?;
                        if options.has_attribute(attribute_name.text()) {
                            return Some(true);
                        }
                    }
                }

                None
            }
        }
    }

    /// Get the text value of this class string node.
    pub fn value(&self) -> Option<TokenText> {
        match &self {
            Self::JsStringLiteralExpression(node) => node.inner_string_text().ok(),
            Self::JsxString(node) => node.inner_string_text().ok(),
            Self::JsTemplateChunkElement(template_chunk) => {
                Some(template_chunk.template_chunk_token().ok()?.token_text())
            }
            Self::JsLiteralMemberName(node) => node.name().ok(),
        }
    }
}
