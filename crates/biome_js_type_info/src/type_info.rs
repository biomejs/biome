//! Data structures for tracking type information across JS/TS files and scopes.
//!
//! Note that while our type inference is based on TypeScript and aims to be
//! compatible with TypeScript's types, we apply the same inference to
//! JavaScript files as well.
//!
//! This can be used by type-informed lint rules such as `noFloatingPromises`.
//!
//! The type information is instantiated and updated inside the Workspace
//! Server.

pub mod literal;

use std::borrow::Cow;
use std::fmt::Debug;
use std::{ops::Deref, str::FromStr, sync::Arc};

use biome_js_type_info_macros::Resolvable;
use biome_resolver::ResolvedPath;
use biome_rowan::Text;

use crate::globals::{
    GLOBAL_ARRAY_ID, GLOBAL_NUMBER_ID, GLOBAL_PROMISE_ID, GLOBAL_STRING_ID, GLOBAL_UNKNOWN_ID,
};
use crate::type_info::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
use crate::{
    GLOBAL_RESOLVER, ModuleId, Resolvable, ResolvedTypeData, ResolvedTypeId, ResolvedTypeMember,
    ResolverId, TypeResolver,
};

const UNKNOWN: TypeData = TypeData::Reference(TypeReference::Resolved(GLOBAL_UNKNOWN_ID));

/// Type identifier referencing the type in a resolver's `types` vector.
///
/// Note that separate modules typically use separate resolvers. Because of
/// this, type IDs are only unique within a single module/resolver.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeId(u32);

impl TypeId {
    pub const fn new(index: usize) -> Self {
        // SAFETY: We don't handle files exceeding `u32::MAX` bytes.
        // So it isn't possible to exceed `u32::MAX` types.
        Self(index as u32)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone)]
pub struct Type {
    resolver: Arc<dyn TypeResolver>,
    id: ResolvedTypeId,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl Default for Type {
    fn default() -> Self {
        Self {
            resolver: GLOBAL_RESOLVER.clone(),
            id: GLOBAL_UNKNOWN_ID,
        }
    }
}

impl Deref for Type {
    type Target = TypeData;

    fn deref(&self) -> &Self::Target {
        self.resolved_data()
            .map_or(&UNKNOWN, |resolved| resolved.as_raw_data())
    }
}

impl Type {
    pub fn from_id(resolver: Arc<dyn TypeResolver>, id: ResolvedTypeId) -> Self {
        Self { resolver, id }
    }

    /// Returns this type's [`TypeId`].
    ///
    /// **Warning:** Type IDs can only be safely compared with other IDs from
    ///              the same module.
    pub fn id(&self) -> TypeId {
        self.id.id()
    }

