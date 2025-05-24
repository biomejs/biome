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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::{ops::Deref, str::FromStr, sync::Arc};

use biome_js_type_info_macros::Resolvable;
use biome_resolver::ResolvedPath;
use biome_rowan::Text;

use crate::globals::{GLOBAL_PROMISE_ID, GLOBAL_UNKNOWN_ID, PROMISE_ID};
use crate::type_info::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
use crate::{GLOBAL_RESOLVER, Resolvable, ResolvedTypeData, ResolvedTypeId, TypeResolver};

const UNKNOWN: TypeData = TypeData::Unknown;

/// Type identifier referencing the type in a resolver's `types` vector.
///
/// Note that separate modules typically use separate resolvers. Because of
/// this, type IDs are only unique within a single module/resolver.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Resolvable)]
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
            resolver: Arc::new(GLOBAL_RESOLVER.clone()),
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
    pub fn from_data(mut resolver: Box<dyn TypeResolver>, data: TypeData) -> Self {
        let id = resolver.register_and_resolve(data);
        Self {
            resolver: Arc::from(resolver),
            id,
        }
    }

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
        match self.deref() {
            TypeData::Union(union) => union
                .types()
                .iter()
                .filter_map(|ty| self.resolve(ty))
                .any(predicate),
            _ => false,
        }
    }

    /// Returns whether this type is the `Promise` class.
    pub fn is_promise(&self) -> bool {
        self.id.is_global() && self.id() == PROMISE_ID
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(&self) -> bool {
        self.resolved_data()
            .is_some_and(|ty| ty.is_instance_of(self.resolver.as_ref(), GLOBAL_PROMISE_ID))
    }

    /// Returns whether the given type is known to reference a function that
    /// returns a `Promise`.
    pub fn is_function_that_returns_promise(&self) -> bool {
        match self.deref() {
            TypeData::Function(function) => function
                .return_type
                .as_type()
                .and_then(|ty| self.resolve(ty))
                .is_some_and(|ty| ty.is_promise()),
            _ => false,
        }
    }

    pub fn resolve(&self, ty: &TypeReference) -> Option<Self> {
        self.resolver
            .resolve_reference(&self.id.apply_module_id_to_reference(ty))
            .map(|resolved_id| self.with_resolved_id(resolved_id))
    }

    #[inline]
    fn resolved_data(&self) -> Option<ResolvedTypeData> {
        self.resolver.get_by_resolved_id(self.id)
    }

    fn with_resolved_id(&self, id: ResolvedTypeId) -> Self {
        Self {
            resolver: self.resolver.clone(),
            id,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub enum TypeData {
    /// The type is unknown because inference couldn't determine a type.
    ///
    /// This is different from `UnknownKeyword`, because an explicit `unknown`
    /// should not be counted as a failure of the inference.
    #[default]
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

    // Complex types
    Class(Box<Class>),
    Constructor(Box<Constructor>),
    Function(Box<Function>),
    Module(Box<Module>),
    Namespace(Box<Namespace>),
    Object(Box<Object>),
    Tuple(Box<Tuple>),

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
    Reference(Box<TypeReference>),

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

impl From<Literal> for TypeData {
    fn from(value: Literal) -> Self {
        Self::Literal(Box::new(value))
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

impl From<TypeofValue> for TypeData {
    fn from(value: TypeofValue) -> Self {
        Self::TypeofValue(Box::new(value))
    }
}

impl TypeData {
    pub fn array_of(ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::Qualifier(TypeReferenceQualifier {
            path: [Text::Static("Array")].into(),
            type_parameters: [ty].into(),
            scope_id: None,
        }))
    }

    pub fn as_class(&self) -> Option<&Class> {
        match self {
            Self::Class(class) => Some(class.as_ref()),
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
        self.resolved(resolver).flattened(resolver)
    }

    /// Creates an intersection of type references.
    ///
    /// References are automatically deduplicated. If only a single type
    /// remains, an instance of `Self::Reference` is returned instead of
    /// `Self::Intersection`.
    pub fn intersection_of(mut types: Vec<TypeReference>) -> Self {
        types.dedup();
        match types.len().cmp(&1) {
            Ordering::Greater => Self::Intersection(Box::new(Intersection(types.into()))),
            Ordering::Equal => Self::reference(types.remove(0)),
            Ordering::Less => Self::Unknown,
        }
    }

    /// Returns whether the given type has been inferred.
    ///
    /// A type is considered inferred if it is anything except `Self::Unknown`,
    /// including an unexplicit `unknown` keyword.
    pub fn is_inferred(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    pub fn reference(reference: impl Into<TypeReference>) -> Self {
        Self::Reference(Box::new(reference.into()))
    }

    pub fn type_parameters(&self) -> Option<&[GenericTypeParameter]> {
        match self {
            Self::Class(class) => Some(&class.type_parameters),
            Self::Function(function) => Some(&function.type_parameters),
            Self::InstanceOf(instance) => Some(&instance.type_parameters),
            _ => None,
        }
    }

    /// Creates a union of type references.
    ///
    /// References are automatically deduplicated. If only a single type
    /// remains, an instance of `Self::Reference` is returned instead of
    /// `Self::Union`.
    pub fn union_of(mut types: Vec<TypeReference>) -> Self {
        types.dedup();
        match types.len().cmp(&1) {
            Ordering::Greater => Self::Union(Box::new(Union(types.into()))),
            Ordering::Equal => Self::reference(types.remove(0)),
            Ordering::Less => Self::Unknown,
        }
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}

/// A class definition.
#[derive(Clone, PartialEq, Resolvable)]
pub struct Class {
    /// Name of the class, if specified in the definition.
    pub name: Option<Text>,

    /// The class's type parameters.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Type of another class being extended by this one.
    pub extends: Option<TypeReference>,

    /// Class members.
    pub members: Box<[TypeMember]>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .field("type_parameters", &self.type_parameters)
            .field("extends", &self.extends)
            .finish()
    }
}

/// A constructor definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Constructor {
    /// Generic type parameters used in the call signature.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameter of the constructor.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<TypeReference>,
}

/// A function definition.
#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct Function {
    /// Whether the function has an `async` specifier or not.
    pub is_async: bool,

    /// Generic type parameters defined in the function signature.
    pub type_parameters: Box<[GenericTypeParameter]>,

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
#[derive(Clone, Debug, PartialEq, Resolvable)]
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
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct FunctionParameterBinding {
    pub name: Text,
    pub ty: TypeData,
}

/// Definition of a generic type parameter.
// TODO: Include modifiers and constraints.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct GenericTypeParameter {
    /// Name of the type parameter.
    pub name: Text,

    /// The resolved type to use.
    ///
    /// May be the default type from the type definition.
    pub ty: TypeReference,
}

impl GenericTypeParameter {
    /// Merges the parameters from `incoming` into `base`.
    pub fn merge_parameters(base: &[Self], incoming: &[Self]) -> Box<[Self]> {
        base.iter()
            .enumerate()
            .map(|(i, param)| Self {
                name: param.name.clone(),
                ty: incoming
                    .get(i)
                    .map_or_else(|| param.ty.clone(), |incoming| incoming.ty.clone()),
            })
            .collect()
    }

    /// Merges the `types` into `parameters`.
    pub fn merge_types(parameters: &[Self], types: &[TypeReference]) -> Box<[Self]> {
        parameters
            .iter()
            .enumerate()
            .map(|(i, param)| Self {
                name: param.name.clone(),
                ty: types.get(i).cloned().unwrap_or_else(|| param.ty.clone()),
            })
            .collect()
    }
}

/// The intersection between other types.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Intersection(pub(super) Box<[TypeReference]>);

impl Intersection {
    pub fn types(&self) -> &[TypeReference] {
        &self.0
    }
}

/// Literal value used as a type.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum Literal {
    BigInt(Text),
    Boolean(BooleanLiteral),
    Null,
    Number(NumberLiteral),
    Object(ObjectLiteral),
    RegExp(Text),
    String(StringLiteral),
    Template(Text), // TODO: Custom impl of PartialEq for template literals
}

/// A module definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Module {
    pub name: Text,
    pub members: Box<[TypeMember]>,
}

/// A namespace definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Namespace {
    pub path: Box<[Text]>,
    pub members: Box<[TypeMember]>,
}

/// An object definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
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
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct ObjectLiteral(pub(super) Box<[TypeMember]>);

impl ObjectLiteral {
    pub fn members(&self) -> &[TypeMember] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum ReturnType {
    Type(TypeReference),
    Predicate(PredicateReturnType),
    Asserts(AssertsReturnType),
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
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct PredicateReturnType {
    pub parameter_name: Text,
    pub ty: TypeReference,
}

/// Defines the function to which it applies to be an assertion that asserts
/// one of its arguments to be of a given type.
///
/// Assertion functions throw at runtime if the type assertion fails.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct AssertsReturnType {
    pub parameter_name: Text,
    pub ty: TypeReference,
}

/// Tuple type.
///
/// Tuples in TypeScript are created using `Array`s of a fixed size.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Tuple(pub(super) Box<[TupleElementType]>);

impl Tuple {
    pub fn elements(&self) -> &[TupleElementType] {
        &self.0
    }

    /// Returns the type at the given index.
    pub fn get_ty<'a>(
        &'a self,
        resolver: &'a mut dyn TypeResolver,
        index: usize,
    ) -> ResolvedTypeData<'a> {
        let resolved_id = if let Some(elem_type) = self.0.get(index) {
            let ty = elem_type.ty.clone();
            let id = if elem_type.is_optional {
                resolver.optional(ty)
            } else {
                resolver.register_type(TypeData::reference(ty))
            };
            ResolvedTypeId::new(resolver.level(), id)
        } else {
            self.0
                .last()
                .filter(|last| last.is_rest)
                .map(|last| resolver.optional(last.ty.clone()))
                .map_or(GLOBAL_UNKNOWN_ID, |id| {
                    ResolvedTypeId::new(resolver.level(), id)
                })
        };

        resolver
            .get_by_resolved_id(resolved_id)
            .expect("tuple element type must be registered")
    }

    /// Returns a new tuple starting at the given index.
    pub fn slice_from(&self, index: usize) -> Self {
        Self(self.0.iter().skip(index).cloned().collect())
    }
}

/// An individual element within a tuple.
#[derive(Clone, Debug, PartialEq, Resolvable)]
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
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeMember {
    pub kind: TypeMemberKind,
    pub is_static: bool,
    pub ty: TypeReference,
}

impl TypeMember {
    pub fn has_name(&self, name: &str) -> bool {
        self.kind.has_name(name)
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn name(&self) -> Option<Text> {
        self.kind.name()
    }
}

/// Kind of a [`TypeMember`], with an optional name.
// TODO: Include getters, setters and index signatures.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum TypeMemberKind {
    CallSignature,
    Constructor,
    Named(Text),
}

impl TypeMemberKind {
    pub fn has_name(&self, name: &str) -> bool {
        match self {
            Self::CallSignature => false,
            Self::Constructor => name == "constructor",
            Self::Named(own_name) => *own_name == name,
        }
    }

    pub fn name(&self) -> Option<Text> {
        match self {
            Self::CallSignature => None,
            Self::Constructor => Some(Text::Static("constructor")),
            Self::Named(name) => Some(name.clone()),
        }
    }
}

/// Instance of another type.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeInstance {
    /// The type being instantiated.
    pub ty: TypeReference,

    /// Generic type parameters that should be passed onto the type being
    /// instantiated.
    pub type_parameters: Box<[GenericTypeParameter]>,
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
        !self.type_parameters.is_empty()
            && self
                .type_parameters
                .iter()
                .any(|param| param.ty != TypeReference::Unknown)
    }
}

