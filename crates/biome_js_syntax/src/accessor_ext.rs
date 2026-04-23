use crate::{JsGetterClassMember, JsGetterObjectMember, JsSetterClassMember, JsSetterObjectMember};
use biome_rowan::{AstNode, AstNodeList, TokenText, declare_node_union};

declare_node_union! {
    pub AnyJsGetter = JsGetterClassMember | JsGetterObjectMember
}

impl AnyJsGetter {
    /// Whether the getter is declared `static`.
    pub fn is_static(&self) -> bool {
        match self {
            Self::JsGetterClassMember(getter) => getter
                .modifiers()
                .iter()
                .any(|m| m.as_js_static_modifier().is_some()),
            Self::JsGetterObjectMember(_) => false,
        }
    }

    /// Returns the canonical name of the getter's literal member name (quotes stripped).
    pub fn member_name(&self) -> Option<TokenText> {
        let literal_name = match self {
            Self::JsGetterClassMember(m) => m.name().ok()?.as_js_literal_member_name().cloned()?,
            Self::JsGetterObjectMember(m) => m.name().ok()?.as_js_literal_member_name().cloned()?,
        };
        literal_name.name().ok()
    }

    /// Iterator over setter siblings declared in the same class or object
    /// member list as this getter.
    pub fn sibling_setters(&self) -> impl Iterator<Item = AnyJsSetter> {
        self.syntax()
            .parent()
            .into_iter()
            .flat_map(|parent| parent.children())
            .filter_map(AnyJsSetter::cast)
    }

    /// Returns true if the getter has a sibling setter with the given name and matching `static` modifier.
    pub fn has_matching_setter(&self, name: &TokenText) -> bool {
        let getter_is_static = self.is_static();
        self.sibling_setters().any(|setter| {
            setter.is_static() == getter_is_static
                && setter
                    .member_name()
                    .is_some_and(|setter_name| setter_name == *name)
        })
    }
}

declare_node_union! {
    pub AnyJsSetter = JsSetterClassMember | JsSetterObjectMember
}

impl AnyJsSetter {
    /// Whether the setter is declared `static`.
    pub fn is_static(&self) -> bool {
        match self {
            Self::JsSetterClassMember(setter) => setter
                .modifiers()
                .iter()
                .any(|m| m.as_js_static_modifier().is_some()),
            Self::JsSetterObjectMember(_) => false,
        }
    }

    /// Returns the canonical name of the setter's literal member name (quotes stripped).
    pub fn member_name(&self) -> Option<TokenText> {
        let literal_name = match self {
            Self::JsSetterClassMember(m) => m.name().ok()?.as_js_literal_member_name().cloned()?,
            Self::JsSetterObjectMember(m) => m.name().ok()?.as_js_literal_member_name().cloned()?,
        };
        literal_name.name().ok()
    }
}