    /// Returns `true` if this type represents a **union type** that has a
    /// variant for which the given `predicate` returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn has_variant(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::Union(union)) => union
                .types()
                .iter()
                .filter_map(|ty| self.resolve(ty))
                .any(predicate),
            _ => false,
        }
    }

    /// Returns whether if this type is an instance of a type matching the given
    /// `predicate`.
    pub fn is_array_of(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::InstanceOf(instance)) => {
                instance.ty == GLOBAL_ARRAY_ID.into()
                    && instance
                        .type_parameters
                        .first()
                        .and_then(|type_param| self.resolve(type_param))
                        .is_some_and(predicate)
            }
            _ => false,
        }
    }

    /// Returns whether `self` is a function with a return type matching the
    /// given `predicate`.
    pub fn is_function_with_return_type(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::Function(function)) => function
                .return_type
                .as_type()
                .and_then(|ty| self.resolve(ty))
                .is_some_and(predicate),
            _ => false,
        }
    }

    /// Returns whether if this type is an instance of a type matching the given
    /// `predicate`.
    pub fn is_instance_of(&self, predicate: impl Fn(Self) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::InstanceOf(instance)) => {
                self.resolve(&instance.ty).is_some_and(predicate)
            }
            _ => false,
        }
    }

    /// Returns whether this type is an interface that has a member matching the
    /// given `predicate`.
    pub fn is_interface_with_member(&self, predicate: impl Fn(ResolvedTypeMember) -> bool) -> bool {
        match self.as_raw_data() {
            Some(TypeData::Interface(interface)) => interface
                .members
                .iter()
                .map(|member| ResolvedTypeMember::from((self.id.resolver_id(), member)))
                .any(predicate),
            _ => false,
        }
    }

    /// Returns whether this type is a number or a literal number.
    pub fn is_number_or_number_literal(&self) -> bool {
        self.id == GLOBAL_NUMBER_ID
            || self.as_raw_data().is_some_and(|ty| match ty {
                TypeData::Number => true,
                TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::Number(_)),
                _ => false,
            })
    }

    /// Returns whether this type is the `Promise` class.
    pub fn is_promise(&self) -> bool {
        self.id == GLOBAL_PROMISE_ID
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(&self) -> bool {
        self.resolved_data()
            .is_some_and(|ty| ty.is_instance_of(self.resolver.as_ref(), GLOBAL_PROMISE_ID))
    }

    /// Returns whether this type is a string or a literal string.
    pub fn is_string_or_string_literal(&self) -> bool {
        self.id == GLOBAL_STRING_ID
            || self.as_raw_data().is_some_and(|ty| match ty {
                TypeData::String => true,
                TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::String(_)),
                _ => false,
            })
    }

    /// Returns whether this type is a string with the given `value`.
    pub fn is_string_literal(&self, value: &str) -> bool {
        self.as_raw_data().is_some_and(|ty| match ty {
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::String(literal) => literal.as_str() == value,
                _ => false,
            },
            _ => false,
        })
    }

    pub fn resolve(&self, ty: &TypeReference) -> Option<Self> {
        self.resolver
            .resolve_reference(&self.id.apply_module_id_to_reference(ty))
            .map(|resolved_id| self.with_resolved_id(resolved_id))
    }

    #[inline]
    pub(super) fn as_raw_data(&self) -> Option<&TypeData> {
        self.resolved_data().map(ResolvedTypeData::as_raw_data)
    }

    #[inline]
    pub fn resolved_data(&self) -> Option<ResolvedTypeData> {
        self.resolver.get_by_resolved_id(self.id)
    }

    fn with_resolved_id(&self, id: ResolvedTypeId) -> Self {
        let mut id = id;
        loop {
            let Some(resolved_data) = self.resolver.get_by_resolved_id(id) else {
                break;
            };
            match resolved_data.as_raw_data() {
                TypeData::Reference(TypeReference::Resolved(resolved_id)) => {
                    id = resolved_data.apply_module_id(*resolved_id);
                }
                _ => break,
            }
        }

        Self {
            resolver: self.resolver.clone(),
            id,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum TypeData {
    /// The type is unknown because inference couldn't determine a type.
    ///
    /// This is different from `UnknownKeyword`, because an explicit `unknown`
    /// should not be counted as a failure of the inference.
    Unknown,

    /// Special type referencing the global scope, which can be explicitly
    /// referenced using `window` or `globalThis`.
    Global,

    // Primitives
    BigInt,
    Boolean,
    Null,
    Number,
    String,
    Symbol,
    Undefined,

    /// Special type that is used for indicating an expression or return value
    /// is interpreted as a conditional, where the condition is decided upon the
    /// truthiness of the value.
    ///
    /// An example is the return value of the callback to `Array#filter()`.
    Conditional,

    /// Special type used to represent a module for which an ad-hoc namespace is
    /// created through `import * as namespace` syntax.
    ImportNamespace(ModuleId),

    // Complex types
    Class(Box<Class>),
    Constructor(Box<Constructor>),
    Function(Box<Function>),
    Interface(Box<Interface>),
    Module(Box<Module>),
    Namespace(Box<Namespace>),
    Object(Box<Object>),
    Tuple(Box<Tuple>),

    // Definition of a generic type argument.
    Generic(Box<GenericTypeParameter>),

    // Compound types
    Intersection(Box<Intersection>),
    Union(Box<Union>),

    /// Type derived from another through a built-in operator.
    TypeOperator(Box<TypeOperatorType>),

    /// Literal value used as a type.
    Literal(Box<Literal>),

    /// Instance of another type.
    InstanceOf(Box<TypeInstance>),

    /// Reference to another type.
    Reference(TypeReference),

    /// This one is nasty: TypeScript allows types, namespaces and values to
    /// exist with the same name within the same scope. This results in _merged_
    /// references that can reference each simultaneously. Merged references can
    /// be tracked across modules, because a single imported symbol can import
    /// all merged references under a single name.
    ///
    /// See also:
    /// https://www.typescriptlang.org/docs/handbook/declaration-merging.html
    MergedReference(Box<MergedReference>),

    /// Reference to the type of a JavaScript expression.
    TypeofExpression(Box<TypeofExpression>),

    /// Reference to another type through the `typeof` operator.
    TypeofType(Box<TypeReference>),

    /// Reference to the type of a named JavaScript value.
    TypeofValue(Box<TypeofValue>),

    /// The `any` keyword.
    ///
    /// This variant may also be used if the `any` keyword is implied.
    /// For instance, in `catch (e) {}`, `e` is implied to be `any` or
    /// `unknown`, depending on the TypeScript configuration.
    AnyKeyword,

    /// The `never` keyword.
    ///
    /// Note that unlike TypeScript itself, we never use `never` as a fallback
    /// for when inference determines that a type can have no values. Instead,
    /// we will infer [`Type::Unknown`] in such a case, erring on the side of
    /// caution that the lack of possible values may be a failure of our own
    /// inference.
    NeverKeyword,

    /// The `object` keyword.
    ObjectKeyword,

    /// The `this` keyword.
    ThisKeyword,

    /// The `unknown` keyword.
    ///
    /// This variant may also be used if the `unknown` keyword is implied.
    /// For instance, in `catch (e) {}`, `e` is implied to be `unknown` or
    /// `any`, depending on the TypeScript configuration.
    UnknownKeyword,

    /// The `void` keyword.
    VoidKeyword,
}

impl Default for TypeData {
    fn default() -> Self {
        Self::unknown()
    }
}

impl From<Class> for TypeData {
    fn from(value: Class) -> Self {
        Self::Class(Box::new(value))
    }
}

impl From<Constructor> for TypeData {
    fn from(value: Constructor) -> Self {
        Self::Constructor(Box::new(value))
    }
}

impl From<Function> for TypeData {
    fn from(value: Function) -> Self {
        Self::Function(Box::new(value))
    }
}

impl From<GenericTypeParameter> for TypeData {
    fn from(value: GenericTypeParameter) -> Self {
        Self::Generic(Box::new(value))
    }
}

impl From<Interface> for TypeData {
    fn from(value: Interface) -> Self {
        Self::Interface(Box::new(value))
    }
}

impl From<Literal> for TypeData {
    fn from(value: Literal) -> Self {
        Self::Literal(Box::new(value))
    }
}

impl From<MergedReference> for TypeData {
    fn from(value: MergedReference) -> Self {
        Self::MergedReference(Box::new(value))
    }
}

impl From<Object> for TypeData {
    fn from(value: Object) -> Self {
        Self::Object(Box::new(value))
    }
}

impl From<Module> for TypeData {
    fn from(value: Module) -> Self {
        Self::Module(Box::new(value))
    }
}

impl From<Namespace> for TypeData {
    fn from(value: Namespace) -> Self {
        Self::Namespace(Box::new(value))
    }
}

impl From<Tuple> for TypeData {
    fn from(value: Tuple) -> Self {
        Self::Tuple(Box::new(value))
    }
}

impl From<TypeofExpression> for TypeData {
    fn from(value: TypeofExpression) -> Self {
        Self::TypeofExpression(Box::new(value))
    }
}

impl From<TypeofValue> for TypeData {
    fn from(value: TypeofValue) -> Self {
        Self::TypeofValue(Box::new(value))
    }
}

impl TypeData {
    pub fn array_of(scope_id: ScopeId, ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::from(
            TypeReferenceQualifier::from_name(scope_id, Text::Static("Array"))
                .with_type_parameters([ty]),
        ))
    }

    pub fn as_class(&self) -> Option<&Class> {
        match self {
            Self::Class(class) => Some(class.as_ref()),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<&Function> {
        match self {
            Self::Function(function) => Some(function.as_ref()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(object) => Some(object.as_ref()),
            _ => None,
        }
    }

    pub fn boolean() -> Self {
        Self::Boolean
    }

    /// Returns the type with inference up to the level supported by the given `resolver`.
    #[inline]
    pub fn inferred(&self, resolver: &mut dyn TypeResolver) -> Self {
        let inferred = match self.resolved(resolver) {
            Some(ty) => ty.flattened(resolver),
            None => self.flattened(resolver),
        };
        inferred.unwrap_or_else(|| self.clone())
    }

    #[inline]
    pub fn instance_of(instance: impl Into<TypeInstance>) -> Self {
        Self::InstanceOf(Box::new(instance.into()))
    }

    /// Returns whether the given type has been inferred.
    ///
    /// A type is considered inferred if it is anything except `Self::Unknown`
    /// or an unknown reference, including an unexplicit `unknown` keyword.
    pub fn is_inferred(&self) -> bool {
        match self {
            Self::Reference(TypeReference::Resolved(resolved)) => *resolved != GLOBAL_UNKNOWN_ID,
            Self::Reference(TypeReference::Unknown) | Self::Unknown => false,
            _ => true,
        }
    }

    /// Returns whether the given type is a primitive type.
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::BigInt
            | Self::Boolean
            | Self::Null
            | Self::Number
            | Self::String
            | Self::Symbol
            | Self::Undefined => true,
            Self::Literal(literal) => literal.is_primitive(),
            _ => false,
        }
    }

    pub fn merged_reference(
        ty: Option<impl Into<TypeReference>>,
        value_ty: Option<impl Into<TypeReference>>,
        namespace_ty: Option<impl Into<TypeReference>>,
    ) -> Self {
        Self::MergedReference(Box::new(MergedReference {
            ty: ty.map(Into::into),
            value_ty: value_ty.map(Into::into),
            namespace_ty: namespace_ty.map(Into::into),
        }))
    }

    #[inline]
    pub fn number() -> Self {
        Self::Reference(TypeReference::Resolved(GLOBAL_NUMBER_ID))
    }

    pub fn reference(reference: impl Into<TypeReference>) -> Self {
        Self::Reference(reference.into())
    }

    /// Returns whether the given `instance` wrapper should be stripped from
    /// this type.
    ///
    /// [`TypeData::InstanceOf`] exists primarily in order to distinguish
    /// classes from their instances. When referencing members of a class, you
    /// will access its static members, whereas when you reference members of an
    /// instance of a class, you will access its non-static members.
    ///
    /// Unfortunately, before resolving has taken place, we can't know whether a
    /// given symbol refers to a class or any other type, so we need to
    /// defensively wrap all references with [`TypeData::InstanceOf`] in places
    /// where an instance is expected. For most types however, this is overkill,
    /// and we should strip these wrappers again to ease analysis elsewhere.
    ///
    /// Then there is a second use for [`TypeData::InstanceOf`], which is to
    /// assign concrete types to generic type parameters. For some types,
    /// flattening instances makes sense _unless one of the generics is set_.
    pub fn should_flatten_instance(&self, instance: &TypeInstance) -> bool {
        match self {
            Self::AnyKeyword
            | Self::BigInt
            | Self::Boolean
            | Self::Conditional
            | Self::Global
            | Self::ImportNamespace(_)
            | Self::Literal(_)
            | Self::Module(_)
            | Self::Namespace(_)
            | Self::NeverKeyword
            | Self::Null
            | Self::Number
            | Self::ObjectKeyword
            | Self::String
            | Self::Symbol
            | Self::ThisKeyword
            | Self::Undefined
            | Self::Unknown
            | Self::UnknownKeyword
            | Self::VoidKeyword => true,
            Self::Constructor(_)
            | Self::Function(_)
            | Self::InstanceOf(_)
            | Self::Interface(_)
            | Self::Intersection(_)
            | Self::Object(_)
            | Self::Tuple(_)
            | Self::Union(_) => instance.type_parameters.is_empty(),
            Self::Class(_)
            | Self::Generic(_)
            | Self::MergedReference(_)
            // For references, we don't know. If a reference was pointing to a
            // class, stripping the instance would change its meaning.
            | Self::Reference(_)
            | Self::TypeOperator(_)
            | Self::TypeofExpression(_)
            | Self::TypeofType(_)
            | Self::TypeofValue(_) => false,
        }
    }

    #[inline]
    pub fn string() -> Self {
        Self::Reference(TypeReference::Resolved(GLOBAL_STRING_ID))
    }

    pub fn type_parameters(&self) -> Option<&[TypeReference]> {
        match self {
            Self::Class(class) => Some(&class.type_parameters),
            Self::Function(function) => Some(&function.type_parameters),
            Self::InstanceOf(type_instance) => Some(&type_instance.type_parameters),
            Self::Interface(interface) => Some(&interface.type_parameters),
            _ => None,
        }
    }

    #[inline]
    pub fn unknown() -> Self {
        Self::Reference(TypeReference::Resolved(GLOBAL_UNKNOWN_ID))
    }
}

/// A class definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Class {
    /// Name of the class, if specified in the definition.
    pub name: Option<Text>,

    /// The class's type parameters.
    pub type_parameters: Box<[TypeReference]>,

    /// Type of another class being extended by this one.
    pub extends: Option<TypeReference>,

    /// Interfaces being implemented by this class.
    pub implements: Box<[TypeReference]>,

    /// Class members.
    pub members: Box<[TypeMember]>,
}

/// A constructor definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Constructor {
    /// Generic type parameters used in the call signature.
    pub type_parameters: Box<[TypeReference]>,

    /// Call parameter of the constructor.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<TypeReference>,
}

