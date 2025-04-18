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

use std::fmt::Debug;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{ops::Deref, str::FromStr, sync::Arc};

use biome_js_type_info_macros::Resolvable;
use biome_rowan::Text;

use crate::{
    Resolvable,
    globals::{ARRAY, PROMISE, WINDOW_TYPE},
};

/// Unique identifier to distinguish between identically named, complex types.
#[derive(Clone, Copy, Eq, PartialEq, Resolvable)]
pub(super) struct TypeId(u64);

// FIXME: This implementation is only necessary for test stability. Once we have
//        better snapshots for types, this won't be necessary anymore.
impl Debug for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeId").finish()
    }
}

impl TypeId {
    pub fn new() -> Self {
        static ID: AtomicU64 = AtomicU64::new(1);

        Self(ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Represents an inferred TypeScript type.
#[derive(Clone, Debug, Default, PartialEq)]
// TODO: Before moving onto full inference, we should probably use a different
//       wrapper than `Arc`. I'm thinking of creating a type called
//       `FragileArc`, which would have a single owner and any number of weak
//       pointers to it. This would allow the types of one module to be dropped
//       (for when the watcher reloads a module), after which we could simply
//       re-lookup the broken arcs. The "fragileness" would come from the fact
//       there would be a single type that can be either owner, or weak pointer,
//       so at the type level you would never know when your pointer is going to
//       break.
pub struct Type(Arc<TypeInner>);

impl Deref for Type {
    type Target = TypeInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<TypeInner> for Type {
    fn from(inner: TypeInner) -> Self {
        Self(Arc::new(inner))
    }
}

impl Type {
    pub fn array_of(ty: Self) -> Self {
        Self(Arc::new(TypeInner::Class(Box::new(ARRAY.clone())))).with_type_parameters(&[ty])
    }

    pub fn boolean() -> Self {
        Self(Arc::new(TypeInner::Boolean))
    }

    pub fn destructuring_of(ty: Self, destructure_field: DestructureField) -> Self {
        TypeInner::TypeofExpression(Box::new(TypeofExpression::Destructure(
            TypeofDestructureExpression {
                ty,
                destructure_field,
            },
        )))
        .into()
    }

    pub fn function(function: Function) -> Self {
        Self(Arc::new(TypeInner::Function(Box::new(function))))
    }

    /// Returns the `TypeInner` referenced by this type.
    ///
    /// This method follows `TypeofType` references and should be used instead
    /// of [`Self::deref()`] when you know you want to use the inner type as a
    /// type rather than an instance.
    pub fn inner_type(&self) -> &TypeInner {
        let inner = &**self;
        if let TypeInner::TypeofType(ty) = inner {
            ty.deref()
        } else {
            inner
        }
    }

    /// Returns a new instance of the given type.
    pub fn instance_of(ty: Self) -> Self {
        match ty.deref() {
            TypeInner::Class(_) => TypeInner::Object(Box::new(Object {
                prototype: Some(ty),
                members: Arc::new([]),
            }))
            .into(),
            TypeInner::Reference(_) => TypeInner::InstanceOf(Box::new(ty)).into(),
            _ => ty,
        }
    }

    /// Takes the owned [TypeInner] from a `Type`.
    ///
    /// Returns `None` if the type has other owners.
    pub fn into_inner(self) -> Option<TypeInner> {
        Arc::into_inner(self.0)
    }

    pub fn number() -> Self {
        Self(Arc::new(TypeInner::Number))
    }

    pub fn object_with_members(members: Arc<[TypeMember]>) -> Self {
        Self(Arc::new(TypeInner::Object(Box::new(Object {
            prototype: None,
            members,
        }))))
    }

    /// Returns the `Type` referenced by this type.
    ///
    /// This method follows `TypeofType` references and should be used instead
    /// of [`Self::deref()`] when you know you want to use the inner type as a
    /// type rather than an instance.
    pub fn owned_inner_type(&self) -> Self {
        if let TypeInner::TypeofType(ty) = self.deref() {
            ty.as_ref().clone()
        } else {
            self.clone()
        }
    }

    pub fn promise_of(ty: Self) -> Self {
        Self(Arc::new(TypeInner::Class(Box::new(PROMISE.clone())))).with_type_parameters(&[ty])
    }

    pub fn undefined() -> Self {
        Self(Arc::new(TypeInner::Undefined))
    }

    pub fn union_with(&self, ty: Self) -> Self {
        if let TypeInner::Union(union) = self.inner_type() {
            if union.contains(&ty) {
                self.clone()
            } else {
                Self(Arc::new(TypeInner::Union(Box::new(union.with_type(ty)))))
            }
        } else {
            Self(Arc::new(TypeInner::Union(Box::new(Union(Box::new([
                self.clone(),
                ty,
            ]))))))
        }
    }

    pub fn unknown() -> Self {
        Self(Arc::new(TypeInner::Unknown))
    }

    pub fn void() -> Self {
        Self(Arc::new(TypeInner::VoidKeyword))
    }

    pub fn window() -> Self {
        WINDOW_TYPE.clone()
    }

    pub fn with_type_parameters(&self, type_parameters: &[Self]) -> Self {
        match self.deref() {
            TypeInner::Class(class) => TypeInner::Class(Box::new(Class {
                type_parameters: class
                    .type_parameters
                    .iter()
                    .enumerate()
                    .map(|(i, param)| GenericTypeParameter {
                        name: param.name.clone(),
                        ty: type_parameters
                            .get(i)
                            .cloned()
                            .unwrap_or_else(|| param.ty.clone()),
                    })
                    .collect(),
                ..class.as_ref().clone()
            }))
            .into(),
            // TODO: Which other types do we need to handle here?
            _ => self.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub enum TypeInner {
    /// The type is unknown because inference couldn't determine a type.
    ///
    /// This is different from `UnknownKeyword`, because an explicit `unknown`
    /// should not be counted as a failure of the inference.
    #[default]
    Unknown,

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

    /// Alias to another type.
    Alias(Box<TypeAlias>),

    /// Literal value used as a type.
    Literal(Box<Literal>),

    /// Instance of another type.
    InstanceOf(Box<Type>),

    /// Reference to another type.
    Reference(Box<TypeReference>),

    /// Reference to the type of a JavaScript expression.
    TypeofExpression(Box<TypeofExpression>),

    /// Reference to another type through the `typeof` operator.
    TypeofType(Box<Type>),

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

impl TypeInner {
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

    /// Returns whether the given type has been inferred.
    ///
    /// A type is considered inferred if it is anything except `Self::Unknown`,
    /// including an unexplicit `unknown` keyword.
    pub fn is_inferred(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns whether the given type is known to reference the `Promise`
    /// class.
    pub fn is_promise(&self) -> bool {
        match self {
            Self::Class(class) => class.id == PROMISE.id,
            _ => false,
        }
    }

    /// Returns whether the given type is known to reference an instance of a
    /// `Promise`.
    pub fn is_promise_instance(&self) -> bool {
        match self {
            Self::Object(object) => object.is_promise(),
            _ => false,
        }
    }

    /// Returns whether the given type is known to reference a function that
    /// returns a `Promise`.
    pub fn is_function_that_returns_promise(&self) -> bool {
        match self {
            Self::Function(function) => function
                .return_type
                .as_type()
                .is_some_and(|ty| ty.is_promise()),
            _ => false,
        }
    }
}

/// A class definition.
#[derive(Clone, PartialEq, Resolvable)]
pub struct Class {
    pub(super) id: TypeId,

    /// Name of the class, if specified in the definition.
    pub name: Option<Text>,

    /// The class's type parameters.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Type of another class being extended by this one.
    pub extends: Option<Type>,

    /// Class members.
    pub members: Arc<[TypeMember]>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("type_parameters", &self.type_parameters)
            .field("extends", &self.extends)
            .finish()
    }
}

impl Class {
    /// Iterates all member fields, including those belonging to extended
    /// classes.
    ///
    /// Note that members which are inherited and overridden may appear multiple
    /// times, but the member that is closest to the subclass is guaranteed to
    /// come first.
    pub fn all_members(&self) -> impl Iterator<Item = &TypeMember> {
        TypeMemberIterator {
            owner: Some(TypeMemberOwner::Class(self)),
            index: 0,
        }
    }
}

/// A constructor definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Constructor {
    /// Generic type parameters used in the call signature.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Call parameter of the constructor.
    pub parameters: Arc<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<Type>,
}

/// A function definition.
#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct Function {
    /// Whether the function has an `async` specifier or not.
    pub is_async: bool,

    /// Generic type parameters defined in the function signature.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Name of the function, if specified in the definition.
    pub name: Option<Text>,

    /// Call parameters of the function.
    pub parameters: Arc<[FunctionParameter]>,

    /// The function's return type.
    pub return_type: ReturnType,
}

impl Function {
    pub fn with_return_type(self, ty: Type) -> Self {
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
    pub ty: Type,

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
    pub ty: Type,
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
    pub ty: Type,
}

/// The intersection between other types.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Intersection(pub(super) Box<[Type]>);

/// Literal value used as a type.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum Literal {
    BigInt(Text),
    Boolean(Text),
    Null,
    Number(Text),
    Object(ObjectLiteral),
    RegExp(Text),
    String(Text),
    Template(Text),
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
    pub prototype: Option<Type>,

    /// The object's own members.
    pub members: Arc<[TypeMember]>,
}

impl Object {
    /// Iterates all member fields, including those belonging to the prototype
    /// chain.
    ///
    /// Note that members which are inherited and overridden may appear multiple
    /// times, but the member that is closest to the own object in the prototype
    /// chain is guaranteed to come first.
    pub fn all_members(&self) -> impl Iterator<Item = &TypeMember> {
        TypeMemberIterator {
            owner: Some(TypeMemberOwner::Object(self)),
            index: 0,
        }
    }

    /// Returns a parent class of this object, if it matches the given `class`.
    ///
    /// The returned [Class] will match the given `class`, but may still have
    /// different type arguments.
    pub fn find_parent_class(&self, class: &Class) -> Option<&Class> {
        let mut prototype = self.prototype.as_ref();
        while let Some(proto) = prototype {
            match proto.as_class() {
                Some(proto_class) if proto_class.id == class.id => return Some(proto_class),
                _ => {}
            }

            prototype = proto.as_object().and_then(|p| p.prototype.as_ref())
        }

        None
    }

    /// Returns the promised type, if this object is an instance of a `Promise`.
    pub fn find_promise_type(&self) -> Option<Type> {
        self.find_parent_class(&PROMISE)
            .map(|class| class.type_parameters[0].ty.clone())
    }

    /// Returns whether this object has the given `class` in its prototype
    /// chain.
    pub fn is_instance_of(&self, class: &Class) -> bool {
        self.find_parent_class(class).is_some()
    }

    /// Returns whether this object is an instance of a `Promise`.
    pub fn is_promise(&self) -> bool {
        self.is_instance_of(&PROMISE)
    }
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
    pub fn get_ty(&self, index: usize) -> Type {
        if let Some(elem_type) = self.0.get(index) {
            let ty = &elem_type.ty;
            if elem_type.is_optional {
                ty.union_with(Type::undefined())
            } else {
                ty.clone()
            }
        } else {
            self.0
                .last()
                .filter(|last| last.is_rest)
                .map(|last| last.ty.union_with(Type::undefined()))
                .unwrap_or_default()
        }
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
    pub ty: Type,

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

    pub fn to_type(&self, object: &Type) -> Type {
        match self {
            Self::CallSignature(member) => TypeInner::Function(Box::new(Function {
                is_async: false,
                type_parameters: member.type_parameters.clone(),
                name: None,
                parameters: member.parameters.clone(),
                return_type: member.return_type.clone(),
            }))
            .into(),
            Self::Constructor(member) => {
                member.return_type.clone().unwrap_or_else(|| object.clone())
            }
            Self::Method(member) => {
                let ty: Type = TypeInner::Function(Box::new(Function {
                    is_async: member.is_async,
                    type_parameters: member.type_parameters.clone(),
                    name: Some(member.name.clone()),
                    parameters: member.parameters.clone(),
                    return_type: member.return_type.clone(),
                }))
                .into();
                if member.is_optional {
                    ty.union_with(Type::undefined())
                } else {
                    ty
                }
            }
            Self::Property(member) => {
                if member.is_optional {
                    member.ty.union_with(Type::undefined())
                } else {
                    member.ty.clone()
                }
            }
        }
    }
}

/// Defines a call signature on an object definition.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct CallSignatureTypeMember {
    /// Generic type parameters defined in the call signature.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Call parameters of the signature.
    pub parameters: Arc<[FunctionParameter]>,

    /// Return type when the object is called.
    pub return_type: ReturnType,
}

/// Defines a call signature for an object's constructor.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct ConstructorTypeMember {
    /// Generic type parameters defined in the constructor.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Call parameters of the constructor.
    pub parameters: Arc<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<Type>,
}

/// Defines a method on an object.
#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct MethodTypeMember {
    /// Whether the function has an `async` specifier or not.
    pub is_async: bool,

    /// Generic type parameters defined in the method.
    pub type_parameters: Arc<[GenericTypeParameter]>,

    /// Name of the method.
    pub name: Text,

    /// Call parameters of the method.
    pub parameters: Arc<[FunctionParameter]>,

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

    pub fn with_return_type(mut self, ty: Type) -> Self {
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
    pub ty: Type,

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

    pub fn with_type(mut self, ty: Type) -> Self {
        self.ty = ty;
        self
    }
}

struct TypeMemberIterator<'a> {
    owner: Option<TypeMemberOwner<'a>>,
    index: usize,
}

impl<'a> Iterator for TypeMemberIterator<'a> {
    type Item = &'a TypeMember;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.owner {
            Some(TypeMemberOwner::Class(class)) => {
                match (class.members.get(self.index), class.extends.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        Some(member)
                    }
                    (None, Some(extends)) => {
                        self.owner = extends.into();
                        self.index = 0;
                        self.next()
                    }
                    (None, None) => {
                        self.owner = None;
                        None
                    }
                }
            }
            Some(TypeMemberOwner::Object(object)) => {
                match (object.members.get(self.index), object.prototype.as_ref()) {
                    (Some(member), _) => {
                        self.index += 1;
                        Some(member)
                    }
                    (None, Some(prototype)) => {
                        self.owner = prototype.into();
                        self.index = 0;
                        self.next()
                    }
                    (None, None) => {
                        self.owner = None;
                        None
                    }
                }
            }
            None => None,
        }
    }
}

enum TypeMemberOwner<'a> {
    Class(&'a Class),
    Object(&'a Object),
}

impl<'a> From<&'a Type> for Option<TypeMemberOwner<'a>> {
    fn from(ty: &'a Type) -> Self {
        match ty.inner_type() {
            TypeInner::Class(class) => Some(TypeMemberOwner::Class(class)),
            TypeInner::Object(object) => Some(TypeMemberOwner::Object(object)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum ReturnType {
    Type(Type),
    Predicate(PredicateReturnType),
    Asserts(AssertsReturnType),
}

impl Default for ReturnType {
    fn default() -> Self {
        Self::Type(Type::unknown())
    }
}

impl ReturnType {
    pub fn as_type(&self) -> Option<&Type> {
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
    pub ty: Type,
}

/// Defines the function to which it applies to be an assertion that asserts
/// one of its arguments to be of a given type.
///
/// Assertion functions throw at runtime if the type assertion fails.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct AssertsReturnType {
    pub parameter_name: Text,
    pub ty: Type,
}

/// Alias to another type.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeAlias {
    /// The type being aliased.
    pub ty: Type,

    /// Generic type parameters that can be passed on the alias itself.
    pub type_parameters: Arc<[GenericTypeParameter]>,
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
    pub left: Type,
    pub right: Type,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofAwaitExpression {
    pub argument: Type,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofCallExpression {
    pub callee: Type,
    pub arguments: Arc<[CallArgumentType]>,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofDestructureExpression {
    /// The type being destructured.
    pub ty: Type,

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
    pub callee: Type,
    pub arguments: Arc<[CallArgumentType]>,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub enum CallArgumentType {
    Argument(Type),
    Spread(Type),
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofStaticMemberExpression {
    pub object: Type,
    pub member: Text,
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeofThisOrSuperExpression {
    /// Type from which the `this` or `super` expression should be resolved.
    pub parent: Type,
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
    pub ty: Type,
}

impl Resolvable for TypeofValue {
    fn needs_resolving(&self, resolver: &dyn crate::TypeResolver, _stack: &[&TypeInner]) -> bool {
        !self.ty.is_inferred() && resolver.resolve_type_of(&self.identifier).is_some()
    }

    fn resolved(&self, resolver: &dyn crate::TypeResolver, stack: &[&TypeInner]) -> Self {
        let ty = match self.ty.is_inferred() {
            true => self.ty.clone(),
            false => resolver
                .resolve_type_of(&self.identifier)
                .map_or_else(Type::unknown, |ty| ty.resolved(resolver, stack)),
        };

        Self {
            identifier: self.identifier.clone(),
            ty,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct TypeOperatorType {
    pub operator: TypeOperator,
    pub ty: Type,
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
#[derive(Clone, Debug, PartialEq)]
pub struct TypeReference {
    /// Qualifier of the type being referenced.
    pub qualifier: TypeReferenceQualifier,

    /// The resolved type.
    pub ty: Type,

    /// Generic type parameters specified in the reference.
    pub type_parameters: Arc<[Type]>,
}

impl Resolvable for TypeReference {
    fn needs_resolving(&self, resolver: &dyn crate::TypeResolver, stack: &[&TypeInner]) -> bool {
        (!self.ty.is_inferred() && resolver.resolve_qualifier(&self.qualifier).is_some())
            || self
                .type_parameters
                .iter()
                .any(|param| param.needs_resolving(resolver, stack))
    }

    fn resolved(&self, resolver: &dyn crate::TypeResolver, stack: &[&TypeInner]) -> Self {
        let ty = match self.ty.is_inferred() {
            true => self.ty.clone(),
            false => resolver
                .resolve_qualifier(&self.qualifier)
                .map_or_else(Type::unknown, |ty| ty.resolved(resolver, stack)),
        };

        Self {
            qualifier: self.qualifier.clone(),
            ty,
            type_parameters: self
                .type_parameters
                .iter()
                .map(|param| param.resolved(resolver, stack))
                .collect(),
        }
    }
}

/// Path of identifiers to the type to a referenced type.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeReferenceQualifier(pub(super) Box<[Text]>);

impl TypeReferenceQualifier {
    /// Checks whether this type qualifier references an `Array` type.
    ///
    /// This method simply checks whether the reference is for a literal
    /// `Array`, without considering whether another symbol named `Array` is
    /// in scope. It can be used _after_ type resolution has failed to find a
    /// `Array` symbol in scope, but should not be used _instead of_ such type
    /// resolution.
    pub fn is_array(&self) -> bool {
        self.0.len() == 1 && self.0[0] == "Array"
    }

    /// Checks whether this type qualifier references a `Promise` type.
    ///
    /// This method simply checks whether the reference is for a literal
    /// `Promise`, without considering whether another symbol named `Promise` is
    /// in scope. It can be used _after_ type resolution has failed to find a
    /// `Promise` symbol in scope, but should not be used _instead of_ such type
    /// resolution.
    pub fn is_promise(&self) -> bool {
        self.0.len() == 1 && self.0[0] == "Promise"
    }

    pub fn parts(&self) -> &[Text] {
        &self.0
    }
}

/// A union of types.
#[derive(Clone, Debug, PartialEq, Resolvable)]
pub struct Union(pub(super) Box<[Type]>);

impl Union {
    pub fn contains(&self, ty: &Type) -> bool {
        self.0.contains(ty)
    }

    pub fn with_type(&self, ty: Type) -> Self {
        Self(self.0.iter().cloned().chain(Some(ty)).collect())
    }
}
