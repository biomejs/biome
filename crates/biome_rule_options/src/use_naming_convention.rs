use crate::shared::restricted_regex::RestrictedRegex;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use biome_string_case::{Case, Cases};
use enumflags2::BitFlags;
use smallvec::SmallVec;
use std::ops::Deref;

/// Rule's options.
#[derive(Debug, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseNamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
    /// This does not affect other [Case].
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub strict_case: bool,

    /// If `false`, then non-ASCII characters are allowed.
    #[serde(default = "enabled", skip_serializing_if = "bool::clone")]
    pub require_ascii: bool,

    /// Custom conventions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub conventions: Box<[Convention]>,
}
impl Default for UseNamingConventionOptions {
    fn default() -> Self {
        Self {
            strict_case: true,
            require_ascii: true,
            conventions: Vec::new().into_boxed_slice(),
        }
    }
}

const fn enabled() -> bool {
    true
}
fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
#[deserializable(with_validator)]
pub struct Convention {
    /// Declarations concerned by this convention
    #[serde(default, skip_serializing_if = "is_default")]
    pub selector: Selector,

    /// Regular expression to enforce
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub matching: Option<RestrictedRegex>,

    /// String cases to enforce
    #[serde(default, skip_serializing_if = "is_default")]
    pub formats: Formats,
}

impl DeserializableValidator for Convention {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.formats.is_empty() && self.matching.is_none() {
            ctx.report(
                DeserializationDiagnostic::new(
                    "At least one field among `formats` and `match` must be set.",
                )
                .with_range(range),
            );
            false
        } else {
            true
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deserializable(with_validator)]
#[serde(deny_unknown_fields)]
pub struct Selector {
    /// Declaration kind
    #[serde(default, skip_serializing_if = "is_default")]
    pub kind: Kind,

    /// Modifiers used on the declaration
    #[serde(default, skip_serializing_if = "is_default")]
    pub modifiers: RestrictedModifiers,

    /// Scope of the declaration
    #[serde(default, skip_serializing_if = "is_default")]
    pub scope: Scope,
}

#[derive(Copy, Clone, Debug)]
pub enum InvalidSelector {
    IncompatibleModifiers(RestrictedModifier, RestrictedModifier),
    UnsupportedModifiers(Kind, RestrictedModifier),
    UnsupportedScope(Kind, Scope),
}
impl std::error::Error for InvalidSelector {}
impl std::fmt::Display for InvalidSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleModifiers(modifier1, modifier2) => {
                write!(
                    f,
                    "The `{modifier1}` and `{modifier2}` modifiers cannot be used together.",
                )
            }
            Self::UnsupportedModifiers(kind, modifier) => {
                write!(
                    f,
                    "The `{modifier}` modifier cannot be used with the `{kind}` kind."
                )
            }
            Self::UnsupportedScope(kind, scope) => {
                let scope = scope.to_string();
                let scope = scope.trim_end();
                write!(
                    f,
                    "The `{scope}` scope cannot be used with the `{kind}` kind."
                )
            }
        }
    }
}