/// Tracks two types that are associated with the same name.
///
/// TypeScript allows types and values to exist with the same name within the
/// same scope. Such duality can even be tracked across modules, because a
/// single imported symbol can import both the value and the type meaning
/// associated with a single name.
///
/// Ultimately, both references are _types_, since those are what's being
/// tracked by the type system. But one is the type associated with a given
/// name, while the other is the type of the value associated with the same
/// name.
///
/// With a dual reference, which type gets used depends entirely on context.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct MergedReference {
    pub ty: Option<TypeReference>,
    pub value_ty: Option<TypeReference>,
    pub namespace_ty: Option<TypeReference>,
}

impl MergedReference {
    /// Maps the references using the given `mapper` function.
    ///
    /// Returns a [`TypeData::MergedReference`] if multiple mapped references
    /// remain, and a regular [`TypeData::Reference`] if only a single reference
    /// remains.
    ///
    /// Returns `None` if all references are mapped to `None`.
    pub fn map_references(
        &self,
        mapper: impl Fn(&TypeReference) -> Option<TypeReference>,
    ) -> Option<TypeData> {
        let ty = self.ty.as_ref().and_then(&mapper);
        let value_ty = self.value_ty.as_ref().and_then(&mapper);
        let namespace_ty = self.namespace_ty.as_ref().and_then(&mapper);
        match (ty, value_ty, namespace_ty) {
            (None, None, None) => None,
            (Some(reference), None, None)
            | (None, Some(reference), None)
            | (None, None, Some(reference)) => Some(TypeData::Reference(reference)),
            (ty, value_ty, namespace_ty) => Some(TypeData::from(Self {
                ty,
                value_ty,
                namespace_ty,
            })),
        }
    }
}

