//! Canonical names for database-backed inferred types.
//!
//! Names use the `Inferred` prefix when the raw type world exports the same
//! conceptual type. Unique names such as [`ConditionalType`] remain unprefixed.

pub use crate::globals_ids::GlobalTypeId;
pub use crate::interned_types::{
    AssertsReturnType as InferredAssertsReturnType, CallArgumentType as InferredCallArgumentType,
    ConditionalSubset, ConditionalType, ConstructorParameter as InferredConstructorParameter,
    FunctionParameter as InferredFunctionParameter,
    FunctionParameterBinding as InferredFunctionParameterBinding, InternedClass as InferredClass,
    InternedConstructor as InferredConstructor, InternedFunction as InferredFunction,
    InternedGenericTypeParameter as InferredInternedGenericTypeParameter,
    InternedInterface as InferredInterface, InternedLiteral as InferredInternedLiteral,
    InternedMergedReference as InferredMergedReference, InternedModule as InferredModule,
    InternedNamespace as InferredNamespace, InternedObject as InferredObject,
    InternedTuple as InferredTuple, InternedTypeOperatorType as InferredTypeOperatorType,
    InternedTypeofValue, InternedUnion as InferredUnion, Literal as InferredLiteral,
    LocalTypeHandle as InferredLocalTypeHandle, LocalTypeId as InferredLocalTypeId,
    ModuleKey as InferredModuleKey, NamedFunctionParameter as InferredNamedFunctionParameter,
    PatternFunctionParameter as InferredPatternFunctionParameter,
    PredicateReturnType as InferredPredicateReturnType, ReturnType as InferredReturnType,
    StructuralMapError, TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
    TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    TypeSubstitution as InferredTypeSubstitution, TypeofExpression as InferredTypeofExpression,
    well_known_symbol_name,
};