impl Selector {
    /// Returns an error if the current selector is not valid.
    pub fn check(self) -> Result<(), InvalidSelector> {
        if self.modifiers.contains(RestrictedModifier::Abstract) {
            if self.kind != Kind::Class && !Kind::ClassMember.contains(self.kind) {
                return Err(InvalidSelector::UnsupportedModifiers(
                    self.kind,
                    RestrictedModifier::Abstract,
                ));
            }
            if self.modifiers.contains(RestrictedModifier::Static) {
                return Err(InvalidSelector::IncompatibleModifiers(
                    RestrictedModifier::Abstract,
                    RestrictedModifier::Static,
                ));
            }
        }
        if self.modifiers.contains(RestrictedModifier::Readonly)
            && !matches!(
                self.kind,
                Kind::ClassProperty | Kind::IndexParameter | Kind::TypeProperty
            )
        {
            return Err(InvalidSelector::UnsupportedModifiers(
                self.kind,
                RestrictedModifier::Readonly,
            ));
        }
        if self
            .modifiers
            .intersects(RestrictedModifier::CLASS_MEMBER_ONLY)
            && !Kind::ClassMember.contains(self.kind)
        {
            let modifiers = self.modifiers.0 & RestrictedModifier::CLASS_MEMBER_ONLY;
            if let Some(modifier) = modifiers.iter().next() {
                return Err(InvalidSelector::UnsupportedModifiers(self.kind, modifier));
            }
        }
        // The rule doesn't allow `Modifier::Public`.
        // So we only need to check for `Modifier::Private`/`Modifier::Protected` incompatibility.
        let accessibility = RestrictedModifier::Private | RestrictedModifier::Protected;
        if *self.modifiers & accessibility == accessibility {
            return Err(InvalidSelector::IncompatibleModifiers(
                RestrictedModifier::Private,
                RestrictedModifier::Protected,
            ));
        }
        let abstarct_or_static = RestrictedModifier::Abstract | RestrictedModifier::Static;
        if *self.modifiers & abstarct_or_static == abstarct_or_static {
            return Err(InvalidSelector::IncompatibleModifiers(
                RestrictedModifier::Abstract,
                RestrictedModifier::Static,
            ));
        }
        if self.scope == Scope::Global
            && !Kind::Variable.contains(self.kind)
            && !Kind::Function.contains(self.kind)
            && !Kind::TypeLike.contains(self.kind)
        {
            return Err(InvalidSelector::UnsupportedScope(self.kind, Scope::Global));
        }
        Ok(())
    }

    pub fn with_modifiers(kind: Kind, modifiers: impl Into<RestrictedModifiers>) -> Self {
        Self {
            kind,
            modifiers: modifiers.into(),
            ..Default::default()
        }
    }

    pub fn with_scope(kind: Kind, scope: Scope) -> Self {
        Self {
            kind,
            scope,
            ..Default::default()
        }
    }

    pub fn contains(&self, other: Self) -> bool {
        other.kind.contains(self.kind)
            && self.modifiers.contains(other.modifiers.0)
            && other.scope.contains(self.scope)
    }
}

impl DeserializableValidator for Selector {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if let Err(error) = self.check() {
            ctx.report(DeserializationDiagnostic::new(format_args!("{error}")).with_range(range));
            return false;
        }
        true
    }
}

impl From<Kind> for Selector {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            modifiers: RestrictedModifiers::default(),
            scope: Scope::Any,
        }
    }
}
impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.scope, self.modifiers, self.kind)
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Any => "",
            Self::Global => "global ",
        };
        write!(f, "{repr}")
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    /// All kinds
    #[default]
    Any,
    /// All type definitions: classes, enums, interfaces, and type aliases
    TypeLike,
    Class,
    Enum,
    /// Named function declarations and expressions
    Function,
    Interface,
    EnumMember,
    /// TypeScript namespaces, import and export namespaces
    NamespaceLike,
    /// TypeScript mamespaces
    Namespace,
    ImportNamespace,
    ExportNamespace,
    // All variable declaration: const, let, using, var
    Variable,
    Const,
    Let,
    Using,
    Var,
    /// All function parameters, but parameter properties
    FunctionParameter,
    CatchParameter,
    IndexParameter,
    /// All generic type parameters
    TypeParameter,
    // All re-export default exports and aliases of re-exported names
    ExportAlias,
    // All default imports and aliases of named imports
    ImportAlias,
    /// All class members: properties, methods, getters, and setters
    ClassMember,
    /// All class properties, including parameter properties
    ClassProperty,
    ClassGetter,
    ClassSetter,
    ClassMethod,
    /// All object literal members: properties, methods, getters, and setters
    ObjectLiteralMember,
    ObjectLiteralProperty,
    ObjectLiteralGetter,
    ObjectLiteralSetter,
    ObjectLiteralMethod,
    TypeAlias,
    /// All members defined in type alaises and interfaces
    TypeMember,
    /// All getters defined in type alaises and interfaces
    TypeGetter,
    /// All properties defined in type alaises and interfaces
    TypeProperty,
    /// All setters defined in type alaises and interfaces
    TypeSetter,
    /// All methods defined in type alaises and interfaces
    TypeMethod,
}

