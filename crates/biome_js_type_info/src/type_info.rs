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

use std::fmt::Debug;
use std::{ops::Deref, str::FromStr, sync::Arc};

use biome_js_type_info_macros::Resolvable;
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};

use crate::globals::{
    GLOBAL_ARRAY_ID, GLOBAL_PROMISE_ID, GLOBAL_TYPE_MEMBERS, GLOBAL_UNKNOWN_ID, PROMISE_ID,
};
use crate::type_info::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
use crate::{GLOBAL_RESOLVER, Resolvable, ResolvedTypeId, TypeResolver};

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
        self.resolver
            .get_by_resolved_id(self.id)
            .unwrap_or(&UNKNOWN)
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

    /// Returns this type's [`TypeId`].
    ///
    /// **Warning:** Type IDs can only be safely compared with other IDs from
    ///              the same module.
    pub fn id(&self) -> TypeId {
        self.id.id()
    }

    /// Returns whether this type is the `Promise` class.
    pub fn is_promise(&self) -> bool {
        self.id.is_global() && self.id() == PROMISE_ID
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(&self) -> bool {
        self.deref()
            .is_instance_of(self.resolver.as_ref(), GLOBAL_PROMISE_ID)
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

    fn resolve(&self, ty: &TypeReference) -> Option<Self> {
        self.resolver
            .resolve_reference(ty)
            .map(|resolved_id| self.with_resolved_id(resolved_id))
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

impl TypeData {
    /// Iterates all member fields, including those belonging to extended
    /// classes or prototype objects.
    ///
    /// Note that members which are inherited and overridden may appear multiple
    /// times, but the member that is closest to the current type is guaranteed
    /// to come first.
    pub fn all_members<'a>(
        &'a self,
        resolver: &'a dyn TypeResolver,
    ) -> impl Iterator<Item = &'a TypeMember> {
        TypeMemberIterator {
            resolver,
            seen_types: Vec::new(),
            owner: TypeMemberOwner::from_type_data(self),
            index: 0,
        }
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

    /// Returns the type of an array's elements, if this object is an instance of `Array`.
    pub fn find_array_element_type<'a>(
        &'a self,
        resolver: &'a dyn TypeResolver,
    ) -> Option<&'a Self> {
        if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) {
            self.find_type_parameter(resolver, "T")
                .and_then(|reference| resolver.resolve_and_get(reference))
        } else {
            None
        }
    }

    /// Returns the type of an element at a given index, if this object is an
    /// array or a tuple.
    pub fn find_element_type_at_index<'a>(
        &'a self,
        resolver: &'a mut dyn TypeResolver,
        index: usize,
    ) -> Option<&'a Self> {
        match self {
            Self::Tuple(tuple) => Some(tuple.get_ty(resolver, index)),
            _ if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) => self
                .find_type_parameter(resolver, "T")
                .cloned()
                .map(|reference| resolver.optional(reference))
                .map(|id| resolver.get_by_id(id)),
            _ => None,
        }
    }

    /// Returns the promised type, if this object is an instance of `Promise`.
    pub fn find_promise_type<'a>(&'a self, resolver: &'a dyn TypeResolver) -> Option<&'a Self> {
        if self.is_instance_of(resolver, GLOBAL_PROMISE_ID) {
            self.find_type_parameter(resolver, "T")
                .and_then(|reference| resolver.resolve_and_get(reference))
        } else {
            None
        }
    }

    /// Returns the type of elements from a given index, if this object is an
    /// array or a tuple.
    pub fn find_type_of_elements_from_index(
        &self,
        resolver: &mut dyn TypeResolver,
        index: usize,
    ) -> Option<Self> {
        match self {
            Self::Tuple(tuple) => Some(Self::Tuple(Box::new(tuple.slice_from(index)))),
            _ if self.is_instance_of(resolver, GLOBAL_ARRAY_ID) => {
                match self.find_type_parameter(resolver, "T") {
                    Some(elem_ty) => Some(Self::InstanceOf(Box::new(TypeInstance {
                        ty: GLOBAL_ARRAY_ID.into(),
                        type_parameters: Box::new([GenericTypeParameter {
                            name: Text::Static("T"),
                            ty: elem_ty.clone(),
                        }]),
                    }))),
                    None => resolver.get_by_resolved_id(GLOBAL_ARRAY_ID).cloned(),
                }
            }
            _ => None,
        }
    }

    pub fn find_type_parameter<'a>(
        &'a self,
        resolver: &'a dyn TypeResolver,
        parameter_name: &str,
    ) -> Option<&'a TypeReference> {
        let mut seen_types = Vec::new();
        let mut current_object = Some(self);
        while let Some(current) = current_object {
            if let Some(argument) = current
                .type_parameters()
                .iter()
                .flat_map(|params| params.iter())
                .find(|param| param.name == parameter_name && param.ty.is_known())
            {
                return Some(&argument.ty);
            }

            let Some(next_object) = current
                .prototype(resolver)
                .and_then(|prototype| resolver.resolve_reference(prototype))
            else {
                break;
            };

            if seen_types.contains(&next_object) {
                break;
            }

            seen_types.push(next_object);
            current_object = resolver.get_by_resolved_id(next_object);
        }

        None
    }

    /// Returns the type with inference up to the level supported by the given `resolver`.
    #[inline]
    pub fn inferred(&self, resolver: &mut dyn TypeResolver) -> Self {
        self.resolved(resolver).flattened(resolver)
    }

    /// Returns whether this object is an instance of the type with the given ID.
    pub fn is_instance_of(&self, resolver: &dyn TypeResolver, id: ResolvedTypeId) -> bool {
        let mut seen_types = Vec::new();
        let mut current_object = Some(self);
        while let Some(current) = current_object {
            let Some(prototype) = current.prototype(resolver) else {
                break;
            };

            let Some(next_id) = resolver.resolve_reference(prototype) else {
                break;
            };

            if next_id == id {
                return true;
            }

            if seen_types.contains(&next_id) {
                break;
            }

            seen_types.push(next_id);
            current_object = resolver.get_by_resolved_id(next_id);
        }

        false
    }

    /// Returns whether this type is an instance of a `Promise`.
    pub fn is_promise_instance(&self, resolver: &dyn TypeResolver) -> bool {
        self.is_instance_of(resolver, GLOBAL_PROMISE_ID)
    }

    /// Returns whether the given type has been inferred.
    ///
    /// A type is considered inferred if it is anything except `Self::Unknown`,
    /// including an unexplicit `unknown` keyword.
    pub fn is_inferred(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns a reference to the type's prototype, if any.
    pub fn prototype<'a>(&'a self, resolver: &'a dyn TypeResolver) -> Option<&'a TypeReference> {
        match self {
            Self::InstanceOf(instance_of) => Some(&instance_of.ty),
            Self::Object(object) => object.prototype.as_ref(),
            Self::Reference(reference) => resolver
                .resolve_and_get(reference)
                .and_then(|ty| ty.prototype(resolver)),
            _ => None,
        }
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

impl From<Literal> for TypeData {
    fn from(value: Literal) -> Self {
        Self::Literal(Box::new(value))
    }
}

/// A namespace definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Namespace(pub(super) Box<[TypeMember]>);

impl Namespace {
    pub fn from_type_members(members: Box<[TypeMember]>) -> Self {
        Self(members)
    }
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
    pub fn get_ty<'a>(&'a self, resolver: &'a mut dyn TypeResolver, index: usize) -> &'a TypeData {
        let resolved_id = if let Some(elem_type) = self.0.get(index) {
            let ty = elem_type.ty.clone();
            let id = if elem_type.is_optional {
                resolver.optional(ty)
            } else {
                resolver.register_type(TypeData::Reference(Box::new(ty)))
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

/// Members of an object definition.
// TODO: Include getters, setters and index signatures.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum TypeMember {
    CallSignature(CallSignatureTypeMember),
    Constructor(ConstructorTypeMember),
    Method(MethodTypeMember),
    Property(PropertyTypeMember),
}

impl TypeMember {
    pub fn has_name(&self, name: &str) -> bool {
        match self {
            Self::CallSignature(_) => false,
            Self::Constructor(_) => name == "constructor",
            Self::Method(member) => member.name == name,
            Self::Property(member) => member.name == name,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Self::CallSignature(_) | Self::Constructor(_) => false,
            Self::Method(member) => member.is_static,
            Self::Property(member) => member.is_static,
        }
    }

    pub fn name(&self) -> Option<Text> {
        match self {
            Self::CallSignature(_) => None,
            Self::Constructor(_) => Some(Text::Static("constructor")),
            Self::Method(member) => Some(member.name.clone()),
            Self::Property(member) => Some(member.name.clone()),
        }
    }
}

/// Defines a call signature on an object definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct CallSignatureTypeMember {
    /// Generic type parameters defined in the call signature.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameters of the signature.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the object is called.
    pub return_type: ReturnType,
}

/// Defines a call signature for an object's constructor.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct ConstructorTypeMember {
    /// Generic type parameters defined in the constructor.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameters of the constructor.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<TypeReference>,
}

/// Defines a method on an object.
#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct MethodTypeMember {
    /// Whether the function has an `async` specifier or not.
    pub is_async: bool,

    /// Generic type parameters defined in the method.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Name of the method.
    pub name: Text,

    /// Call parameters of the method.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type of the method.
    pub return_type: ReturnType,

    /// Whether the method is optional.
    pub is_optional: bool,

    /// Whether the method is static.
    pub is_static: bool,
}

impl MethodTypeMember {
    pub fn with_name(mut self, name: Text) -> Self {
        self.name = name;
        self
    }

    pub fn with_return_type(mut self, ty: TypeReference) -> Self {
        self.return_type = ReturnType::Type(ty);
        self
    }

    pub fn with_static(mut self) -> Self {
        self.is_static = true;
        self
    }
}

/// Defines an object property and its type.
#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct PropertyTypeMember {
    /// Name of the property.
    pub name: Text,

    /// Type of the property.
    pub ty: TypeReference,

    /// Whether the property is optional.
    pub is_optional: bool,

    /// Whether the property is static.
    pub is_static: bool,
}

impl PropertyTypeMember {
    pub fn with_name(mut self, name: Text) -> Self {
        self.name = name;
        self
    }

    pub fn with_type(mut self, ty: TypeReference) -> Self {
        self.ty = ty;
        self
    }
}

struct TypeMemberIterator<'a> {
    resolver: &'a dyn TypeResolver,
    seen_types: Vec<ResolvedTypeId>,
    owner: Option<TypeMemberOwner<'a>>,
    index: usize,
}

