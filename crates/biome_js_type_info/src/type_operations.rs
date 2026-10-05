//! Evaluates types that select object keys or array elements.
//!
//! The operands must already be resolved and normalized: references point to
//! their types, and wrappers such as `typeof` have been removed. An incomplete
//! result stays unknown so a lint rule cannot mistake a partial list for every
//! possible key or element. This does not depend on which branches execute.

use crate::interned_types::{
    InternedIndexedAccessType, InternedLiteral, InternedMappedType, Literal, MappedTypeKeys,
    ProjectedKey, ReturnType, TypeData, TypeDb, TypeMember, TypeMemberKind,
    requires_escape_handling,
};
use crate::type_transform::TypeSubstitution;
use crate::{MappedTypeModifier, TypeOperator};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_PROJECTION_MEMBERS: usize = 1024;

/// Returns every member of an object, or `None` if the list may be incomplete.
///
/// The list is incomplete for objects with uncollected or inherited members.
/// An object with more than 1024 members also returns `None`, rather than its
/// first 1024 members, because callers treat the result as the object's whole
/// member list: `keyof` would omit keys, and a mapped type would omit
/// properties.
fn complete_object_members<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
) -> Option<&'db [TypeMember<'db>]> {
    let TypeData::Object(object) = ty else {
        return None;
    };
    if object.has_unknown_members(db) || object.prototype(db).is_some() {
        return None;
    }
    let members = object.members(db);
    (members.len() <= MAX_PROJECTION_MEMBERS).then_some(members)
}

/// Builds a union of the names of an object's properties.
///
/// For example, the keys of this object form the type `"A" | "B" | "C"`:
///
/// ```ts
/// const values = { A: 1, B: 2, C: 3 } as const;
/// type Letter = keyof typeof values;
/// ```
///
/// Returns `None` if the object has uncollected or inherited members, numeric or
/// computed keys, names requiring escape handling, or more than 1024 members.
/// Reporting only the keys we know would incorrectly describe the whole object.
pub(crate) fn keyof<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<TypeData<'db>> {
    let members = complete_object_members(db, ty)?;
    let mut keys = Vec::with_capacity(members.len());
    for member in members {
        match member.projected_key() {
            ProjectedKey::Name(name) => keys.push(string_literal(db, name)),
            ProjectedKey::NoKey => {}
            ProjectedKey::Unsupported => return None,
        }
    }
    Some(TypeData::union_from_types(db, keys))
}

fn string_literal<'db>(db: &'db dyn TypeDb, name: &Text) -> TypeData<'db> {
    TypeData::Literal(InternedLiteral::new(
        db,
        Literal::String(name.clone().into()),
    ))
}

