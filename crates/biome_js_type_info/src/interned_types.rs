//! Salsa-backed type data for the resolved type-inference world.
//!
//! The existing `type_data` module remains the raw, collector-side representation
//! for now. This module introduces the interned resolved representation that
//! later phases will wire into module inference.

use biome_rowan::Text;

use crate::{
    ScopeId,
    globals_ids::{
        GLOBAL_BOOLEAN_ID, GLOBAL_CONDITIONAL_ID, GLOBAL_GLOBAL_ID, GLOBAL_NUMBER_ID,
        GLOBAL_STRING_ID, GLOBAL_UNDEFINED_ID, GLOBAL_UNKNOWN_ID, GLOBAL_VOID_ID,
    },
    literal::{BooleanLiteral, NumberLiteral, RegexpLiteral, StringLiteral},
    type_data as raw,
};

pub type RawTypeData = raw::TypeData;

#[salsa::db]
pub trait TypeDb: biome_db::Db {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct DivergentType {
    pub id: salsa::Id,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeData<'db> {
    Unknown,
    Divergent(DivergentType),
    Global,
    BigInt,
    Boolean,
    Null,
    Number,
    String,
    Symbol,
    Undefined,
    Conditional,
    Class(InternedClass<'db>),
    Constructor(InternedConstructor<'db>),
    Function(InternedFunction<'db>),
    Interface(InternedInterface<'db>),
    Module(InternedModule<'db>),
    Namespace(InternedNamespace<'db>),
    Object(InternedObject<'db>),
    Tuple(InternedTuple<'db>),
    Generic(InternedGenericTypeParameter<'db>),
    Intersection(InternedIntersection<'db>),
    Union(InternedUnion<'db>),
    TypeOperator(InternedTypeOperatorType<'db>),
    Literal(InternedLiteral<'db>),
    InstanceOf(InternedTypeInstance<'db>),
    MergedReference(InternedMergedReference<'db>),
    TypeofExpression(InternedTypeofExpression<'db>),
    TypeofType(InternedTypeofType<'db>),
    TypeofValue(InternedTypeofValue<'db>),
    AnyKeyword,
    NeverKeyword,
    ObjectKeyword,
    ThisKeyword,
    UnknownKeyword,
    VoidKeyword,
}

impl<'db> Default for TypeData<'db> {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'db> TypeData<'db> {
    pub fn divergent(id: salsa::Id) -> Self {
        Self::Divergent(DivergentType { id })
    }

    pub fn from_raw_lossy(db: &'db dyn TypeDb, raw: &RawTypeData) -> Self {
        match raw {
            raw::TypeData::Unknown => Self::Unknown,
            raw::TypeData::Global => Self::Global,
            raw::TypeData::BigInt => Self::BigInt,
            raw::TypeData::Boolean => Self::Boolean,
            raw::TypeData::Null => Self::Null,
            raw::TypeData::Number => Self::Number,
            raw::TypeData::String => Self::String,
            raw::TypeData::Symbol => Self::Symbol,
            raw::TypeData::Undefined => Self::Undefined,
            raw::TypeData::Conditional => Self::Conditional,
            raw::TypeData::ImportNamespace(_) => Self::Unknown,
            raw::TypeData::Class(class) => Self::Class(InternedClass::new(
                db,
                convert_references(db, &class.type_parameters),
                class.extends.as_ref().map(|ty| Self::from_raw_reference_lossy(db, ty)),
                convert_references(db, &class.implements),
                convert_type_members(db, &class.members),
                class.name.clone(),
            )),
            raw::TypeData::Constructor(constructor) => Self::Constructor(InternedConstructor::new(
                db,
                convert_references(db, &constructor.type_parameters),
                convert_constructor_parameters(db, &constructor.parameters),
                constructor
                    .return_type
                    .as_ref()
                    .map(|ty| Self::from_raw_reference_lossy(db, ty)),
            )),
            raw::TypeData::Function(function) => Self::Function(InternedFunction::new(
                db,
                convert_references(db, &function.type_parameters),
                convert_function_parameters(db, &function.parameters),
                convert_return_type(db, &function.return_type),
                function.is_async,
                function.name.clone(),
            )),
            raw::TypeData::Interface(interface) => Self::Interface(InternedInterface::new(
                db,
                convert_references(db, &interface.type_parameters),
                convert_references(db, &interface.extends),
                convert_type_members(db, &interface.members),
                interface.name.clone(),
            )),
            raw::TypeData::Module(module) => Self::Module(InternedModule::new(
                db,
                convert_type_members(db, &module.members),
                module.name.clone(),
            )),
            raw::TypeData::Namespace(namespace) => Self::Namespace(InternedNamespace::new(
                db,
                convert_type_members(db, &namespace.members),
                namespace.path.clone(),
            )),
            raw::TypeData::Object(object) => Self::Object(InternedObject::new(
                db,
                object
                    .prototype
                    .as_ref()
                    .map(|ty| Self::from_raw_reference_lossy(db, ty)),
                convert_type_members(db, &object.members),
            )),
            raw::TypeData::Tuple(tuple) => Self::Tuple(InternedTuple::new(
                db,
                tuple
                    .elements()
                    .iter()
                    .map(|element| TupleElementType {
                        ty: Self::from_raw_reference_lossy(db, &element.ty),
                        name: element.name.clone(),
                        is_optional: element.is_optional,
                        is_rest: element.is_rest,
                    })
                    .collect::<Box<[_]>>(),
            )),
            raw::TypeData::Generic(generic) => Self::Generic(InternedGenericTypeParameter::new(
                db,
                generic
                    .constraint
                    .is_known()
                    .then(|| Self::from_raw_reference_lossy(db, &generic.constraint)),
                generic
                    .default
                    .is_known()
                    .then(|| Self::from_raw_reference_lossy(db, &generic.default)),
                generic.name.clone(),
            )),
            raw::TypeData::Intersection(intersection) => Self::Intersection(InternedIntersection::new(
                db,
                convert_references(db, intersection.types()),
            )),
            raw::TypeData::Union(union) => {
                Self::Union(InternedUnion::new(db, convert_references(db, union.types())))
            }
            raw::TypeData::TypeOperator(type_operator) => {
                Self::TypeOperator(InternedTypeOperatorType::new(
                    db,
                    Self::from_raw_reference_lossy(db, &type_operator.ty),
                    type_operator.operator,
                ))
            }
            raw::TypeData::Literal(literal) => Self::Literal(InternedLiteral::new(
                db,
                convert_literal(db, literal.as_ref()),
            )),
            raw::TypeData::InstanceOf(instance) => Self::InstanceOf(InternedTypeInstance::new(
                db,
                Self::from_raw_reference_lossy(db, &instance.ty),
                convert_references(db, &instance.type_parameters),
            )),
            raw::TypeData::Reference(reference) => Self::from_raw_reference_lossy(db, reference),
            raw::TypeData::MergedReference(reference) => {
                Self::MergedReference(InternedMergedReference::new(
                    db,
                    reference
                        .ty
                        .as_ref()
                        .map(|ty| Self::from_raw_reference_lossy(db, ty)),
                    reference
                        .value_ty
                        .as_ref()
                        .map(|ty| Self::from_raw_reference_lossy(db, ty)),
                    reference
                        .namespace_ty
                        .as_ref()
                        .map(|ty| Self::from_raw_reference_lossy(db, ty)),
                ))
            }
            raw::TypeData::TypeofExpression(expression) => {
                Self::TypeofExpression(InternedTypeofExpression::new(
                    db,
                    convert_typeof_expression(db, expression.as_ref()),
                ))
            }
            raw::TypeData::TypeofType(ty) => Self::TypeofType(InternedTypeofType::new(
                db,
                Self::from_raw_reference_lossy(db, ty),
            )),
            raw::TypeData::TypeofValue(value) => Self::TypeofValue(InternedTypeofValue::new(
                db,
                Self::from_raw_reference_lossy(db, &value.ty),
                value.identifier.clone(),
                value.scope_id,
            )),
            raw::TypeData::AnyKeyword => Self::AnyKeyword,
            raw::TypeData::NeverKeyword => Self::NeverKeyword,
            raw::TypeData::ObjectKeyword => Self::ObjectKeyword,
            raw::TypeData::ThisKeyword => Self::ThisKeyword,
            raw::TypeData::UnknownKeyword => Self::UnknownKeyword,
            raw::TypeData::VoidKeyword => Self::VoidKeyword,
        }
    }

    pub fn from_raw_reference_lossy(db: &'db dyn TypeDb, reference: &raw::TypeReference) -> Self {
        match reference {
            raw::TypeReference::Resolved(id) if *id == GLOBAL_UNKNOWN_ID => Self::Unknown,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_UNDEFINED_ID => Self::Undefined,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_VOID_ID => Self::VoidKeyword,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_CONDITIONAL_ID => Self::Conditional,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_NUMBER_ID => Self::Number,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_STRING_ID => Self::String,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_GLOBAL_ID => Self::Global,
            raw::TypeReference::Resolved(id) if *id == GLOBAL_BOOLEAN_ID => Self::Boolean,
            raw::TypeReference::Resolved(_) => Self::Unknown,
            raw::TypeReference::Qualifier(qualifier) => qualifier
                .type_parameters
                .iter()
                .find(|param| param.is_known())
                .map_or(Self::Unknown, |param| Self::from_raw_reference_lossy(db, param)),
            raw::TypeReference::Import(_) => Self::Unknown,
        }
    }

    pub fn to_raw_lossy(self, db: &'db dyn TypeDb) -> RawTypeData {
        match self {
            Self::Unknown | Self::Divergent(_) => raw::TypeData::Unknown,
            Self::Global => raw::TypeData::Global,
            Self::BigInt => raw::TypeData::BigInt,
            Self::Boolean => raw::TypeData::Boolean,
            Self::Null => raw::TypeData::Null,
            Self::Number => raw::TypeData::Number,
            Self::String => raw::TypeData::String,
            Self::Symbol => raw::TypeData::Symbol,
            Self::Undefined => raw::TypeData::Undefined,
            Self::Conditional => raw::TypeData::Conditional,
            Self::Class(class) => raw::TypeData::Class(Box::new(raw::Class {
                name: class.name(db).clone(),
                type_parameters: raw_references_from_types(db, class.type_parameters(db)),
                extends: self.raw_reference_from_option(db, class.extends(db)),
                implements: raw_references_from_types(db, class.implements(db)),
                members: raw_type_members_from_types(db, class.members(db)),
            })),
            Self::Constructor(constructor) => raw::TypeData::Constructor(Box::new(raw::Constructor {
                type_parameters: raw_references_from_types(db, constructor.type_parameters(db)),
                parameters: raw_constructor_parameters_from_types(db, constructor.parameters(db)),
                return_type: self.raw_reference_from_option(db, constructor.return_type(db)),
            })),
            Self::Function(function) => raw::TypeData::Function(Box::new(raw::Function {
                is_async: function.is_async(db),
                type_parameters: raw_references_from_types(db, function.type_parameters(db)),
                name: function.name(db).clone(),
                parameters: raw_function_parameters_from_types(db, function.parameters(db)),
                return_type: raw_return_type_from_type(db, function.return_type(db)),
            })),
            Self::Interface(interface) => raw::TypeData::Interface(Box::new(raw::Interface {
                name: interface.name(db).clone(),
                type_parameters: raw_references_from_types(db, interface.type_parameters(db)),
                extends: raw_references_from_types(db, interface.extends(db)),
                members: raw_type_members_from_types(db, interface.members(db)),
            })),
            Self::Module(module) => raw::TypeData::Module(Box::new(raw::Module {
                name: module.name(db).clone(),
                members: raw_type_members_from_types(db, module.members(db)),
            })),
            Self::Namespace(namespace) => raw::TypeData::Namespace(Box::new(raw::Namespace {
                path: namespace.path(db).clone(),
                members: raw_type_members_from_types(db, namespace.members(db)),
            })),
            Self::Object(object) => raw::TypeData::Object(Box::new(raw::Object {
                prototype: self.raw_reference_from_option(db, object.prototype(db)),
                members: raw_type_members_from_types(db, object.members(db)),
            })),
            Self::Tuple(tuple) => raw::TypeData::Tuple(Box::new(raw::Tuple(
                tuple
                    .elements(db)
                    .iter()
                    .map(|element| raw::TupleElementType {
                        ty: element.ty.to_raw_reference_lossy(),
                        name: element.name.clone(),
                        is_optional: element.is_optional,
                        is_rest: element.is_rest,
                    })
                    .collect(),
            ))),
            Self::Generic(generic) => raw::TypeData::Generic(Box::new(raw::GenericTypeParameter {
                name: generic.name(db).clone(),
                constraint: generic
                    .constraint(db)
                    .map_or_else(raw::TypeReference::unknown, TypeData::to_raw_reference_lossy),
                default: generic
                    .default(db)
                    .map_or_else(raw::TypeReference::unknown, TypeData::to_raw_reference_lossy),
            })),
            Self::Intersection(intersection) => raw::TypeData::Intersection(Box::new(
                raw::Intersection(raw_references_from_types(db, intersection.types(db))),
            )),
            Self::Union(union) => raw::TypeData::Union(Box::new(raw::Union(raw_references_from_types(
                db,
                union.types(db),
            )))),
            Self::TypeOperator(type_operator) => {
                raw::TypeData::TypeOperator(Box::new(raw::TypeOperatorType {
                    operator: type_operator.operator(db),
                    ty: type_operator.ty(db).to_raw_reference_lossy(),
                }))
            }
            Self::Literal(literal) => raw::TypeData::Literal(Box::new(raw_literal_from_type(
                db,
                literal.literal(db),
            ))),
            Self::InstanceOf(instance) => raw::TypeData::InstanceOf(Box::new(raw::TypeInstance {
                ty: instance.ty(db).to_raw_reference_lossy(),
                type_parameters: raw_references_from_types(db, instance.type_parameters(db)),
            })),
            Self::MergedReference(reference) => {
                raw::TypeData::MergedReference(Box::new(raw::MergedReference {
                    ty: self.raw_reference_from_option(db, reference.ty(db)),
                    value_ty: self.raw_reference_from_option(db, reference.value_ty(db)),
                    namespace_ty: self.raw_reference_from_option(db, reference.namespace_ty(db)),
                }))
            }
            Self::TypeofExpression(expression) => raw::TypeData::TypeofExpression(Box::new(
                raw_typeof_expression_from_type(db, expression.expression(db)),
            )),
            Self::TypeofType(ty) => raw::TypeData::TypeofType(Box::new(ty.ty(db).to_raw_reference_lossy())),
            Self::TypeofValue(value) => raw::TypeData::TypeofValue(Box::new(raw::TypeofValue {
                identifier: value.identifier(db).clone(),
                ty: value.ty(db).to_raw_reference_lossy(),
                scope_id: value.scope_id(db),
            })),
            Self::AnyKeyword => raw::TypeData::AnyKeyword,
            Self::NeverKeyword => raw::TypeData::NeverKeyword,
            Self::ObjectKeyword => raw::TypeData::ObjectKeyword,
            Self::ThisKeyword => raw::TypeData::ThisKeyword,
            Self::UnknownKeyword => raw::TypeData::UnknownKeyword,
            Self::VoidKeyword => raw::TypeData::VoidKeyword,
        }
    }

    pub fn to_raw_reference_lossy(self) -> raw::TypeReference {
        match self {
            Self::Unknown | Self::Divergent(_) => raw::TypeReference::Resolved(GLOBAL_UNKNOWN_ID),
            Self::Global => raw::TypeReference::Resolved(GLOBAL_GLOBAL_ID),
            Self::Boolean => raw::TypeReference::Resolved(GLOBAL_BOOLEAN_ID),
            Self::Number => raw::TypeReference::Resolved(GLOBAL_NUMBER_ID),
            Self::String => raw::TypeReference::Resolved(GLOBAL_STRING_ID),
            Self::Undefined => raw::TypeReference::Resolved(GLOBAL_UNDEFINED_ID),
            Self::Conditional => raw::TypeReference::Resolved(GLOBAL_CONDITIONAL_ID),
            Self::VoidKeyword => raw::TypeReference::Resolved(GLOBAL_VOID_ID),
            _ => raw::TypeReference::Resolved(GLOBAL_UNKNOWN_ID),
        }
    }

    fn raw_reference_from_option(
        self,
        _db: &'db dyn TypeDb,
        ty: Option<TypeData<'db>>,
    ) -> Option<raw::TypeReference> {
        ty.map(TypeData::to_raw_reference_lossy)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum Literal<'db> {
    BigInt(Text),
    Boolean(BooleanLiteral),
    Number(NumberLiteral),
    Object(Box<[TypeMember<'db>]>),
    RegExp(RegexpLiteral),
    String(StringLiteral),
    Template(Text),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum ReturnType<'db> {
    Type(TypeData<'db>),
    Predicate(PredicateReturnType<'db>),
    Asserts(AssertsReturnType<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct PredicateReturnType<'db> {
    pub parameter_name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct AssertsReturnType<'db> {
    pub parameter_name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct ConstructorParameter<'db> {
    pub parameter: FunctionParameter<'db>,
    pub accessibility: Option<raw::TypeMemberAccessibility>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum FunctionParameter<'db> {
    Named(NamedFunctionParameter<'db>),
    Pattern(PatternFunctionParameter<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct NamedFunctionParameter<'db> {
    pub name: Text,
    pub ty: TypeData<'db>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct PatternFunctionParameter<'db> {
    pub bindings: Box<[FunctionParameterBinding<'db>]>,
    pub ty: TypeData<'db>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct FunctionParameterBinding<'db> {
    pub name: Text,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TupleElementType<'db> {
    pub ty: TypeData<'db>,
    pub name: Option<Text>,
    pub is_optional: bool,
    pub is_rest: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeMember<'db> {
    pub kind: TypeMemberKind<'db>,
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeMemberKind<'db> {
    CallSignature,
    ComputedValue(TypeData<'db>),
    ConstAssertedCallSignature,
    ConstAssertedComputedValue(TypeData<'db>),
    ConstAssertedConstructor,
    ConstAssertedGetter(Text),
    ConstAssertedIndexSignature(TypeData<'db>),
    ConstAssertedNamed(Text),
    ConstAssertedNamedOptional(Text),
    ConstAssertedNamedStatic(Text),
    Constructor,
    Getter(Text),
    IndexSignature(TypeData<'db>),
    Named(Text),
    NamedOptional(Text),
    NamedStatic(Text),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum TypeofExpression<'db> {
    Addition(TypeofAdditionExpression<'db>),
    Await(TypeofAwaitExpression<'db>),
    BitwiseNot(TypeofBitwiseNotExpression<'db>),
    Call(TypeofCallExpression<'db>),
    Conditional(TypeofConditionalExpression<'db>),
    Destructure(TypeofDestructureExpression<'db>),
    Index(TypeofIndexExpression<'db>),
    IterableValueOf(TypeofIterableValueOfExpression<'db>),
    LogicalAnd(TypeofLogicalAndExpression<'db>),
    LogicalOr(TypeofLogicalOrExpression<'db>),
    New(TypeofNewExpression<'db>),
    NullishCoalescing(TypeofNullishCoalescingExpression<'db>),
    StaticMember(TypeofStaticMemberExpression<'db>),
    Super(TypeofThisOrSuperExpression<'db>),
    This(TypeofThisOrSuperExpression<'db>),
    Typeof(TypeofTypeofExpression<'db>),
    UnaryMinus(TypeofUnaryMinusExpression<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofAdditionExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofAwaitExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofBitwiseNotExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofCallExpression<'db> {
    pub callee: TypeData<'db>,
    pub arguments: Box<[CallArgumentType<'db>]>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofConditionalExpression<'db> {
    pub test: TypeData<'db>,
    pub consequent: TypeData<'db>,
    pub alternate: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofDestructureExpression<'db> {
    pub ty: TypeData<'db>,
    pub destructure_field: raw::DestructureField,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofIterableValueOfExpression<'db> {
    pub ty: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofLogicalAndExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofLogicalOrExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofNewExpression<'db> {
    pub callee: TypeData<'db>,
    pub arguments: Box<[CallArgumentType<'db>]>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub enum CallArgumentType<'db> {
    Argument(TypeData<'db>),
    Spread(TypeData<'db>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofIndexExpression<'db> {
    pub object: TypeData<'db>,
    pub index: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofNullishCoalescingExpression<'db> {
    pub left: TypeData<'db>,
    pub right: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofStaticMemberExpression<'db> {
    pub object: TypeData<'db>,
    pub member: Text,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofThisOrSuperExpression<'db> {
    pub parent: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofTypeofExpression<'db> {
    pub argument: TypeData<'db>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct TypeofUnaryMinusExpression<'db> {
    pub argument: TypeData<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedFunction<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub parameters: Box<[FunctionParameter<'db>]>,
    #[returns(ref)]
    pub return_type: ReturnType<'db>,
    pub is_async: bool,
    #[returns(ref)]
    pub name: Option<Text>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedClass<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    pub extends: Option<TypeData<'db>>,
    #[returns(ref)]
    pub implements: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Option<Text>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedConstructor<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub parameters: Box<[ConstructorParameter<'db>]>,
    pub return_type: Option<TypeData<'db>>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedInterface<'db> {
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub extends: Box<[TypeData<'db>]>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedObject<'db> {
    pub prototype: Option<TypeData<'db>>,
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedUnion<'db> {
    #[returns(ref)]
    pub types: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedIntersection<'db> {
    #[returns(ref)]
    pub types: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTuple<'db> {
    #[returns(ref)]
    pub elements: Box<[TupleElementType<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedModule<'db> {
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedNamespace<'db> {
    #[returns(ref)]
    pub members: Box<[TypeMember<'db>]>,
    #[returns(ref)]
    pub path: raw::Path,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedLiteral<'db> {
    #[returns(ref)]
    pub literal: Literal<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeInstance<'db> {
    pub ty: TypeData<'db>,
    #[returns(ref)]
    pub type_parameters: Box<[TypeData<'db>]>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedMergedReference<'db> {
    pub ty: Option<TypeData<'db>>,
    pub value_ty: Option<TypeData<'db>>,
    pub namespace_ty: Option<TypeData<'db>>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedGenericTypeParameter<'db> {
    pub constraint: Option<TypeData<'db>>,
    pub default: Option<TypeData<'db>>,
    #[returns(ref)]
    pub name: Text,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeOperatorType<'db> {
    pub ty: TypeData<'db>,
    pub operator: raw::TypeOperator,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofExpression<'db> {
    #[returns(ref)]
    pub expression: TypeofExpression<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofType<'db> {
    pub ty: TypeData<'db>,
}

#[salsa::interned]
#[derive(Debug)]
pub struct InternedTypeofValue<'db> {
    pub ty: TypeData<'db>,
    #[returns(ref)]
    pub identifier: Text,
    pub scope_id: Option<ScopeId>,
}

fn convert_references<'db>(
    db: &'db dyn TypeDb,
    references: &[raw::TypeReference],
) -> Box<[TypeData<'db>]> {
    references
        .iter()
        .map(|reference| TypeData::from_raw_reference_lossy(db, reference))
        .collect()
}

fn convert_type_members<'db>(
    db: &'db dyn TypeDb,
    members: &[raw::TypeMember],
) -> Box<[TypeMember<'db>]> {
    members
        .iter()
        .map(|member| TypeMember {
            kind: convert_type_member_kind(db, &member.kind),
            ty: TypeData::from_raw_reference_lossy(db, &member.ty),
        })
        .collect()
}

fn convert_type_member_kind<'db>(
    db: &'db dyn TypeDb,
    kind: &raw::TypeMemberKind,
) -> TypeMemberKind<'db> {
    match kind {
        raw::TypeMemberKind::CallSignature => TypeMemberKind::CallSignature,
        raw::TypeMemberKind::ComputedValue(ty) => {
            TypeMemberKind::ComputedValue(TypeData::from_raw_reference_lossy(db, ty))
        }
        raw::TypeMemberKind::ConstAssertedCallSignature => TypeMemberKind::ConstAssertedCallSignature,
        raw::TypeMemberKind::ConstAssertedComputedValue(ty) => {
            TypeMemberKind::ConstAssertedComputedValue(TypeData::from_raw_reference_lossy(db, ty))
        }
        raw::TypeMemberKind::ConstAssertedConstructor => TypeMemberKind::ConstAssertedConstructor,
        raw::TypeMemberKind::ConstAssertedGetter(name) => {
            TypeMemberKind::ConstAssertedGetter(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedIndexSignature(ty) => {
            TypeMemberKind::ConstAssertedIndexSignature(TypeData::from_raw_reference_lossy(db, ty))
        }
        raw::TypeMemberKind::ConstAssertedNamed(name) => {
            TypeMemberKind::ConstAssertedNamed(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedNamedOptional(name) => {
            TypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        raw::TypeMemberKind::ConstAssertedNamedStatic(name) => {
            TypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        raw::TypeMemberKind::Constructor => TypeMemberKind::Constructor,
        raw::TypeMemberKind::Getter(name) => TypeMemberKind::Getter(name.clone()),
        raw::TypeMemberKind::IndexSignature(ty) => {
            TypeMemberKind::IndexSignature(TypeData::from_raw_reference_lossy(db, ty))
        }
        raw::TypeMemberKind::Named(name) => TypeMemberKind::Named(name.clone()),
        raw::TypeMemberKind::NamedOptional(name) => TypeMemberKind::NamedOptional(name.clone()),
        raw::TypeMemberKind::NamedStatic(name) => TypeMemberKind::NamedStatic(name.clone()),
    }
}

fn convert_constructor_parameters<'db>(
    db: &'db dyn TypeDb,
    parameters: &[raw::ConstructorParameter],
) -> Box<[ConstructorParameter<'db>]> {
    parameters
        .iter()
        .map(|parameter| ConstructorParameter {
            parameter: convert_function_parameter(db, &parameter.parameter),
            accessibility: parameter.accessibility,
        })
        .collect()
}

fn convert_function_parameters<'db>(
    db: &'db dyn TypeDb,
    parameters: &[raw::FunctionParameter],
) -> Box<[FunctionParameter<'db>]> {
    parameters
        .iter()
        .map(|parameter| convert_function_parameter(db, parameter))
        .collect()
}

fn convert_function_parameter<'db>(
    db: &'db dyn TypeDb,
    parameter: &raw::FunctionParameter,
) -> FunctionParameter<'db> {
    match parameter {
        raw::FunctionParameter::Named(named) => FunctionParameter::Named(NamedFunctionParameter {
            name: named.name.clone(),
            ty: TypeData::from_raw_reference_lossy(db, &named.ty),
            is_optional: named.is_optional,
            is_rest: named.is_rest,
        }),
        raw::FunctionParameter::Pattern(pattern) => {
            FunctionParameter::Pattern(PatternFunctionParameter {
                bindings: pattern
                    .bindings
                    .iter()
                    .map(|binding| FunctionParameterBinding {
                        name: binding.name.clone(),
                        ty: TypeData::from_raw_reference_lossy(db, &binding.ty),
                    })
                    .collect(),
                ty: TypeData::from_raw_reference_lossy(db, &pattern.ty),
                is_optional: pattern.is_optional,
                is_rest: pattern.is_rest,
            })
        }
    }
}

fn convert_return_type<'db>(db: &'db dyn TypeDb, return_type: &raw::ReturnType) -> ReturnType<'db> {
    match return_type {
        raw::ReturnType::Type(ty) => ReturnType::Type(TypeData::from_raw_reference_lossy(db, ty)),
        raw::ReturnType::Predicate(predicate) => ReturnType::Predicate(PredicateReturnType {
            parameter_name: predicate.parameter_name.clone(),
            ty: TypeData::from_raw_reference_lossy(db, &predicate.ty),
        }),
        raw::ReturnType::Asserts(asserts) => ReturnType::Asserts(AssertsReturnType {
            parameter_name: asserts.parameter_name.clone(),
            ty: TypeData::from_raw_reference_lossy(db, &asserts.ty),
        }),
    }
}

fn convert_literal<'db>(db: &'db dyn TypeDb, literal: &raw::Literal) -> Literal<'db> {
    match literal {
        raw::Literal::BigInt(text) => Literal::BigInt(text.clone()),
        raw::Literal::Boolean(boolean) => Literal::Boolean(boolean.clone()),
        raw::Literal::Number(number) => Literal::Number(number.clone()),
        raw::Literal::Object(object) => Literal::Object(convert_type_members(db, object.members())),
        raw::Literal::RegExp(regexp) => Literal::RegExp(regexp.clone()),
        raw::Literal::String(string) => Literal::String(string.clone()),
        raw::Literal::Template(text) => Literal::Template(text.clone()),
    }
}

fn convert_typeof_expression<'db>(
    db: &'db dyn TypeDb,
    expression: &raw::TypeofExpression,
) -> TypeofExpression<'db> {
    match expression {
        raw::TypeofExpression::Addition(expression) => {
            TypeofExpression::Addition(TypeofAdditionExpression {
                left: TypeData::from_raw_reference_lossy(db, &expression.left),
                right: TypeData::from_raw_reference_lossy(db, &expression.right),
            })
        }
        raw::TypeofExpression::Await(expression) => TypeofExpression::Await(TypeofAwaitExpression {
            argument: TypeData::from_raw_reference_lossy(db, &expression.argument),
        }),
        raw::TypeofExpression::BitwiseNot(expression) => {
            TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                argument: TypeData::from_raw_reference_lossy(db, &expression.argument),
            })
        }
        raw::TypeofExpression::Call(expression) => TypeofExpression::Call(TypeofCallExpression {
            callee: TypeData::from_raw_reference_lossy(db, &expression.callee),
            arguments: convert_call_arguments(db, &expression.arguments),
        }),
        raw::TypeofExpression::Conditional(expression) => {
            TypeofExpression::Conditional(TypeofConditionalExpression {
                test: TypeData::from_raw_reference_lossy(db, &expression.test),
                consequent: TypeData::from_raw_reference_lossy(db, &expression.consequent),
                alternate: TypeData::from_raw_reference_lossy(db, &expression.alternate),
            })
        }
        raw::TypeofExpression::Destructure(expression) => {
            TypeofExpression::Destructure(TypeofDestructureExpression {
                ty: TypeData::from_raw_reference_lossy(db, &expression.ty),
                destructure_field: expression.destructure_field.clone(),
            })
        }
        raw::TypeofExpression::Index(expression) => TypeofExpression::Index(TypeofIndexExpression {
            object: TypeData::from_raw_reference_lossy(db, &expression.object),
            index: expression.index,
        }),
        raw::TypeofExpression::IterableValueOf(expression) => {
            TypeofExpression::IterableValueOf(TypeofIterableValueOfExpression {
                ty: TypeData::from_raw_reference_lossy(db, &expression.ty),
            })
        }
        raw::TypeofExpression::LogicalAnd(expression) => {
            TypeofExpression::LogicalAnd(TypeofLogicalAndExpression {
                left: TypeData::from_raw_reference_lossy(db, &expression.left),
                right: TypeData::from_raw_reference_lossy(db, &expression.right),
            })
        }
        raw::TypeofExpression::LogicalOr(expression) => {
            TypeofExpression::LogicalOr(TypeofLogicalOrExpression {
                left: TypeData::from_raw_reference_lossy(db, &expression.left),
                right: TypeData::from_raw_reference_lossy(db, &expression.right),
            })
        }
        raw::TypeofExpression::New(expression) => TypeofExpression::New(TypeofNewExpression {
            callee: TypeData::from_raw_reference_lossy(db, &expression.callee),
            arguments: convert_call_arguments(db, &expression.arguments),
        }),
        raw::TypeofExpression::NullishCoalescing(expression) => {
            TypeofExpression::NullishCoalescing(TypeofNullishCoalescingExpression {
                left: TypeData::from_raw_reference_lossy(db, &expression.left),
                right: TypeData::from_raw_reference_lossy(db, &expression.right),
            })
        }
        raw::TypeofExpression::StaticMember(expression) => {
            TypeofExpression::StaticMember(TypeofStaticMemberExpression {
                object: TypeData::from_raw_reference_lossy(db, &expression.object),
                member: expression.member.clone(),
            })
        }
        raw::TypeofExpression::Super(expression) => TypeofExpression::Super(TypeofThisOrSuperExpression {
            parent: TypeData::from_raw_reference_lossy(db, &expression.parent),
        }),
        raw::TypeofExpression::This(expression) => TypeofExpression::This(TypeofThisOrSuperExpression {
            parent: TypeData::from_raw_reference_lossy(db, &expression.parent),
        }),
        raw::TypeofExpression::Typeof(expression) => {
            TypeofExpression::Typeof(TypeofTypeofExpression {
                argument: TypeData::from_raw_reference_lossy(db, &expression.argument),
            })
        }
        raw::TypeofExpression::UnaryMinus(expression) => {
            TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                argument: TypeData::from_raw_reference_lossy(db, &expression.argument),
            })
        }
    }
}

fn convert_call_arguments<'db>(
    db: &'db dyn TypeDb,
    arguments: &[raw::CallArgumentType],
) -> Box<[CallArgumentType<'db>]> {
    arguments
        .iter()
        .map(|argument| match argument {
            raw::CallArgumentType::Argument(ty) => {
                CallArgumentType::Argument(TypeData::from_raw_reference_lossy(db, ty))
            }
            raw::CallArgumentType::Spread(ty) => {
                CallArgumentType::Spread(TypeData::from_raw_reference_lossy(db, ty))
            }
        })
        .collect()
}

fn raw_references_from_types<'db>(
    _db: &'db dyn TypeDb,
    types: &[TypeData<'db>],
) -> Box<[raw::TypeReference]> {
    types
        .iter()
        .map(|ty| ty.to_raw_reference_lossy())
        .collect()
}

fn raw_type_members_from_types<'db>(
    db: &'db dyn TypeDb,
    members: &[TypeMember<'db>],
) -> Box<[raw::TypeMember]> {
    members
        .iter()
        .map(|member| raw::TypeMember {
            kind: raw_type_member_kind_from_type(db, &member.kind),
            ty: member.ty.to_raw_reference_lossy(),
        })
        .collect()
}

fn raw_type_member_kind_from_type<'db>(
    _db: &'db dyn TypeDb,
    kind: &TypeMemberKind<'db>,
) -> raw::TypeMemberKind {
    match kind {
        TypeMemberKind::CallSignature => raw::TypeMemberKind::CallSignature,
        TypeMemberKind::ComputedValue(ty) => {
            raw::TypeMemberKind::ComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedCallSignature => raw::TypeMemberKind::ConstAssertedCallSignature,
        TypeMemberKind::ConstAssertedComputedValue(ty) => {
            raw::TypeMemberKind::ConstAssertedComputedValue(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedConstructor => raw::TypeMemberKind::ConstAssertedConstructor,
        TypeMemberKind::ConstAssertedGetter(name) => {
            raw::TypeMemberKind::ConstAssertedGetter(name.clone())
        }
        TypeMemberKind::ConstAssertedIndexSignature(ty) => {
            raw::TypeMemberKind::ConstAssertedIndexSignature(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::ConstAssertedNamed(name) => raw::TypeMemberKind::ConstAssertedNamed(name.clone()),
        TypeMemberKind::ConstAssertedNamedOptional(name) => {
            raw::TypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        TypeMemberKind::ConstAssertedNamedStatic(name) => {
            raw::TypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        TypeMemberKind::Constructor => raw::TypeMemberKind::Constructor,
        TypeMemberKind::Getter(name) => raw::TypeMemberKind::Getter(name.clone()),
        TypeMemberKind::IndexSignature(ty) => {
            raw::TypeMemberKind::IndexSignature(ty.to_raw_reference_lossy())
        }
        TypeMemberKind::Named(name) => raw::TypeMemberKind::Named(name.clone()),
        TypeMemberKind::NamedOptional(name) => raw::TypeMemberKind::NamedOptional(name.clone()),
        TypeMemberKind::NamedStatic(name) => raw::TypeMemberKind::NamedStatic(name.clone()),
    }
}

fn raw_constructor_parameters_from_types<'db>(
    db: &'db dyn TypeDb,
    parameters: &[ConstructorParameter<'db>],
) -> Box<[raw::ConstructorParameter]> {
    parameters
        .iter()
        .map(|parameter| raw::ConstructorParameter {
            parameter: raw_function_parameter_from_type(db, &parameter.parameter),
            accessibility: parameter.accessibility,
        })
        .collect()
}

fn raw_function_parameters_from_types<'db>(
    db: &'db dyn TypeDb,
    parameters: &[FunctionParameter<'db>],
) -> Box<[raw::FunctionParameter]> {
    parameters
        .iter()
        .map(|parameter| raw_function_parameter_from_type(db, parameter))
        .collect()
}

fn raw_function_parameter_from_type<'db>(
    _db: &'db dyn TypeDb,
    parameter: &FunctionParameter<'db>,
) -> raw::FunctionParameter {
    match parameter {
        FunctionParameter::Named(named) => raw::FunctionParameter::Named(raw::NamedFunctionParameter {
            name: named.name.clone(),
            ty: named.ty.to_raw_reference_lossy(),
            is_optional: named.is_optional,
            is_rest: named.is_rest,
        }),
        FunctionParameter::Pattern(pattern) => {
            raw::FunctionParameter::Pattern(raw::PatternFunctionParameter {
                bindings: pattern
                    .bindings
                    .iter()
                    .map(|binding| raw::FunctionParameterBinding {
                        name: binding.name.clone(),
                        ty: binding.ty.to_raw_reference_lossy(),
                    })
                    .collect(),
                ty: pattern.ty.to_raw_reference_lossy(),
                is_optional: pattern.is_optional,
                is_rest: pattern.is_rest,
            })
        }
    }
}

fn raw_return_type_from_type<'db>(
    _db: &'db dyn TypeDb,
    return_type: &ReturnType<'db>,
) -> raw::ReturnType {
    match return_type {
        ReturnType::Type(ty) => raw::ReturnType::Type(ty.to_raw_reference_lossy()),
        ReturnType::Predicate(predicate) => raw::ReturnType::Predicate(Box::new(
            raw::PredicateReturnType {
                parameter_name: predicate.parameter_name.clone(),
                ty: predicate.ty.to_raw_reference_lossy(),
            },
        )),
        ReturnType::Asserts(asserts) => raw::ReturnType::Asserts(Box::new(raw::AssertsReturnType {
            parameter_name: asserts.parameter_name.clone(),
            ty: asserts.ty.to_raw_reference_lossy(),
        })),
    }
}

fn raw_literal_from_type<'db>(db: &'db dyn TypeDb, literal: &Literal<'db>) -> raw::Literal {
    match literal {
        Literal::BigInt(text) => raw::Literal::BigInt(text.clone()),
        Literal::Boolean(boolean) => raw::Literal::Boolean(boolean.clone()),
        Literal::Number(number) => raw::Literal::Number(number.clone()),
        Literal::Object(members) => raw::Literal::Object(raw::ObjectLiteral(
            raw_type_members_from_types(db, members),
        )),
        Literal::RegExp(regexp) => raw::Literal::RegExp(regexp.clone()),
        Literal::String(string) => raw::Literal::String(string.clone()),
        Literal::Template(text) => raw::Literal::Template(text.clone()),
    }
}

fn raw_typeof_expression_from_type<'db>(
    db: &'db dyn TypeDb,
    expression: &TypeofExpression<'db>,
) -> raw::TypeofExpression {
    match expression {
        TypeofExpression::Addition(expression) => raw::TypeofExpression::Addition(
            raw::TypeofAdditionExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            },
        ),
        TypeofExpression::Await(expression) => raw::TypeofExpression::Await(raw::TypeofAwaitExpression {
            argument: expression.argument.to_raw_reference_lossy(),
        }),
        TypeofExpression::BitwiseNot(expression) => {
            raw::TypeofExpression::BitwiseNot(raw::TypeofBitwiseNotExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Call(expression) => raw::TypeofExpression::Call(raw::TypeofCallExpression {
            callee: expression.callee.to_raw_reference_lossy(),
            arguments: raw_call_arguments_from_types(db, &expression.arguments),
        }),
        TypeofExpression::Conditional(expression) => {
            raw::TypeofExpression::Conditional(raw::TypeofConditionalExpression {
                test: expression.test.to_raw_reference_lossy(),
                consequent: expression.consequent.to_raw_reference_lossy(),
                alternate: expression.alternate.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::Destructure(expression) => {
            raw::TypeofExpression::Destructure(raw::TypeofDestructureExpression {
                ty: expression.ty.to_raw_reference_lossy(),
                destructure_field: expression.destructure_field.clone(),
            })
        }
        TypeofExpression::Index(expression) => raw::TypeofExpression::Index(raw::TypeofIndexExpression {
            object: expression.object.to_raw_reference_lossy(),
            index: expression.index,
        }),
        TypeofExpression::IterableValueOf(expression) => {
            raw::TypeofExpression::IterableValueOf(raw::TypeofIterableValueOfExpression {
                ty: expression.ty.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::LogicalAnd(expression) => {
            raw::TypeofExpression::LogicalAnd(raw::TypeofLogicalAndExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::LogicalOr(expression) => {
            raw::TypeofExpression::LogicalOr(raw::TypeofLogicalOrExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::New(expression) => raw::TypeofExpression::New(raw::TypeofNewExpression {
            callee: expression.callee.to_raw_reference_lossy(),
            arguments: raw_call_arguments_from_types(db, &expression.arguments),
        }),
        TypeofExpression::NullishCoalescing(expression) => raw::TypeofExpression::NullishCoalescing(
            raw::TypeofNullishCoalescingExpression {
                left: expression.left.to_raw_reference_lossy(),
                right: expression.right.to_raw_reference_lossy(),
            },
        ),
        TypeofExpression::StaticMember(expression) => {
            raw::TypeofExpression::StaticMember(raw::TypeofStaticMemberExpression {
                object: expression.object.to_raw_reference_lossy(),
                member: expression.member.clone(),
            })
        }
        TypeofExpression::Super(expression) => raw::TypeofExpression::Super(raw::TypeofThisOrSuperExpression {
            parent: expression.parent.to_raw_reference_lossy(),
        }),
        TypeofExpression::This(expression) => raw::TypeofExpression::This(raw::TypeofThisOrSuperExpression {
            parent: expression.parent.to_raw_reference_lossy(),
        }),
        TypeofExpression::Typeof(expression) => {
            raw::TypeofExpression::Typeof(raw::TypeofTypeofExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
        TypeofExpression::UnaryMinus(expression) => {
            raw::TypeofExpression::UnaryMinus(raw::TypeofUnaryMinusExpression {
                argument: expression.argument.to_raw_reference_lossy(),
            })
        }
    }
}

fn raw_call_arguments_from_types<'db>(
    _db: &'db dyn TypeDb,
    arguments: &[CallArgumentType<'db>],
) -> Box<[raw::CallArgumentType]> {
    arguments
        .iter()
        .map(|argument| match argument {
            CallArgumentType::Argument(ty) => raw::CallArgumentType::Argument(ty.to_raw_reference_lossy()),
            CallArgumentType::Spread(ty) => raw::CallArgumentType::Spread(ty.to_raw_reference_lossy()),
        })
        .collect()
}