impl<'a> Iterator for TypeMemberIterator<'a> {
    type Item = &'a TypeMember;

    fn next(&mut self) -> Option<Self::Item> {
        let next_reference = match &self.owner {
            Some(TypeMemberOwner::Class(class)) => {
                match (class.members.get(self.index), class.extends.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        return Some(member);
                    }
                    (None, Some(extends)) => extends,
                    (None, None) => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            Some(TypeMemberOwner::Global) => {
                if let Some(member) = GLOBAL_TYPE_MEMBERS.get(self.index) {
                    self.index += 1;
                    return Some(member);
                } else {
                    self.owner = None;
                    return None;
                }
            }
            Some(TypeMemberOwner::InstanceOf(instance_of)) => &instance_of.ty,
            Some(TypeMemberOwner::Object(object)) => {
                match (object.members.get(self.index), object.prototype.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        return Some(member);
                    }
                    (None, Some(prototype)) => prototype,
                    (None, None) => {
                        self.owner = None;
                        return None;
                    }
                }
            }
            None => return None,
        };

        let Some(next_resolved_id) = self.resolver.resolve_reference(next_reference) else {
            self.owner = None;
            return None;
        };

        if self.seen_types.contains(&next_resolved_id) {
            self.owner = None;
            return None;
        }

        self.seen_types.push(next_resolved_id);
        self.owner = self
            .resolver
            .get_by_resolved_id(next_resolved_id)
            .and_then(TypeMemberOwner::from_type_data);
        self.index = 0;
        self.next()
    }
}

