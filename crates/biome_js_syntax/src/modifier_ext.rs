use enumflags2::BitFlags;

use crate::{
    AnyJsMethodModifier, AnyJsPropertyModifier, AnyTsIndexSignatureModifier,
    AnyTsMethodSignatureModifier, AnyTsPropertyParameterModifier, AnyTsPropertySignatureModifier,
    AnyTsTypeParameterModifier, JsMethodModifierList, JsPropertyModifierList, JsSyntaxKind,
    TsAccessibilityModifier, TsIndexSignatureModifierList, TsMethodSignatureModifierList,
    TsPropertySignatureModifierList,
};

/// Helpful data structure to make the order of modifiers predictable inside the formatter
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[enumflags2::bitflags]
#[repr(u16)]
pub enum Modifier {
    // modifiers must be sorted by precedence.
    Decorator = 1 << 0,
    BogusAccessibility = 1 << 1,
    Private = 1 << 2,
    Protected = 1 << 3,
    Public = 1 << 4,
    Declare = 1 << 5,
    Static = 1 << 6,
    Abstract = 1 << 7,
    Override = 1 << 8,
    Readonly = 1 << 9,
    Accessor = 1 << 10,
}

impl Modifier {
    pub const ACCESSIBILITY: BitFlags<Self> = BitFlags::<Self>::from_bits_truncate_c(
        Self::BogusAccessibility as u16
            | Self::Private as u16
            | Self::Protected as u16
            | Self::Public as u16,
        BitFlags::CONST_TOKEN,
    );
    pub const CLASS_MEMBER_ONLY: BitFlags<Self> =
        Self::ACCESSIBILITY.union_c(BitFlags::<Self>::from_bits_truncate_c(
            Self::Static as u16 | Self::Override as u16 | Self::Accessor as u16,
            BitFlags::CONST_TOKEN,
        ));
    pub const CLASS_TYPE_PROPERTY: BitFlags<Self> = BitFlags::<Self>::from_bits_truncate_c(
        Self::Readonly as u16 | Self::Accessor as u16,
        BitFlags::CONST_TOKEN,
    );
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modifier::Decorator => "decorator",
                Modifier::BogusAccessibility => "accessibility",
                Modifier::Private => "private",
                Modifier::Protected => "protected",
                Modifier::Public => "public",
                Modifier::Declare => "declare",
                Modifier::Static => "static",
                Modifier::Abstract => "abstract",
                Modifier::Override => "override",
                Modifier::Readonly => "readonly",
                Modifier::Accessor => "accessor",
            }
        )
    }
}

impl From<&AnyTsIndexSignatureModifier> for Modifier {
    fn from(modifier: &AnyTsIndexSignatureModifier) -> Self {
        match modifier {
            AnyTsIndexSignatureModifier::JsStaticModifier(_) => Modifier::Static,
            AnyTsIndexSignatureModifier::TsReadonlyModifier(_) => Modifier::Readonly,
        }
    }
}

impl From<&AnyJsMethodModifier> for Modifier {
    fn from(modifier: &AnyJsMethodModifier) -> Self {
        match modifier {
            AnyJsMethodModifier::JsDecorator(_) => Modifier::Decorator,
            AnyJsMethodModifier::JsStaticModifier(_) => Modifier::Static,
            AnyJsMethodModifier::TsAccessibilityModifier(accessibility) => accessibility.into(),
            AnyJsMethodModifier::TsOverrideModifier(_) => Modifier::Override,
        }
    }
}

impl From<&AnyTsMethodSignatureModifier> for Modifier {
    fn from(modifier: &AnyTsMethodSignatureModifier) -> Self {
        match modifier {
            AnyTsMethodSignatureModifier::JsDecorator(_) => Modifier::Decorator,
            AnyTsMethodSignatureModifier::JsStaticModifier(_) => Modifier::Static,
            AnyTsMethodSignatureModifier::TsAbstractModifier(_) => Modifier::Abstract,
            AnyTsMethodSignatureModifier::TsAccessibilityModifier(accessibility) => {
                accessibility.into()
            }
            AnyTsMethodSignatureModifier::TsOverrideModifier(_) => Modifier::Override,
        }
    }
}

