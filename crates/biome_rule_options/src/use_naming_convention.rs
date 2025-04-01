use crate::shared::restricted_regex::RestrictedRegex;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

/// Rule's options.
#[derive(Debug, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schema::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamingConventionOptions {
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
impl Default for NamingConventionOptions {
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
    pub modifiers: Modifiers,

    /// Scope of the declaration
    #[serde(default, skip_serializing_if = "is_default")]
    pub scope: Scope,
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
            modifiers: Modifiers::default(),
            scope: Scope::Any,
        }
    }
}
impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.scope, self.modifiers, self.kind)
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
pub struct Modifiers(BitFlags<Modifier>);

impl Deref for Modifiers {
    type Target = BitFlags<Modifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<Modifier> for Modifiers {
    fn from(value: Modifier) -> Self {
        Modifiers(value.into())
    }
}
impl From<Modifiers> for SmallVec<[RestrictedModifier; 4]> {
    fn from(value: Modifiers) -> Self {
        value.into_iter().map(|modifier| modifier.into()).collect()
    }
}
impl From<SmallVec<[RestrictedModifier; 4]>> for Modifiers {
    fn from(values: SmallVec<[RestrictedModifier; 4]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<RestrictedModifier> for Modifiers {
    fn from_iter<T: IntoIterator<Item = RestrictedModifier>>(values: T) -> Self {
        Self(
            values
                .into_iter()
                .map(Modifier::from)
                .fold(BitFlags::empty(), |acc, m| acc | m),
        )
    }
}
#[cfg(feature = "schema")]
impl JsonSchema for Modifiers {
    fn schema_name() -> String {
        "Modifiers".to_string()
    }

    fn json_schema(generator: &mut schema::r#gen::SchemaGenerator) -> schema::schema::Schema {
        <std::collections::HashSet<RestrictedModifier>>::json_schema(generator)
    }
}
impl From<JsMethodModifierList> for Modifiers {
    fn from(value: JsMethodModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<JsPropertyModifierList> for Modifiers {
    fn from(value: JsPropertyModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsIndexSignatureModifierList> for Modifiers {
    fn from(value: TsIndexSignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsMethodSignatureModifierList> for Modifiers {
    fn from(value: TsMethodSignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl From<TsPropertySignatureModifierList> for Modifiers {
    fn from(value: TsPropertySignatureModifierList) -> Self {
        Modifiers((&value).into())
    }
}
impl std::fmt::Display for Modifiers {
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
    /// Returns the scope of `node` or `None` if the scope cannot be determined or
    /// if the scope is an external module.
    fn from_declaration(node: &AnyJsBindingDeclaration) -> Option<Scope> {
        let control_flow_root = node.syntax().ancestors().skip(1).find(|x| {
            AnyJsControlFlowRoot::can_cast(x.kind())
                || x.kind() == JsSyntaxKind::TS_DECLARATION_MODULE
        })?;
        match control_flow_root.kind() {
            JsSyntaxKind::JS_MODULE
            | JsSyntaxKind::JS_SCRIPT
            | JsSyntaxKind::TS_DECLARATION_MODULE
            | JsSyntaxKind::TS_MODULE_DECLARATION => Some(Scope::Global),
            // Ignore declarations in an external module declaration
            JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => None,
            _ => Some(Scope::Any),
        }
    }

    fn contains(self, scope: Scope) -> bool {
        matches!(self, Self::Any) || self == scope
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
impl JsonSchema for Formats {
    fn schema_name() -> String {
        "Formats".to_string()
    }
    fn json_schema(generator: &mut schema::r#gen::SchemaGenerator) -> schema::schema::Schema {
        <std::collections::HashSet<Format>>::json_schema(generator)
    }
}