/// Builds the object described by a mapped type whose parts are normalized.
///
/// For example, this mapped type produces
/// `{ A?: number | undefined; B?: string | undefined }` because the keys of
/// `Source` are known, `Source[K]` selects each property type, and the `?`
/// modifier applies to every property:
///
/// ```ts
/// type Source = { A: number; B: string };
/// type Optional = { [K in keyof Source]?: Source[K] };
/// ```
///
/// The result depends on how the keys are written:
///
/// - For keys written as `keyof T`, each property of `T` becomes a property of
///   the result. `T[K]` becomes the type of that property, and the result's
///   property is optional when the property of `T` is optional.
/// - For other keys, such as `"a" | "b"`, the keys must be a union of string
///   literals. Each literal becomes a required property.
///
/// In both cases, each key replaces `K` everywhere in the property type. The
/// `?` modifier then makes every property optional, as `Partial<T>` does, and
/// `-?` makes every property required, as `Required<T>` does. The `readonly`
/// modifier has no effect on the result, just as `Readonly<T>` has no effect.
///
/// Returns `None` if the keys are not resolved, refer to an object with
/// uncollected or inherited members, contain numeric or escaped names, number
/// more than 1024, or if substituting into one property takes more than 1024
/// steps. Such a mapped type must stay unevaluated: a generic one can still be
/// instantiated later, and a partial result would incorrectly describe the
/// whole object.
pub(crate) fn mapped_type<'db>(
    db: &'db dyn TypeDb,
    mapped: InternedMappedType<'db>,
) -> Option<TypeData<'db>> {
    let type_parameter = *mapped.type_parameter(db);
    let parameter_reference = TypeData::instance_of(db, type_parameter, Box::default());
    let property_ty = mapped.ty(db);
    let substitute = |substitutions: &[TypeSubstitution<'db>]| {
        property_ty
            .substitute_types(db, substitutions)
            .into_result()
            .ok()
    };

    let mut members = Vec::new();
    match mapped.keys(db) {
        MappedTypeKeys::Keyof(source) => {
            let property_access = TypeData::IndexedAccess(InternedIndexedAccessType::new(
                db,
                source,
                parameter_reference,
            ));
            for member in complete_object_members(db, source)? {
                let name = match member.projected_key() {
                    ProjectedKey::Name(name) => name,
                    ProjectedKey::NoKey => continue,
                    ProjectedKey::Unsupported => return None,
                };
                // Both substitutions apply in one pass. The pass reaches
                // `T[K]` before its index, so `T[K]` becomes this member's type
                // instead of an indexed access with a literal key.
                let ty = substitute(&[
                    TypeSubstitution {
                        generic: property_access,
                        replacement: member_value_type(db, member),
                    },
                    TypeSubstitution {
                        generic: parameter_reference,
                        replacement: string_literal(db, name),
                    },
                ])?;
                // Reading an optional property may yield `undefined` even when
                // the property type does not come from `T[K]`.
                let ty = if member.kind.is_optional() {
                    TypeData::union_from_types(db, Vec::from([ty, TypeData::Undefined]))
                } else {
                    ty
                };
                members.push(TypeMember {
                    kind: mapped_member_kind(name, member.kind.is_optional()),
                    ty,
                });
            }
        }
        MappedTypeKeys::Type(keys) => {
            if keys.union_iterator(db).len() > MAX_PROJECTION_MEMBERS {
                return None;
            }
            // `never` is the empty union, so the mapped type has no members.
            let keys = (keys != TypeData::NeverKeyword).then_some(keys);
            for key in keys.into_iter().flat_map(|keys| keys.union_iterator(db)) {
                let TypeData::Literal(literal) = key else {
                    return None;
                };
                let Literal::String(name) = literal.literal(db) else {
                    return None;
                };
                if requires_escape_handling(name.as_str()) {
                    return None;
                }
                let ty = substitute(&[TypeSubstitution {
                    generic: parameter_reference,
                    replacement: key,
                }])?;
                members.push(TypeMember {
                    kind: mapped_member_kind(&Text::from(name.clone()), false),
                    ty,
                });
            }
        }
    }

    // The modifiers behave like `Partial<T>` and `Required<T>`: adding `?` also
    // adds `undefined` to each property type, and removing it strips
    // `undefined` from the properties that were optional.
    Some(match mapped.optional_modifier(db) {
        Some(MappedTypeModifier::Add) => TypeData::with_all_optional_members(db, members),
        Some(MappedTypeModifier::Remove) => TypeData::with_all_required_members(db, members),
        None => TypeData::object_from_members(db, members),
    })
}

/// Returns the kind of a property that a mapped type produces.
///
/// Mapped properties are always plain named properties. The `ConstAsserted*`
/// kinds record that a member came from an `as const` expression, and a mapped
/// type is a type declaration rather than such an expression. Callers pass the
/// source member's [`TypeMemberKind::is_optional`], which is also true for
/// `ConstAssertedNamedOptional`, so optionality carries over from both kinds.
fn mapped_member_kind<'db>(name: &Text, is_optional: bool) -> TypeMemberKind<'db> {
    if is_optional {
        TypeMemberKind::NamedOptional(name.clone())
    } else {
        TypeMemberKind::Named(name.clone())
    }
}

