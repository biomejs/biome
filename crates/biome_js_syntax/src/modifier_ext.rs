use crate::{
    AnyJsMethodModifier, AnyJsPropertyModifier, AnyTsIndexSignatureModifier,
    AnyTsMethodSignatureModifier, AnyTsPropertyParameterModifier, AnyTsPropertySignatureModifier,
    AnyTsTypeParameterModifier, JsSyntaxKind, TsAccessibilityModifier,
};

/// Helpful data structure to make the order of modifiers predictable inside the formatter
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Modifiers {
    // modifiers must be sorted by precedence.
    Decorator,
    Accessibility,
    Declare,
    Static,
    Abstract,
    Override,
    Readonly,
    Accessor,
}

impl From<&AnyTsIndexSignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsIndexSignatureModifier) -> Self {
        match modifier {
            AnyTsIndexSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsIndexSignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyJsMethodModifier> for Modifiers {
    fn from(modifier: &AnyJsMethodModifier) -> Self {
        match modifier {
            AnyJsMethodModifier::JsDecorator(_) => Modifiers::Decorator,
            AnyJsMethodModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyJsMethodModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyJsMethodModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&AnyTsMethodSignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsMethodSignatureModifier) -> Self {
        match modifier {
            AnyTsMethodSignatureModifier::JsDecorator(_) => Modifiers::Decorator,
            AnyTsMethodSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsMethodSignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            AnyTsMethodSignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsMethodSignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&AnyJsPropertyModifier> for Modifiers {
    fn from(modifier: &AnyJsPropertyModifier) -> Self {
        match modifier {
            AnyJsPropertyModifier::JsDecorator(_) => Modifiers::Decorator,
            AnyJsPropertyModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyJsPropertyModifier::JsAccessorModifier(_) => Modifiers::Accessor,
            AnyJsPropertyModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyJsPropertyModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyJsPropertyModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyTsPropertyParameterModifier> for Modifiers {
    fn from(modifier: &AnyTsPropertyParameterModifier) -> Self {
        match modifier {
            AnyTsPropertyParameterModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsPropertyParameterModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyTsPropertyParameterModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyTsPropertySignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsPropertySignatureModifier) -> Self {
        match modifier {
            AnyTsPropertySignatureModifier::JsDecorator(_) => Modifiers::Decorator,
            AnyTsPropertySignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsPropertySignatureModifier::TsDeclareModifier(_) => Modifiers::Declare,
            AnyTsPropertySignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsPropertySignatureModifier::JsAccessorModifier(_) => Modifiers::Accessor,
            AnyTsPropertySignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            AnyTsPropertySignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyTsPropertySignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

/// Helpful data structure to make the order of type parameter modifiers predictable inside the formatter
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeParameterModifiers {
    Const,
    In,
    Out,
}

impl From<&AnyTsTypeParameterModifier> for TypeParameterModifiers {
    fn from(modifier: &AnyTsTypeParameterModifier) -> Self {
        match modifier {
            AnyTsTypeParameterModifier::TsConstModifier(_) => Self::Const,
            AnyTsTypeParameterModifier::TsInModifier(_) => Self::In,
            AnyTsTypeParameterModifier::TsOutModifier(_) => Self::Out,
        }
    }
}

impl TsAccessibilityModifier {
    /// Is `self` the `private` accessibility modifier?
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let modifier = make::ts_accessibility_modifier(make::token(T![private]));
    ///
    /// assert!(modifier.is_private());
    /// ```
    pub fn is_private(&self) -> bool {
        if let Ok(modifier_token) = self.modifier_token() {
            modifier_token.kind() == JsSyntaxKind::PRIVATE_KW
        } else {
            false
        }
    }

    /// Is `self` the `protected` accessibility modifier?
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let modifier = make::ts_accessibility_modifier(make::token(T![protected]));
    ///
    /// assert!(modifier.is_protected());
    /// ```
    pub fn is_protected(&self) -> bool {
        if let Ok(modifier_token) = self.modifier_token() {
            modifier_token.kind() == JsSyntaxKind::PROTECTED_KW
        } else {
            false
        }
    }

    /// Is `self` the `public` accessibility modifier?
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let modifier = make::ts_accessibility_modifier(make::token(T![public]));
    ///
    /// assert!(modifier.is_public());
    /// ```
    pub fn is_public(&self) -> bool {
        if let Ok(modifier_token) = self.modifier_token() {
            modifier_token.kind() == JsSyntaxKind::PUBLIC_KW
        } else {
            false
        }
    }
}
