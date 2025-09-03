use std::ops::Deref;

use biome_analyze::QueryMatch;

use super::*;

impl AnyVueComponent {
    /// Try to infer the component's name from its definition.
    pub fn component_name(&self) -> Option<(TokenText, TextRange)> {
        let object_expression = match self {
            Self::OptionsApi(c) => c
                .definition_expression()
                .and_then(|e| e.inner_expression())
                .and_then(|e| e.as_js_object_expression().cloned()),
            Self::CreateApp(c) => c
                .definition_expression()
                .and_then(|e| e.inner_expression())
                .and_then(|e| e.as_js_object_expression().cloned()),
            Self::DefineComponent(c) => c
                .definition_expression()
                .and_then(|e| e.inner_expression())
                .and_then(|e| e.as_js_object_expression().cloned()),
            // <script setup> components are named by the file name, so we can't infer it here.
            Self::Setup(_) => None,
        }?;

        // Find `name` property
        for member in object_expression.members().into_iter().flatten() {
            if let AnyJsObjectMember::JsPropertyObjectMember(property) = member {
                if property
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_none_or(|n| n != "name")
                {
                    continue;
                };

                if let Ok(value_expr) = property.value() {
                    let value_expr = value_expr.omit_parentheses();
                    if let Some(str_lit) = value_expr
                        .as_any_js_literal_expression()
                        .and_then(|e| e.as_js_string_literal_expression())
                        && let Ok(token_text) = str_lit.inner_string_text()
                    {
                        return Some((token_text, str_lit.syntax().text_range()));
                    }
                }
            }
        }
        None
    }
}

/// A Vue component name, either extracted from the component definition or inferred from the file path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VueComponentName<'a> {
    FromComponent((TokenText, TextRange)),
    FromPath(&'a str),
}

impl PartialEq<str> for VueComponentName<'_> {
    fn eq(&self, other: &str) -> bool {
        match self {
            VueComponentName::FromComponent((name, _)) => *name == other,
            VueComponentName::FromPath(name) => *name == other,
        }
    }
}

impl PartialOrd<str> for VueComponentName<'_> {
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        match self {
            VueComponentName::FromComponent((name, _)) => name.text().partial_cmp(other),
            VueComponentName::FromPath(name) => (*name).partial_cmp(other),
        }
    }
}

impl Deref for VueComponentName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            VueComponentName::FromComponent((name, _)) => name.text(),
            VueComponentName::FromPath(name) => name,
        }
    }
}

impl AsRef<str> for VueComponentName<'_> {
    fn as_ref(&self) -> &str {
        match self {
            VueComponentName::FromComponent((name, _)) => name.text(),
            VueComponentName::FromPath(name) => name,
        }
    }
}