/// A function definition.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Resolvable)]
pub struct Function {
    /// Whether the function has an `async` specifier or not.
    pub is_async: bool,

    /// Generic type parameters defined in the function signature.
    pub type_parameters: Box<[TypeReference]>,

    /// Name of the function, if specified in the definition.
    pub name: Option<Text>,

    /// Call parameters of the function.
    pub parameters: Box<[FunctionParameter]>,

    /// The function's return type.
    pub return_type: ReturnType,
}

impl Function {
    pub fn with_return_type(self, ty: TypeReference) -> Self {
        Self {
            return_type: ReturnType::Type(ty),
            ..self
        }
    }
}

/// Definition of a function argument.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct FunctionParameter {
    /// Name of the argument, if specified in the definition.
    pub name: Option<Text>,

    /// Type of the argument.
    pub ty: TypeReference,

    /// Bindings created for the parameter within the function body.
    pub bindings: Box<[FunctionParameterBinding]>,

    /// Whether the argument is optional or not.
    pub is_optional: bool,

    /// Whether this is a rest argument (`...`) or not.
    pub is_rest: bool,
}

/// An individual binding created from a function parameter.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct FunctionParameterBinding {
    pub name: Text,
    pub ty: TypeReference,
}

/// Definition of a generic type parameter.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct GenericTypeParameter {
    /// Name of the type parameter.
    pub name: Text,

    /// Optional constraint of the parameter.
    pub constraint: TypeReference,

    /// Default to use if the parameter is unknown.
    pub default: TypeReference,
}

