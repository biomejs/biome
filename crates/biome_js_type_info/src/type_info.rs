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

use std::str::FromStr;

use biome_rowan::Text;
use static_assertions::assert_eq_size;

/// Represents an inferred TypeScript type.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Type {
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
    Array(Box<Type>),
    Class(Box<Class>),
    Constructor(Box<Constructor>),
    Function(Box<Function>),
    Namespace(Box<Namespace>),
    Object(Box<Object>),
    Promise(Box<Type>),
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

    /// Reference to another type.
    Reference(Box<TypeReference>),

    /// Reference to another type through the `typeof` operator.
    TypeofType(Box<TypeReference>),

    /// Reference to the type of a named JavaScript value.
    ///
    /// We explicitly do not allow full expressions to be used as values,
    /// meaning our inference needs to break down expressions into parts before
    /// deciding the values to reference.
    TypeofValue(Box<Text>),

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

// `Type` should not be bigger than 16 bytes.
assert_eq_size!(Type, [usize; 2]);

impl Type {
    /// Returns whether the given type has been inferred.
    ///
    /// A type is considered inferred if it is anything except `Self::Unknown`,
    /// including an unexplicit `unknown` keyword.
    pub fn is_inferred(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns whether the given type is known to reference a `Promise`.
    pub fn is_promise(&self) -> bool {
        matches!(self, Self::Promise(_))
    }

    /// Returns whether the given type is known to reference a function that
    /// returns a `Promise`.
    pub fn is_function_that_returns_promise(&self) -> bool {
        match self {
            Self::Function(function) => {
                function.return_type.as_type().is_some_and(Type::is_promise)
            }
            _ => false,
        }
    }
}

/// A class definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    /// Name of the class, if specified in the definition.
    pub name: Option<Text>,

    /// Class members.
    pub members: Box<[ClassMember]>,
}

/// Members of a class definition.
// TODO: Include getters, setters and index signatures.
#[derive(Clone, Debug, PartialEq)]
pub enum ClassMember {
    Constructor(ConstructorTypeMember),
    Method(MethodTypeMember),
    Property(PropertyTypeMember),
}

/// A constructor definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Constructor {
    /// Generic type parameters used in the call signature.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameter of the constructor.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<Type>,
}

/// A function definition.
#[derive(Clone, Debug, PartialEq)]
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

/// Definition of a function argument.
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionParameter {
    /// Name of the argument, if specified in the definition.
    pub name: Option<Text>,

    /// Type of the argument.
    pub ty: Type,

    /// Whether the argument is optional or not.
    pub is_optional: bool,

    /// Whether this is a rest argument (`...`) or not.
    pub is_rest: bool,
}

/// Definition of a generic type parameter.
// TODO: Include modifiers and constraints.
#[derive(Clone, Debug, PartialEq)]
pub struct GenericTypeParameter {
    /// Name of the type parameter.
    pub name: Text,

    /// Default type to use if the type parameter is not specified.
    pub default_ty: Type,
}

/// The intersection between other types.
#[derive(Clone, Debug, PartialEq)]
pub struct Intersection(pub Box<[Type]>);

/// Literal value used as a type.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct Namespace(pub Box<[TypeMember]>);

/// An object definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Object(pub Box<[TypeMember]>);

impl Object {
    pub fn members(&self) -> &[TypeMember] {
        &self.0
    }
}

/// Object literal used as a type.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectLiteral(pub Box<[TypeMember]>);

/// Tuple type.
///
/// Tuples in TypeScript are created using `Array`s of a fixed size.
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple(pub Box<[TupleElementType]>);

/// An individual element within a tuple.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum TypeMember {
    CallSignature(CallSignatureTypeMember),
    Constructor(ConstructorTypeMember),
    Method(MethodTypeMember),
    Property(PropertyTypeMember),
}

/// Defines a call signature on an object definition.
#[derive(Clone, Debug, PartialEq)]
pub struct CallSignatureTypeMember {
    /// Generic type parameters defined in the call signature.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameters of the signature.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the object is called.
    pub return_type: ReturnType,
}

/// Defines a call signature for an object's constructor.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructorTypeMember {
    /// Generic type parameters defined in the constructor.
    pub type_parameters: Box<[GenericTypeParameter]>,

    /// Call parameters of the constructor.
    pub parameters: Box<[FunctionParameter]>,

    /// Return type when the constructor is called.
    pub return_type: Option<Type>,
}

/// Defines a method on an object.
// TODO: Include modifiers.
#[derive(Clone, Debug, PartialEq)]
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
}

/// Defines an object property and its type.
// TODO: Include modifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyTypeMember {
    pub name: Text,
    pub ty: Type,
    pub is_optional: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    Type(Type),
    Predicate(PredicateReturnType),
    Asserts(AssertsReturnType),
}

impl Default for ReturnType {
    fn default() -> Self {
        Self::Type(Type::Unknown)
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
#[derive(Clone, Debug, PartialEq)]
pub struct PredicateReturnType {
    pub parameter_name: Text,
    pub ty: Type,
}

/// Defines the function to which it applies to be an assertion that asserts
/// one of its arguments to be of a given type.
///
/// Assertion functions throw at runtime if the type assertion fails.
#[derive(Clone, Debug, PartialEq)]
pub struct AssertsReturnType {
    pub parameter_name: Text,
    pub ty: Type,
}

/// Alias to another type.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeAlias {
    /// The type being aliased.
    pub ty: Type,

    /// Generic type parameters that can be passed on the alias itself.
    pub type_parameters: Box<[GenericTypeParameter]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeOperatorType {
    pub operator: TypeOperator,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq)]
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
/// This definition may be imported from another module.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeReference {
    /// Qualifier of the type being referenced.
    pub qualifier: TypeReferenceQualifier,

    /// Generic type parameters specified in the reference.
    pub type_parameters: Box<[Type]>,
}

/// Path of identifiers to the type to a referenced type.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeReferenceQualifier(pub Box<[Text]>);

impl TypeReferenceQualifier {
    /// HACK: This method simply checks whether the reference is for a literal
    ///       `Promise`, without considering whether another symbol named
    ///       `Promise` is in scope. It's a shortcut for getting
    ///       `noFloatingPromises` to work, but we'd like to do a proper lookup
    ///       later.
    pub fn is_promise(&self) -> bool {
        self.0.len() == 1 && self.0[0] == "Promise"
    }
}

/// A union of types.
#[derive(Clone, Debug, PartialEq)]
pub struct Union(pub Box<[Type]>);

#[cfg(test)]
#[path = "type_info.tests.rs"]
mod tests;