enum TypeMemberOwner<'a> {
    Class(&'a Class),
    Global,
    InstanceOf(&'a TypeInstance),
    Object(&'a Object),
}

impl<'a> TypeMemberOwner<'a> {
    fn from_type_data(type_data: &'a TypeData) -> Option<Self> {
        match type_data {
            TypeData::Class(class) => Some(Self::Class(class)),
            TypeData::Global => Some(Self::Global),
            TypeData::InstanceOf(type_instance) => Some(Self::InstanceOf(type_instance)),
            TypeData::Object(object) => Some(Self::Object(object)),
            _ => None,
        }
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

/// Reference-counted resolved path wrapped in a [Result] that contains a string
/// message if resolution failed.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResolvedPath(Arc<Result<Utf8PathBuf, String>>);

impl Deref for ResolvedPath {
    type Target = Result<Utf8PathBuf, String>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl ResolvedPath {
    pub fn new(resolved_path: Result<Utf8PathBuf, String>) -> Self {
        Self(Arc::new(resolved_path))
    }

    pub fn as_path(&self) -> Option<&Utf8Path> {
        self.as_deref().ok()
    }

    pub fn from_path(path: impl Into<Utf8PathBuf>) -> Self {
        Self::new(Ok(path.into()))
    }
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

    pub fn without_type_parameters(&self) -> Self {
        Self {
            path: self.path.clone(),
            type_parameters: [].into(),
        }
    }
}

/// A union of types.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Union(pub(super) Box<[TypeReference]>);

impl Union {
    pub fn contains(&self, ty: &TypeReference) -> bool {
        self.0.contains(ty)
    }

    pub fn with_type(&self, ty: TypeReference) -> Self {
        Self(self.0.iter().cloned().chain(Some(ty)).collect())
    }
}