impl Kind {
    pub fn contains(self, other: Self) -> bool {
        self == other
            || matches!(
                (self, other),
                (Self::Any, _)
                    | (
                        Self::Variable,
                        Self::Const | Self::Let | Self::Using | Self::Var,
                    )
                    | (
                        Self::ClassMember,
                        Self::ClassGetter
                            | Self::ClassMethod
                            | Self::ClassProperty
                            | Self::ClassSetter
                    )
                    | (
                        Self::ObjectLiteralMember,
                        Self::ObjectLiteralGetter
                            | Self::ObjectLiteralMethod
                            | Self::ObjectLiteralProperty
                            | Self::ObjectLiteralSetter
                    )
                    | (
                        Self::TypeMember,
                        Self::TypeGetter
                            | Self::TypeMethod
                            | Self::TypeParameter
                            | Self::TypeProperty
                            | Self::TypeSetter
                    )
                    | (
                        Self::NamespaceLike,
                        Self::ExportNamespace | Self::ImportNamespace | Self::Namespace
                    )
                    | (
                        Self::TypeLike,
                        Self::Class
                            | Self::Enum
                            | Self::EnumMember
                            | Self::Interface
                            | Self::TypeAlias
                            | Self::TypeParameter
                    )
            )
    }
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Any => "declaration",
            Self::CatchParameter => "catch parameter",
            Self::Class => "class",
            Self::ClassGetter => "class getter",
            Self::ClassMember => "class member",
            Self::ClassMethod => "class method",
            Self::ClassProperty => "class property",
            Self::ClassSetter => "class setter",
            Self::Const => "const",
            Self::Enum => "enum",
            Self::EnumMember => "enum member",
            Self::ExportAlias => "export alias",
            Self::ExportNamespace => "export namespace",
            Self::Function => "function",
            Self::ImportAlias => "import alias",
            Self::ImportNamespace => "import namespace",
            Self::IndexParameter => "index parameter",
            Self::Interface => "interface",
            Self::Let => "let",
            Self::Namespace => "namespace",
            Self::NamespaceLike => "namespace",
            Self::ObjectLiteralGetter => "object getter",
            Self::ObjectLiteralMember => "object member",
            Self::ObjectLiteralMethod => "object method",
            Self::ObjectLiteralProperty => "object property",
            Self::ObjectLiteralSetter => "object setter",
            Self::FunctionParameter => "function parameter",
            Self::TypeAlias => "type alias",
            Self::TypeGetter => "getter",
            Self::TypeLike => "type",
            Self::TypeMember => "type member",
            Self::TypeMethod => "method",
            Self::TypeParameter => "type parameter",
            Self::TypeProperty => "property",
            Self::TypeSetter => "setter",
            Self::Using => "using",
            Self::Var => "var",
            Self::Variable => "variable",
        };
        write!(f, "{repr}")
    }
}

#[derive(
    Debug,
    Copy,
    Default,
    Deserializable,
    Clone,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(
    from = "SmallVec<[RestrictedModifier; 4]>",
    into = "SmallVec<[RestrictedModifier; 4]>"
)]
pub struct RestrictedModifiers(BitFlags<RestrictedModifier>);