/// An interface definition.
#[derive(Clone, Hash, Eq, PartialEq, Resolvable)]
pub struct Interface {
    /// Name of the interface.
    pub name: Text,

    /// The interface's type parameters.
    pub type_parameters: Box<[TypeReference]>,

    /// Types being extended by this interface.
    pub extends: Box<[TypeReference]>,

    /// Interface members.
    pub members: Box<[TypeMember]>,
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface")
            .field("name", &self.name)
            .field("type_parameters", &self.type_parameters)
            .field("extends", &self.extends)
            .finish()
    }
}

/// The intersection between other types.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Intersection(pub(super) Box<[TypeReference]>);

impl Intersection {
    pub fn types(&self) -> &[TypeReference] {
        &self.0
    }
}

/// Literal value used as a type.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum Literal {
    BigInt(Text),
    Boolean(BooleanLiteral),
    Number(NumberLiteral),
    Object(ObjectLiteral),
    RegExp(Text),
    String(StringLiteral),
    Template(Text), // TODO: Custom impl of PartialEq for template literals
}

impl Literal {
    /// Returns whether the literal is a primitive type.
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::BigInt(_)
                | Self::Boolean(_)
                | Self::Number(_)
                | Self::String(_)
                | Self::Template(_)
        )
    }
}

/// A module definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Module {
    pub name: Text,
    pub members: Box<[TypeMember]>,
}

/// A namespace definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Namespace {
    pub path: Box<[Text]>,
    pub members: Box<[TypeMember]>,
}

/// An object definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Object {
    /// Optional prototype of the object.
    ///
    /// The type that would be returned by `Object.getPrototypeOf()` or the
    /// legacy `object.__proto__`.
    pub prototype: Option<TypeReference>,

    /// The object's own members.
    pub members: Box<[TypeMember]>,
}

/// Object literal used as a type.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct ObjectLiteral(pub(super) Box<[TypeMember]>);

impl ObjectLiteral {
    pub fn into_members(self) -> Box<[TypeMember]> {
        self.0
    }

    pub fn members(&self) -> &[TypeMember] {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum ReturnType {
    Type(TypeReference),
    Predicate(Box<PredicateReturnType>),
    Asserts(Box<AssertsReturnType>),
}

impl Default for ReturnType {
    fn default() -> Self {
        Self::Type(TypeReference::Unknown)
    }
}

impl ReturnType {
    pub fn as_type(&self) -> Option<&TypeReference> {
        match self {
            Self::Type(ty) => Some(ty),
            _ => None,
        }
    }
}

/// Defines the function to which it applies to be a predicate that tests
/// whether one of its arguments is of a given type.
///
/// Predicate functions return `boolean` at runtime.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct PredicateReturnType {
    pub parameter_name: Text,
    pub ty: TypeReference,
}

/// Defines the function to which it applies to be an assertion that asserts
/// one of its arguments to be of a given type.
///
/// Assertion functions throw at runtime if the type assertion fails.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct AssertsReturnType {
    pub parameter_name: Text,
    pub ty: TypeReference,
}

/// Tuple type.
///
/// Tuples in TypeScript are created using `Array`s of a fixed size.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Tuple(pub(super) Box<[TupleElementType]>);

impl Tuple {
    pub fn elements(&self) -> &[TupleElementType] {
        &self.0
    }

    /// Returns the element at the given index.
    pub fn get_element(&self, index: usize) -> Option<&TupleElementType> {
        self.0
            .get(index)
            .or_else(|| self.0.last().filter(|last| last.is_rest))
    }

    /// Returns a new tuple starting at the given index.
    pub fn slice_from(&self, resolver_id: ResolverId, index: usize) -> Self {
        Self(
            self.0
                .iter()
                .skip(index)
                .map(|element| TupleElementType {
                    ty: resolver_id
                        .apply_module_id_to_reference(&element.ty)
                        .into_owned(),
                    name: element.name.clone(),
                    ..*element
                })
                .collect(),
        )
    }
}

