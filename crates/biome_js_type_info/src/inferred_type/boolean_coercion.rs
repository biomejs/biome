use super::{InferredType, MAX_TYPE_VARIANT_STEPS, TypeTraversalError};
use crate::TypeDb;
use crate::interned_types::{FunctionParameter, Literal, ReturnType, TypeData};
use crate::type_traversal::{DepthFirstVisitor, TraversalOutcome, VisitContext};
use std::ops::ControlFlow;

/// Distinguishes unambiguous truthiness checks from checks that conflate nullish
/// values with falsy primitives, or whose result is constant or unrestricted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BooleanCoercion {
    Safe,
    NullableBoolean,
    NullableString,
    NullableNumber,
    AlwaysTruthy,
    AlwaysNullish,
    Unrestricted,
    Mixed,
}

impl<'db> InferredType<'db> {
    /// Classifies boolean coercion, allowing non-nullable strings and numbers
    /// and nullable objects. Unresolved types, cycles, and exhausted traversal
    /// budgets return an error rather than a classification.
    pub fn boolean_coercion(self) -> Result<BooleanCoercion, TypeTraversalError> {
        let mut visitor = BooleanCoercionVisitor {
            db: self.db,
            variants: 0,
        };
        match visitor.visit(self.data, MAX_TYPE_VARIANT_STEPS) {
            TraversalOutcome::Break(error) => return Err(error),
            TraversalOutcome::LimitExceeded => return Err(TypeTraversalError::LimitExceeded),
            TraversalOutcome::Complete {
                encountered_cycle: true,
            } => return Err(TypeTraversalError::RecursiveType),
            TraversalOutcome::Complete {
                encountered_cycle: false,
            } => {}
        }
        let variants = visitor.variants;
        Ok(if variants & ANY != 0 {
            BooleanCoercion::Unrestricted
        } else if variants == NULLISH {
            BooleanCoercion::AlwaysNullish
        } else if variants == OBJECT {
            BooleanCoercion::AlwaysTruthy
        } else if variants & !(NULLISH | TRUE | FALSE) == 0 {
            if variants & NULLISH != 0 && variants & FALSE != 0 {
                BooleanCoercion::NullableBoolean
            } else {
                BooleanCoercion::Safe
            }
        } else if variants & !(NULLISH | STRING | TRUTHY_STRING) == 0 {
            if variants & NULLISH != 0 && variants & STRING != 0 {
                BooleanCoercion::NullableString
            } else {
                BooleanCoercion::Safe
            }
        } else if variants & !(NULLISH | NUMBER | TRUTHY_NUMBER) == 0 {
            if variants & NULLISH != 0 && variants & NUMBER != 0 {
                BooleanCoercion::NullableNumber
            } else {
                BooleanCoercion::Safe
            }
        } else if variants == NULLISH | OBJECT {
            BooleanCoercion::Safe
        } else {
            BooleanCoercion::Mixed
        })
    }

    /// Returns whether every union variant is an array or tuple.
    /// Unresolved types and non-array variants return `false`.
    pub fn is_array_or_tuple(self) -> bool {
        self.try_all_variants_match(|data| {
            matches!(data, TypeData::Tuple(_)) ||
            matches!(data, TypeData::InstanceOf(instance) if instance.ty(self.db).is_array_class(self.db))
        }).unwrap_or(false)
    }

    /// Returns the argument index asserted for truthiness by a function.
    /// Type predicates such as `asserts value is string` do not assert truthiness.
    pub fn truthiness_asserted_argument(self) -> Option<usize> {
        let function = self.data.callable_function(self.db)?;
        let ReturnType::Asserts(assertion) = function.return_type(self.db) else {
            return None;
        };
        if assertion.ty != TypeData::Conditional {
            return None;
        }
        function.parameters(self.db).iter().filter(|parameter| !parameter.is_this()).position(|parameter| {
            matches!(parameter, FunctionParameter::Named(named) if named.name == assertion.parameter_name)
        })
    }

    /// Returns a callable's declared return type. Type predicates are boolean;
    /// assertion functions return void. Non-callable or ambiguous types return `None`.
    pub fn callable_return_type(self) -> Option<Self> {
        let function = self.data.callable_function(self.db)?;
        let data = match function.return_type(self.db) {
            ReturnType::Type(ty) => *ty,
            ReturnType::Predicate(_) => TypeData::Boolean,
            ReturnType::Asserts(_) => TypeData::VoidKeyword,
        };
        Some(Self::new(self.db, data))
    }
}