impl From<&AnyJsPropertyModifier> for Modifier {
    fn from(modifier: &AnyJsPropertyModifier) -> Self {
        match modifier {
            AnyJsPropertyModifier::JsDecorator(_) => Modifier::Decorator,
            AnyJsPropertyModifier::JsStaticModifier(_) => Modifier::Static,
            AnyJsPropertyModifier::JsAccessorModifier(_) => Modifier::Accessor,
            AnyJsPropertyModifier::TsAccessibilityModifier(accessibility) => accessibility.into(),
            AnyJsPropertyModifier::TsOverrideModifier(_) => Modifier::Override,
            AnyJsPropertyModifier::TsReadonlyModifier(_) => Modifier::Readonly,
        }
    }
}

impl From<&AnyTsPropertyParameterModifier> for Modifier {
    fn from(modifier: &AnyTsPropertyParameterModifier) -> Self {
        match modifier {
            AnyTsPropertyParameterModifier::TsAccessibilityModifier(accessibility) => {
                accessibility.into()
            }
            AnyTsPropertyParameterModifier::TsOverrideModifier(_) => Modifier::Override,
            AnyTsPropertyParameterModifier::TsReadonlyModifier(_) => Modifier::Readonly,
        }
    }
}

impl From<&AnyTsPropertySignatureModifier> for Modifier {
    fn from(modifier: &AnyTsPropertySignatureModifier) -> Self {
        match modifier {
            AnyTsPropertySignatureModifier::JsDecorator(_) => Modifier::Decorator,
            AnyTsPropertySignatureModifier::TsAccessibilityModifier(accessibility) => {
                accessibility.into()
            }
            AnyTsPropertySignatureModifier::TsDeclareModifier(_) => Modifier::Declare,
            AnyTsPropertySignatureModifier::JsStaticModifier(_) => Modifier::Static,
            AnyTsPropertySignatureModifier::JsAccessorModifier(_) => Modifier::Accessor,
            AnyTsPropertySignatureModifier::TsAbstractModifier(_) => Modifier::Abstract,
            AnyTsPropertySignatureModifier::TsOverrideModifier(_) => Modifier::Override,
            AnyTsPropertySignatureModifier::TsReadonlyModifier(_) => Modifier::Readonly,
        }
    }
}

impl From<&TsAccessibilityModifier> for Modifier {
    fn from(value: &TsAccessibilityModifier) -> Self {
        if let Ok(modifier_token) = value.modifier_token() {
            match modifier_token.kind() {
                JsSyntaxKind::PRIVATE_KW => Self::Private,
                JsSyntaxKind::PROTECTED_KW => Self::Protected,
                JsSyntaxKind::PUBLIC_KW => Self::Public,
                _ => Self::BogusAccessibility,
            }
        } else {
            Self::BogusAccessibility
        }
    }
}

impl From<&JsMethodModifierList> for enumflags2::BitFlags<Modifier> {
    fn from(value: &JsMethodModifierList) -> Self {
        value
            .into_iter()
            .map(|m| Modifier::from(&m))
            .fold(Self::empty(), |acc, m| acc | m)
    }
}
impl From<&JsPropertyModifierList> for enumflags2::BitFlags<Modifier> {
    fn from(value: &JsPropertyModifierList) -> Self {
        value
            .into_iter()
            .map(|m| Modifier::from(&m))
            .fold(Self::empty(), |acc, m| acc | m)
    }
}
impl From<&TsIndexSignatureModifierList> for enumflags2::BitFlags<Modifier> {
    fn from(value: &TsIndexSignatureModifierList) -> Self {
        value
            .into_iter()
            .map(|m| Modifier::from(&m))
            .fold(Self::empty(), |acc, m| acc | m)
    }
}
impl From<&TsPropertySignatureModifierList> for enumflags2::BitFlags<Modifier> {
    fn from(value: &TsPropertySignatureModifierList) -> Self {
        value
            .into_iter()
            .map(|m| Modifier::from(&m))
            .fold(Self::empty(), |acc, m| acc | m)
    }
}
impl From<&TsMethodSignatureModifierList> for enumflags2::BitFlags<Modifier> {
    fn from(value: &TsMethodSignatureModifierList) -> Self {
        value
            .into_iter()
            .map(|m| Modifier::from(&m))
            .fold(Self::empty(), |acc, m| acc | m)
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