/// An individual element within a tuple.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TupleElementType {
    /// Type of the element.
    pub ty: TypeReference,

    /// Name of the element, if it has one.
    pub name: Option<Text>,

    /// Whether this element is optional or not.
    pub is_optional: bool,

    /// Whether this is a rest element (`...`) or not.
    pub is_rest: bool,
}

/// Members of a definition, such as an object, namespace or module.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeMember {
    pub kind: TypeMemberKind,
    pub ty: TypeReference,
}

impl TypeMember {
    /// Returns a reference to the type of the member if we dereference it.
    ///
    /// This means if the member represents a getter or setter, it will
    /// dereference to the type of the property being get or set.
    pub fn deref_ty<'a>(&'a self, resolver: &'a dyn TypeResolver) -> Cow<'a, TypeReference> {
        if self.is_getter() {
            resolver
                .resolve_and_get(&self.ty)
                .and_then(|resolved| match resolved.as_raw_data() {
                    TypeData::Function(function) => function
                        .return_type
                        .as_type()
                        .map(|return_ty| resolved.apply_module_id_to_reference(return_ty)),
                    _ => None,
                })
                .unwrap_or(Cow::Owned(TypeReference::Resolved(GLOBAL_UNKNOWN_ID)))
        } else {
            Cow::Borrowed(&self.ty)
        }
    }

    pub fn has_name(&self, name: &str) -> bool {
        self.kind.has_name(name)
    }

    #[inline]
    pub fn is_constructor(&self) -> bool {
        self.kind.is_constructor()
    }

    pub fn is_index_signature_with_ty(&self, predicate: impl Fn(&TypeReference) -> bool) -> bool {
        match &self.kind {
            TypeMemberKind::IndexSignature(ty) => predicate(ty),
            _ => false,
        }
    }

    #[inline]
    pub fn is_getter(&self) -> bool {
        self.kind.is_getter()
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.kind.is_static()
    }

    pub fn name(&self) -> Option<Text> {
        self.kind.name()
    }
}

/// Kind of a [`TypeMember`], with an optional name.
// TODO: Include setters.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum TypeMemberKind {
    CallSignature,
    Constructor,
    Getter(Text),
    IndexSignature(TypeReference),
    Named(Text),
    NamedStatic(Text),
}

impl TypeMemberKind {
    pub fn has_name(&self, name: &str) -> bool {
        match self {
            Self::CallSignature | Self::IndexSignature(_) => false,
            Self::Constructor => name == "constructor",
            Self::Getter(own_name) | Self::Named(own_name) | Self::NamedStatic(own_name) => {
                *own_name == name
            }
        }
    }

    #[inline]
    pub fn is_call_signature(&self) -> bool {
        matches!(self, Self::CallSignature)
    }

    #[inline]
    pub fn is_constructor(&self) -> bool {
        matches!(self, Self::Constructor)
    }

    #[inline]
    pub fn is_getter(&self) -> bool {
        matches!(self, Self::Getter(_))
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Constructor | Self::NamedStatic(_))
    }

    pub fn name(&self) -> Option<Text> {
        match self {
            Self::CallSignature | Self::IndexSignature(_) => None,
            Self::Constructor => Some(Text::Static("constructor")),
            Self::Getter(name) | Self::Named(name) | Self::NamedStatic(name) => Some(name.clone()),
        }
    }
}

/// Instance of another type.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeInstance {
    /// The type being instantiated.
    pub ty: TypeReference,

    /// Generic type parameters that should be passed onto the type being
    /// instantiated.
    pub type_parameters: Box<[TypeReference]>,
}

impl From<TypeReference> for TypeInstance {
    fn from(ty: TypeReference) -> Self {
        Self {
            ty,
            type_parameters: [].into(),
        }
    }
}

impl TypeInstance {
    pub fn has_known_type_parameters(&self) -> bool {
        self.type_parameters.iter().any(TypeReference::is_known)
    }
}

