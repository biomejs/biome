//! Helpers for rules that check Biome JS plugins, which use the plugin API exported by
//! `@biomejs/runtime/plugin`.
//!
//! A plugin names syntax kinds with strings, such as `"JS_CALL_EXPRESSION"` or `"IDENT"`.
//! `nodeKinds()` accepts only node kinds and `tokenKinds()` only token kinds. The names
//! alone don't say which is which (`JS_NUMBER_LITERAL` is a token kind), so the accepted
//! names are generated from the grammar into [`kind_names`].

mod kind_names;

use biome_console::fmt::{Display, Formatter};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, JsVariableDeclarator};
use biome_rowan::AstNode;
use kind_names::{NODE_KIND_NAMES, TOKEN_KIND_NAMES};

use super::imported_member_name;

/// The module that Biome JS plugins import the plugin API from.
const PLUGIN_API_MODULE: &str = "@biomejs/runtime/plugin";

/// Whether a syntax kind names a node or a token in the plugin API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KindCategory {
    Node,
    Token,
}

impl KindCategory {
    /// Returns the category of the kind named `name`, or `None` if neither `nodeKinds()`
    /// nor `tokenKinds()` accepts `name`.
    pub(crate) fn from_kind_name(name: &str) -> Option<Self> {
        if NODE_KIND_NAMES.binary_search(&name).is_ok() {
            Some(Self::Node)
        } else if TOKEN_KIND_NAMES.binary_search(&name).is_ok() {
            Some(Self::Token)
        } else {
            None
        }
    }

    /// Returns the category of the set that `callee` creates when it refers to `nodeKinds`
    /// or `tokenKinds` imported from the plugin API, including renamed imports and members
    /// of a namespace import.
    pub(crate) fn from_set_constructor(
        callee: &AnyJsExpression,
        model: &SemanticModel,
    ) -> Option<Self> {
        let name =
            imported_member_name(&callee.clone().omit_parentheses(), model, PLUGIN_API_MODULE)?;
        match name.text_trimmed() {
            "nodeKinds" => Some(Self::Node),
            "tokenKinds" => Some(Self::Token),
            _ => None,
        }
    }
}

/// Writes the name of the plugin API function that creates a set of this category.
impl Display for KindCategory {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Node => "nodeKinds",
            Self::Token => "tokenKinds",
        })
    }
}

/// Returns the value of `expression`, following `const` variables to the expression they're
/// initialized with. Parentheses and TypeScript-only syntax such as `as const` are ignored.
///
/// Returns `None` when `expression` reads a variable that isn't a `const` variable with an
/// initializer, such as a parameter, a `let` variable, or a destructured `const` variable.
pub(crate) fn resolve_constant(
    expression: AnyJsExpression,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let mut expression = expression.omit_type_wrappers()?;
    // The limit stops cycles such as `const a = b, b = a;`, which parse without errors.
    for _ in 0..MAX_CONSTANT_STEPS {
        let AnyJsExpression::JsIdentifierExpression(identifier) = &expression else {
            return Some(expression);
        };
        let binding = model.binding(&identifier.name().ok()?)?;
        let declarator = JsVariableDeclarator::cast(binding.syntax().parent()?)?;
        if !declarator.declaration()?.is_const() {
            return None;
        }
        expression = declarator
            .initializer()?
            .expression()
            .ok()?
            .omit_type_wrappers()?;
    }
    None
}

/// The number of `const` variables that [`resolve_constant`] follows before giving up.
const MAX_CONSTANT_STEPS: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sorted(names: &[&str]) {
        for pair in names.windows(2) {
            assert!(
                pair[0] < pair[1],
                "`{}` must come before `{}`, and names must be unique",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn kind_names_are_sorted() {
        assert_sorted(NODE_KIND_NAMES);
        assert_sorted(TOKEN_KIND_NAMES);
    }

    #[test]
    fn kind_categories() {
        assert_eq!(
            KindCategory::from_kind_name("JS_CALL_EXPRESSION"),
            Some(KindCategory::Node)
        );
        assert_eq!(
            KindCategory::from_kind_name("JS_MODULE_ITEM_LIST"),
            Some(KindCategory::Node)
        );
        assert_eq!(
            KindCategory::from_kind_name("JS_BOGUS"),
            Some(KindCategory::Node)
        );
        assert_eq!(
            KindCategory::from_kind_name("JS_NUMBER_LITERAL"),
            Some(KindCategory::Token)
        );
        assert_eq!(
            KindCategory::from_kind_name("EOF"),
            Some(KindCategory::Token)
        );
        assert_eq!(KindCategory::from_kind_name("typescript"), None);
    }
}
