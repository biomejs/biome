//! Canonical names for database-backed inferred types.
//!
//! Database-backed handles and values use the `Inferred` prefix when the raw
//! type world exports the same conceptual type. Unique names such as
//! [`ConditionalType`] remain unprefixed. Payload values use a descriptive
//! suffix when their enclosing handle needs the unsuffixed name.

pub use crate::globals_ids::GlobalTypeId;
pub use crate::interned_types::{
    AssertsReturnType as InferredAssertsReturnType, CallArgumentType as InferredCallArgumentType,
    ConditionalSubset, ConditionalType, ConstructorParameter as InferredConstructorParameter,
    FunctionParameter as InferredFunctionParameter,
    FunctionParameterBinding as InferredFunctionParameterBinding, InternedClass as InferredClass,
    InternedConstructor as InferredConstructor, InternedFunction as InferredFunction,
    InternedGenericTypeParameter as InferredGenericTypeParameter,
    InternedInterface as InferredInterface, InternedIntersection as InferredIntersection,
    InternedLiteral as InferredLiteral, InternedMergedReference as InferredMergedReference,
    InternedModule as InferredModule, InternedNamespace as InferredNamespace,
    InternedObject as InferredObject, InternedTuple as InferredTuple,
    InternedTypeInstance as InferredTypeInstance,
    InternedTypeOperatorType as InferredTypeOperatorType, InternedTypeofType as InferredTypeofType,
    InternedTypeofValue as InferredTypeofValue, InternedUnion as InferredUnion,
    Literal as InferredLiteralValue, LocalTypeHandle as InferredLocalTypeHandle,
    LocalTypeId as InferredLocalTypeId, ModuleKey as InferredModuleKey,
    NamedFunctionParameter as InferredNamedFunctionParameter,
    PatternFunctionParameter as InferredPatternFunctionParameter,
    PredicateReturnType as InferredPredicateReturnType, ReturnType as InferredReturnType,
    StructuralMapError, TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
    TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    TypeSubstitution as InferredTypeSubstitution, TypeofExpression as InferredTypeofExpression,
    well_known_symbol_name,
};