/// Returns the type of reading `member`, which is what `T[K]` evaluates to
/// when `K` names that member.
///
/// A getter is stored as a function type, so reading it produces the
/// function's return type rather than the function. In this example,
/// `Source["size"]` is `number`, not `() => number`:
///
/// ```ts
/// interface Source {
///     get size(): number;
/// }
/// ```
fn member_value_type<'db>(db: &'db dyn TypeDb, member: &TypeMember<'db>) -> TypeData<'db> {
    if matches!(
        member.kind,
        TypeMemberKind::Getter(_) | TypeMemberKind::ConstAssertedGetter(_)
    ) && let TypeData::Function(function) = member.ty
        && let ReturnType::Type(return_ty) = function.return_type(db)
    {
        *return_ty
    } else {
        member.ty
    }
}

impl TypeData<'_> {
    /// Returns whether [`indexed_access`] can never evaluate `T[self]`,
    /// whatever `T` is.
    ///
    /// Only `T[number]` is evaluated. A literal index, as in `T[0]` or
    /// `T["key"]`, is never `number`, and substituting generics can't change
    /// it.
    pub(crate) fn is_never_supported_index(self) -> bool {
        matches!(self, Self::Literal(_))
    }
}

/// Combines the possible element types selected by `T[number]`.
///
/// For example, indexing this const tuple produces `"A" | "B" | "C"`:
///
/// ```ts
/// const values = ["A", "B", "C"] as const;
/// type Letter = (typeof values)[number];
/// ```
///
/// Optional tuple elements also contribute `undefined`. A declared array type
/// contributes its element type; for example, `("A" | "B")[][number]` produces
/// `"A" | "B"`.
///
/// Returns `None` for unannotated mutable arrays, tuples with rest elements,
/// indices other than `number`, signed bigint literals, strings requiring escape handling, or work
/// exceeding 1024 members or visited types. These results must stay unknown.
pub(crate) fn indexed_access<'db>(
    db: &'db dyn TypeDb,
    mut object: TypeData<'db>,
    index: TypeData<'db>,
) -> Option<TypeData<'db>> {
    if index != TypeData::Number {
        return None;
    }
    if let TypeData::TypeOperator(operator) = object
        && operator.operator(db) == TypeOperator::Readonly
    {
        object = operator.ty(db);
    }
    let projected = match object {
        TypeData::Tuple(tuple) => {
            if tuple.is_inferred_array(db) {
                return None;
            }
            let elements = tuple.elements(db);
            if elements.len() > MAX_PROJECTION_MEMBERS
                || elements.iter().any(|element| element.is_rest)
            {
                return None;
            }
            let mut types = Vec::with_capacity(elements.len() + 1);
            for element in elements {
                types.push(element.ty);
            }
            if elements.iter().any(|element| element.is_optional) {
                types.push(TypeData::Undefined);
            }
            Some(TypeData::union_from_types(db, types))
        }
        TypeData::InstanceOf(instance) if instance.ty(db).is_array_class(db) => {
            instance.type_parameters(db).first().copied()
        }
        _ => None,
    }?;
    if projected
        .union_iterator(db)
        .any(|ty| ty == TypeData::UnknownKeyword)
    {
        return Some(TypeData::UnknownKeyword);
    }

    let mut pending = vec![projected];
    let mut seen = FxHashSet::default();
    // String literals retain their source spelling. Escapes can compare
    // differently from their values, and quotes or line breaks need escaping
    // before a consumer can safely insert them into a double-quoted literal.
    for _ in 0..MAX_PROJECTION_MEMBERS {
        let Some(ty) = pending.pop() else {
            return Some(projected);
        };
        if !seen.insert(ty) {
            continue;
        }
        match ty {
            TypeData::Literal(literal) => match literal.literal(db) {
                Literal::String(string) if requires_escape_handling(string.as_str()) => {
                    return None;
                }
                Literal::BigInt(bigint) if bigint.text().starts_with('-') => return None,
                _ => {}
            },
            TypeData::Union(union) => pending.extend(union.types(db).iter().copied()),
            TypeData::Intersection(intersection) => {
                pending.extend(intersection.types(db).iter().copied())
            }
            TypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
            _ => {}
        }
    }
    pending.is_empty().then_some(projected)
}