/// Reference to the type of a JavaScript expression.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum TypeofExpression {
    Addition(TypeofAdditionExpression),
    Await(TypeofAwaitExpression),
    BitwiseNot(TypeofBitwiseNotExpression),
    Call(TypeofCallExpression),
    Conditional(TypeofConditionalExpression),
    Destructure(TypeofDestructureExpression),
    LogicalAnd(TypeofLogicalAndExpression),
    LogicalOr(TypeofLogicalOrExpression),
    New(TypeofNewExpression),
    NullishCoalescing(TypeofNullishCoalescingExpression),
    StaticMember(TypeofStaticMemberExpression),
    Super(TypeofThisOrSuperExpression),
    This(TypeofThisOrSuperExpression),
    Typeof(TypeofTypeofExpression),
    UnaryMinus(TypeofUnaryMinusExpression),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofAdditionExpression {
    pub left: TypeReference,
    pub right: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofAwaitExpression {
    pub argument: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofBitwiseNotExpression {
    pub argument: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofCallExpression {
    pub callee: TypeReference,
    pub arguments: Box<[CallArgumentType]>,
}

/// Represents the type of a ternary expression.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofConditionalExpression {
    pub test: TypeReference,
    pub consequent: TypeReference,
    pub alternate: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofDestructureExpression {
    /// The type being destructured.
    pub ty: TypeReference,

    /// The field being destructured.
    pub destructure_field: DestructureField,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum DestructureField {
    Index(usize),
    Name(Text),
    RestExcept(Box<[Text]>),
    RestFrom(usize),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofLogicalAndExpression {
    pub left: TypeReference,
    pub right: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofLogicalOrExpression {
    pub left: TypeReference,
    pub right: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofNewExpression {
    pub callee: TypeReference,
    pub arguments: Box<[CallArgumentType]>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum CallArgumentType {
    Argument(TypeReference),
    Spread(TypeReference),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofNullishCoalescingExpression {
    pub left: TypeReference,
    pub right: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofStaticMemberExpression {
    pub object: TypeReference,
    pub member: Text,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofThisOrSuperExpression {
    /// Type from which the `this` or `super` expression should be resolved.
    pub parent: TypeReference,
}

/// Type of expressions using the `typeof` operator.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofTypeofExpression {
    /// Reference to the type of the expression from which a string
    /// representation should be created.
    pub argument: TypeReference,
}

/// Reference to the type of a named JavaScript value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeofValue {
    /// Identifier of the type being referenced.
    ///
    /// We explicitly do not allow full expressions to be used as values,
    /// meaning our inference needs to break down expressions into parts before
    /// deciding the values to reference. See [TypeofExpression] for that.
    pub identifier: Text,

    /// The resolved type.
    pub ty: TypeReference,

    /// ID of the scope from which the value is being referenced.
    pub scope_id: Option<ScopeId>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeofUnaryMinusExpression {
    pub argument: TypeReference,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct TypeOperatorType {
    pub operator: TypeOperator,
    pub ty: TypeReference,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Resolvable)]
pub enum TypeOperator {
    Keyof,
    Readonly,
    Unique,
}

impl FromStr for TypeOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "keyof" => Ok(Self::Keyof),
            "readonly" => Ok(Self::Readonly),
            "unique" => Ok(Self::Unique),
            _ => Err(()),
        }
    }
}

/// Reference to another type definition.
///
/// This definition may require importing from another module.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum TypeReference {
    Qualifier(Box<TypeReferenceQualifier>),
    Resolved(ResolvedTypeId),
    Import(Box<TypeImportQualifier>),
    #[default]
    Unknown,
}

impl From<TypeReferenceQualifier> for TypeReference {
    fn from(qualifier: TypeReferenceQualifier) -> Self {
        Self::Qualifier(Box::new(qualifier))
    }
}

impl From<ResolvedTypeId> for TypeReference {
    fn from(resolved_id: ResolvedTypeId) -> Self {
        Self::Resolved(resolved_id)
    }
}

impl From<TypeImportQualifier> for TypeReference {
    fn from(qualifier: TypeImportQualifier) -> Self {
        Self::Import(Box::new(qualifier))
    }
}

impl TypeReference {
    /// Returns `true` if the reference references anything but `Unknown`.
    #[inline]
    pub fn is_known(&self) -> bool {
        match self {
            Self::Import(_) => true,
            Self::Qualifier(_) => true,
            Self::Resolved(resolved_id) => *resolved_id != GLOBAL_UNKNOWN_ID,
            Self::Unknown => false,
        }
    }

    /// Merges the generic type parameters referenced by `incoming` into `base`.
    pub fn merge_parameters(base: &[Self], incoming: &[Self]) -> Box<[Self]> {
        base.iter()
            .enumerate()
            .map(|(i, param)| incoming.get(i).unwrap_or(param).clone())
            .collect()
    }

    pub fn resolved_params(&self, resolver: &mut dyn TypeResolver) -> Box<[Self]> {
        match self {
            Self::Qualifier(qualifier) => qualifier
                .type_parameters
                .iter()
                .map(|param| param.resolved(resolver).unwrap_or_else(|| param.clone()))
                .collect(),
            _ => [].into(),
        }
    }

    pub fn set_module_id(&mut self, module_id: ModuleId) {
        match self {
            Self::Qualifier(_) => {
                // When we assign a module ID in order to store a type in the
                // scoped resolver, we also clear out qualifiers to avoid
                // resolving from an incorrect scope.
                *self = Self::Unknown;
            }
            Self::Resolved(resolved_id) => {
                *resolved_id = resolved_id.with_module_id(module_id);
            }
            _ => {}
        }
    }

    pub fn with_excluded_binding_id(self, binding_id: BindingId) -> Self {
        match self {
            Self::Qualifier(qualifier) => {
                Self::Qualifier(Box::new(qualifier.with_excluded_binding_id(binding_id)))
            }
            other => other,
        }
    }
}

/// Qualifier for a type that should be imported from another module.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeImportQualifier {
    /// The imported symbol.
    pub symbol: ImportSymbol,

    /// Resolved path of the module to import the type from.
    pub resolved_path: ResolvedPath,

    /// If `true`, this qualifier imports the type only.
    pub type_only: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ImportSymbol {
    /// Imports the `default` export.
    Default,

    /// Imports a named symbol.
    Named(Text),

    /// Imports all symbols, including the `default` export.
    All,
}

impl From<Text> for ImportSymbol {
    fn from(name: Text) -> Self {
        Self::Named(name)
    }
}

impl From<&'static str> for ImportSymbol {
    fn from(name: &'static str) -> Self {
        Self::Named(name.into())
    }
}

/// Path of identifiers to a referenced type, with associated type parameters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeReferenceQualifier {
    /// The identifier path.
    pub path: Box<[Text]>,

    /// Generic type parameters specified in the reference.
    pub type_parameters: Box<[TypeReference]>,

    /// ID of the scope from which the qualifier is being referenced.
    pub scope_id: ScopeId,

    /// If `true`, this qualifier can reference types (and namespaces) only.
    pub type_only: bool,

    /// Optional [`BindingId`] this qualifier may not reference.
    ///
    /// This is used to prevent self-references.
    pub excluded_binding_id: Option<BindingId>,
}

impl TypeReferenceQualifier {
    pub fn has_known_type_parameters(&self) -> bool {
        self.type_parameters.iter().any(TypeReference::is_known)
    }

    /// Checks whether this type qualifier references an `Array` type.
    ///
    /// This method simply checks whether the reference is for a literal
    /// `Array`, without considering whether another symbol named `Array` is
    /// in scope. It can be used _after_ type resolution has failed to find a
    /// `Array` symbol in scope, but should not be used _instead of_ such type
    /// resolution.
    pub fn is_array(&self) -> bool {
        self.path.len() == 1 && self.path[0] == "Array"
    }

    /// Checks whether this type qualifier references a `Promise` type.
    ///
    /// This method simply checks whether the reference is for a literal
    /// `Promise`, without considering whether another symbol named `Promise` is
    /// in scope. It can be used _after_ type resolution has failed to find a
    /// `Promise` symbol in scope, but should not be used _instead of_ such type
    /// resolution.
    pub fn is_promise(&self) -> bool {
        self.path.len() == 1 && self.path[0] == "Promise"
    }

    pub fn with_excluded_binding_id(mut self, binding_id: BindingId) -> Self {
        self.excluded_binding_id = Some(binding_id);
        self
    }

    pub fn without_type_parameters(&self) -> Self {
        Self {
            path: self.path.clone(),
            type_parameters: [].into(),
            scope_id: self.scope_id,
            type_only: self.type_only,
            excluded_binding_id: self.excluded_binding_id,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingId(u32);

impl BindingId {
    pub const fn new(index: usize) -> Self {
        // SAFETY: We don't handle files exceeding `u32::MAX` bytes.
        // Thus, it isn't possible to exceed `u32::MAX` bindings.
        Self(index as u32)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

// We allow conversion from `BindingId` into `TypeId`, and vice versa, because
// for project-level `ResolvedTypeId` instances, the `TypeId` is an indirection
// that is resolved through a binding.
impl From<BindingId> for TypeId {
    fn from(id: BindingId) -> Self {
        Self::new(id.0 as usize)
    }
}

impl From<TypeId> for BindingId {
    fn from(id: TypeId) -> Self {
        Self::new(id.index())
    }
}

// We use `NonZeroU32` to allow niche optimizations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScopeId(pub(crate) std::num::NonZeroU32);

// We don't implement `From<usize> for ScopeId` and `From<ScopeId> for usize`
// to ensure that the API consumers don't create `ScopeId`.
impl ScopeId {
    pub const GLOBAL: Self = Self::new(0);

    pub const fn new(index: usize) -> Self {
        // SAFETY: We don't handle files exceeding `u32::MAX` bytes.
        // Thus, it isn't possible to exceed `u32::MAX` scopes.
        //
        // Adding 1 ensures that the value is never equal to 0.
        // Instead of adding 1, we could XOR the value with `u32::MAX`.
        // This is what the [nonmax](https://docs.rs/nonmax/latest/nonmax/) crate does.
        // However, this doesn't preserve the order.
        // It is why we opted for adding 1.
        Self(unsafe { std::num::NonZeroU32::new_unchecked(index.unchecked_add(1) as u32) })
    }

    pub const fn index(self) -> usize {
        // SAFETY: The internal representation ensures that the value is never equal to 0.
        // Thus, it is safe to substract 1.
        (unsafe { self.0.get().unchecked_sub(1) }) as usize
    }
}

/// A union of types.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Resolvable)]
pub struct Union(pub(super) Box<[TypeReference]>);

impl Union {
    pub fn contains(&self, ty: &TypeReference) -> bool {
        self.0.contains(ty)
    }

    pub fn types(&self) -> &[TypeReference] {
        &self.0
    }

    pub fn with_type(&self, ty: TypeReference) -> Self {
        Self(self.0.iter().cloned().chain(Some(ty)).collect())
    }
}