/// Reference to the type of a JavaScript expression.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum TypeofExpression {
    Addition(TypeofAdditionExpression),
    Await(TypeofAwaitExpression),
    Call(TypeofCallExpression),
    Destructure(TypeofDestructureExpression),
    New(TypeofNewExpression),
    StaticMember(TypeofStaticMemberExpression),
    Super(TypeofThisOrSuperExpression),
    This(TypeofThisOrSuperExpression),
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofAdditionExpression {
    pub left: TypeReference,
    pub right: TypeReference,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofAwaitExpression {
    pub argument: TypeReference,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofCallExpression {
    pub callee: TypeReference,
    pub arguments: Box<[CallArgumentType]>,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofDestructureExpression {
    /// The type being destructured.
    pub ty: TypeReference,

    /// The field being destructured.
    pub destructure_field: DestructureField,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum DestructureField {
    Index(usize),
    Name(Text),
    RestExcept(Box<[Text]>),
    RestFrom(usize),
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofNewExpression {
    pub callee: TypeReference,
    pub arguments: Box<[CallArgumentType]>,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum CallArgumentType {
    Argument(TypeReference),
    Spread(TypeReference),
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofStaticMemberExpression {
    pub object: TypeReference,
    pub member: Text,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofThisOrSuperExpression {
    /// Type from which the `this` or `super` expression should be resolved.
    pub parent: TypeReference,
}

/// Reference to the type of a named JavaScript value.
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeOperatorType {
    pub operator: TypeOperator,
    pub ty: TypeReference,
}

#[derive(Clone, Copy, Debug, PartialEq, Resolvable)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TypeReference {
    Qualifier(TypeReferenceQualifier),
    Resolved(ResolvedTypeId),
    Import(TypeImportQualifier),
    #[default]
    Unknown,
}

impl From<TypeReferenceQualifier> for TypeReference {
    fn from(qualifier: TypeReferenceQualifier) -> Self {
        Self::Qualifier(qualifier)
    }
}

impl From<ResolvedTypeId> for TypeReference {
    fn from(resolved_id: ResolvedTypeId) -> Self {
        Self::Resolved(resolved_id)
    }
}

impl From<TypeImportQualifier> for TypeReference {
    fn from(qualifier: TypeImportQualifier) -> Self {
        Self::Import(qualifier)
    }
}

impl TypeReference {
    #[inline]
    pub fn is_known(&self) -> bool {
        *self != Self::Unknown
    }

    pub fn resolved_params(&self, resolver: &mut dyn TypeResolver) -> Box<[Self]> {
        match self {
            Self::Qualifier(qualifier) => qualifier
                .type_parameters
                .iter()
                .map(|param| param.resolved(resolver))
                .collect(),
            _ => [].into(),
        }
    }
}

/// Qualifier for a type that should be imported from another module.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeImportQualifier {
    /// The imported symbol.
    pub symbol: ImportSymbol,

    /// Resolved path of the module to import the type from.
    pub resolved_path: ResolvedPath,
}

#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct TypeReferenceQualifier {
    /// The identifier path.
    pub path: Box<[Text]>,

    /// Generic type parameters specified in the reference.
    pub type_parameters: Box<[TypeReference]>,

    /// ID of the scope from which the qualifier is being referenced.
    pub scope_id: Option<ScopeId>,
}

impl TypeReferenceQualifier {
    pub fn has_known_type_parameters(&self) -> bool {
        !self.type_parameters.is_empty()
            && self
                .type_parameters
                .iter()
                .any(|param| *param != TypeReference::Unknown)
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

    pub fn with_scope_id(self, scope_id: ScopeId) -> Self {
        Self {
            scope_id: Some(scope_id),
            ..self
        }
    }

    pub fn without_type_parameters(&self) -> Self {
        Self {
            path: self.path.clone(),
            type_parameters: [].into(),
            scope_id: self.scope_id,
        }
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
#[derive(Clone, Debug, PartialEq, Resolvable)]
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