const NULLISH: u16 = 1;
const TRUE: u16 = 1 << 1;
const FALSE: u16 = 1 << 2;
const STRING: u16 = 1 << 3;
const TRUTHY_STRING: u16 = 1 << 4;
const NUMBER: u16 = 1 << 5;
const TRUTHY_NUMBER: u16 = 1 << 6;
const OBJECT: u16 = 1 << 7;
const ANY: u16 = 1 << 8;

struct BooleanCoercionVisitor<'db> {
    db: &'db dyn TypeDb,
    variants: u16,
}

impl<'db> DepthFirstVisitor<TypeData<'db>> for BooleanCoercionVisitor<'db> {
    type Break = TypeTraversalError;

    fn enter(
        &mut self,
        data: TypeData<'db>,
        context: &mut VisitContext<'_, TypeData<'db>>,
    ) -> ControlFlow<Self::Break> {
        self.variants |= match data {
            TypeData::Unknown
            | TypeData::Local(_)
            | TypeData::TypeofExpression(_)
            | TypeData::ThisKeyword => {
                return ControlFlow::Break(TypeTraversalError::UnresolvedType);
            }
            TypeData::AnyKeyword | TypeData::UnknownKeyword => ANY,
            TypeData::NeverKeyword => 0,
            TypeData::Null | TypeData::Undefined | TypeData::VoidKeyword => NULLISH,
            TypeData::Boolean | TypeData::Conditional => TRUE | FALSE,
            TypeData::String => STRING,
            TypeData::Number | TypeData::BigInt => NUMBER,
            TypeData::Literal(literal) => match literal.literal(self.db) {
                Literal::Boolean(boolean) => {
                    if boolean.as_bool() {
                        TRUE
                    } else {
                        FALSE
                    }
                }
                Literal::String(string) => {
                    if string.as_str().is_empty() {
                        STRING
                    } else {
                        TRUTHY_STRING
                    }
                }
                Literal::Number(number) => match number.to_f64() {
                    Some(value) if value != 0.0 && !value.is_nan() => TRUTHY_NUMBER,
                    Some(_) => NUMBER,
                    None => return ControlFlow::Break(TypeTraversalError::UnresolvedType),
                },
                Literal::BigInt(_) => NUMBER,
                Literal::Template(_) => STRING,
                Literal::Object(_) | Literal::RegExp(_) => OBJECT,
            },
            TypeData::Union(union) => {
                context.extend(union.types(self.db).iter().copied());
                0
            }
            TypeData::Intersection(intersection) => {
                let primitives = intersection
                    .types(self.db)
                    .iter()
                    .copied()
                    .filter(|ty| ty.is_primitive(self.db));
                if primitives.clone().next().is_some() {
                    context.extend(primitives);
                    0
                } else if intersection
                    .types(self.db)
                    .iter()
                    .all(|ty| ty.is_object_like(self.db))
                {
                    OBJECT
                } else {
                    return ControlFlow::Break(TypeTraversalError::UnresolvedType);
                }
            }
            TypeData::Generic(generic) => {
                if let Some(constraint) = generic.constraint(self.db) {
                    context.push(constraint);
                    0
                } else {
                    ANY
                }
            }
            TypeData::GlobalType(id) => {
                context.push(crate::global_types(self.db).get(id));
                0
            }
            TypeData::TypeofType(ty) => {
                context.push(ty.ty(self.db));
                0
            }
            TypeData::TypeofValue(ty) => {
                context.push(ty.ty(self.db));
                0
            }
            TypeData::TypeOperator(ty) => {
                if ty.operator(self.db) == crate::TypeOperator::Keyof {
                    return ControlFlow::Break(TypeTraversalError::UnresolvedType);
                }
                context.push(ty.ty(self.db));
                0
            }
            TypeData::InstanceOf(instance) => {
                let target = instance.ty(self.db);
                if matches!(
                    target,
                    TypeData::Unknown | TypeData::Local(_) | TypeData::TypeofExpression(_)
                ) {
                    return ControlFlow::Break(TypeTraversalError::UnresolvedType);
                }
                if matches!(
                    target,
                    TypeData::Class(_) | TypeData::Interface(_) | TypeData::Object(_)
                ) {
                    OBJECT
                } else {
                    context.push(target);
                    0
                }
            }
            TypeData::MergedReference(_) => {
                return ControlFlow::Break(TypeTraversalError::UnresolvedType);
            }
            TypeData::Global
            | TypeData::Class(_)
            | TypeData::Constructor(_)
            | TypeData::Function(_)
            | TypeData::Interface(_)
            | TypeData::Module(_)
            | TypeData::Namespace(_)
            | TypeData::Object(_)
            | TypeData::Tuple(_)
            | TypeData::ObjectKeyword
            | TypeData::Symbol => OBJECT,
        };
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interned_types::{InternedIntersection, InternedLiteral, InternedUnion};
    use crate::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
    use biome_rowan::Text;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }
    #[salsa::db]
    impl salsa::Database for TestDb {}
    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }
    #[salsa::db]
    impl TypeDb for TestDb {}

    #[test]
    fn distinguishes_absence_from_falsy_primitives() {
        let db = TestDb::default();
        for (ty, expected) in [
            (TypeData::Boolean, BooleanCoercion::NullableBoolean),
            (TypeData::String, BooleanCoercion::NullableString),
            (TypeData::Number, BooleanCoercion::NullableNumber),
            (TypeData::BigInt, BooleanCoercion::NullableNumber),
            (TypeData::ObjectKeyword, BooleanCoercion::Safe),
            (TypeData::Symbol, BooleanCoercion::Safe),
            (TypeData::AnyKeyword, BooleanCoercion::Unrestricted),
            (TypeData::UnknownKeyword, BooleanCoercion::Unrestricted),
        ] {
            let union = TypeData::Union(InternedUnion::new(
                &db,
                Vec::from([TypeData::Null, TypeData::Undefined, ty]).into_boxed_slice(),
            ));
            assert_eq!(
                InferredType::new(&db, union).boolean_coercion(),
                Ok(expected)
            );
        }
    }

    #[test]
    fn preserves_truthy_literal_exceptions_and_mixed_unions() {
        let db = TestDb::default();
        for literal in [
            Literal::Boolean(BooleanLiteral::from(true)),
            Literal::String(StringLiteral::from("present")),
            Literal::Number(NumberLiteral::new(Text::new_static("1"))),
        ] {
            let literal = TypeData::Literal(InternedLiteral::new(&db, literal));
            let union = TypeData::Union(InternedUnion::new(
                &db,
                Vec::from([TypeData::Null, literal]).into_boxed_slice(),
            ));
            assert_eq!(
                InferredType::new(&db, union).boolean_coercion(),
                Ok(BooleanCoercion::Safe)
            );
        }
        let mixed = TypeData::Union(InternedUnion::new(
            &db,
            Vec::from([TypeData::String, TypeData::Number]).into_boxed_slice(),
        ));
        assert_eq!(
            InferredType::new(&db, mixed).boolean_coercion(),
            Ok(BooleanCoercion::Mixed)
        );
    }

    #[test]
    fn unresolved_variants_and_exhausted_walks_are_not_diagnostics() {
        let db = TestDb::default();
        for uncertain in [TypeData::Unknown, TypeData::ThisKeyword] {
            let union = TypeData::Union(InternedUnion::new(
                &db,
                Vec::from([TypeData::Null, TypeData::Number, uncertain]).into_boxed_slice(),
            ));
            assert_eq!(
                InferredType::new(&db, union).boolean_coercion(),
                Err(TypeTraversalError::UnresolvedType)
            );
            let intersection = TypeData::Intersection(InternedIntersection::new(
                &db,
                Vec::from([uncertain, TypeData::ObjectKeyword]).into_boxed_slice(),
            ));
            assert_eq!(
                InferredType::new(&db, intersection).boolean_coercion(),
                Err(TypeTraversalError::UnresolvedType)
            );
        }
        let members = (0..MAX_TYPE_VARIANT_STEPS)
            .map(|value| {
                TypeData::Literal(InternedLiteral::new(
                    &db,
                    Literal::BigInt(Text::new_owned(format!("{value}n").into())),
                ))
            })
            .collect::<Box<[_]>>();
        let union = TypeData::Union(InternedUnion::new(&db, members));
        assert_eq!(
            InferredType::new(&db, union).boolean_coercion(),
            Err(TypeTraversalError::LimitExceeded)
        );
    }
}