impl Deref for RestrictedModifiers {
    type Target = BitFlags<RestrictedModifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<RestrictedModifier> for RestrictedModifiers {
    fn from(value: RestrictedModifier) -> Self {
        RestrictedModifiers(value.into())
    }
}
impl From<RestrictedModifiers> for SmallVec<[RestrictedModifier; 4]> {
    fn from(value: RestrictedModifiers) -> Self {
        value.into_iter().collect()
    }
}
impl From<SmallVec<[RestrictedModifier; 4]>> for RestrictedModifiers {
    fn from(values: SmallVec<[RestrictedModifier; 4]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<RestrictedModifier> for RestrictedModifiers {
    fn from_iter<T: IntoIterator<Item = RestrictedModifier>>(values: T) -> Self {
        Self(values.into_iter().fold(BitFlags::empty(), |acc, m| acc | m))
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for RestrictedModifiers {
    fn schema_name() -> String {
        "Modifiers".to_string()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<RestrictedModifier>>::json_schema(generator)
    }
}

impl std::fmt::Display for RestrictedModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.0.iter() {
            write!(f, "{value} ")?;
        }
        Ok(())
    }
}

#[derive(
    Debug,
    Copy,
    Default,
    Deserializable,
    Clone,
    Hash,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    #[default]
    Any,
    Global,
}

impl Scope {
    pub fn contains(self, scope: Self) -> bool {
        matches!(self, Self::Any) || self == scope
    }
}

/// Supported cases.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Format {
    #[serde(rename = "camelCase")]
    Camel,

    #[serde(rename = "CONSTANT_CASE")]
    Constant,

    #[serde(rename = "PascalCase")]
    #[default]
    Pascal,

    #[serde(rename = "snake_case")]
    Snake,
}

impl From<Format> for Case {
    fn from(value: Format) -> Self {
        match value {
            Format::Camel => Case::Camel,
            Format::Constant => Case::Constant,
            Format::Pascal => Case::Pascal,
            Format::Snake => Case::Snake,
        }
    }
}
impl TryFrom<Case> for Format {
    type Error = &'static str;

    fn try_from(value: Case) -> Result<Self, Self::Error> {
        match value {
            Case::Camel => Ok(Format::Camel),
            Case::Constant => Ok(Format::Constant),
            Case::Pascal => Ok(Format::Pascal),
            Case::Snake => Ok(Format::Snake),
            Case::Kebab
            | Case::Lower
            | Case::Number
            | Case::NumberableCapital
            | Case::Uni
            | Case::Upper
            | Case::Unknown => Err("Unsupported case"),
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserializable,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(from = "SmallVec<[Format; 4]>", into = "SmallVec<[Format; 4]>")]
pub struct Formats(Cases);

impl Deref for Formats {
    type Target = Cases;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<Cases> for Formats {
    fn from(value: Cases) -> Self {
        Self(value)
    }
}
impl From<SmallVec<[Format; 4]>> for Formats {
    fn from(values: SmallVec<[Format; 4]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<Format> for Formats {
    fn from_iter<T: IntoIterator<Item = Format>>(values: T) -> Self {
        Self(values.into_iter().map(|format| format.into()).collect())
    }
}
impl From<Formats> for SmallVec<[Format; 4]> {
    fn from(value: Formats) -> Self {
        value
            .0
            .into_iter()
            .filter_map(|case| case.try_into().ok())
            .collect()
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for Formats {
    fn schema_name() -> String {
        "Formats".to_string()
    }
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        <std::collections::HashSet<Format>>::json_schema(generator)
    }
}

#[derive(Debug, Deserializable, Copy, Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[enumflags2::bitflags]
#[repr(u8)]
pub enum RestrictedModifier {
    Abstract = 1 << 0,
    Private = 1 << 1,
    Protected = 1 << 2,
    Readonly = 1 << 3,
    Static = 1 << 4,
}
impl RestrictedModifier {
    pub const ACCESSIBILITY: BitFlags<Self> = BitFlags::<Self>::from_bits_truncate_c(
        Self::Private as u8 | Self::Protected as u8,
        BitFlags::CONST_TOKEN,
    );
    pub const CLASS_MEMBER_ONLY: BitFlags<Self> = Self::ACCESSIBILITY.union_c(
        BitFlags::<Self>::from_bits_truncate_c(Self::Static as u8, BitFlags::CONST_TOKEN),
    );
    pub const CLASS_TYPE_PROPERTY: BitFlags<Self> =
        BitFlags::<Self>::from_bits_truncate_c(Self::Readonly as u8, BitFlags::CONST_TOKEN);
}
impl std::fmt::Display for RestrictedModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Abstract => "abstract",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Readonly => "readonly",
            Self::Static => "static",
        })
    }
}
