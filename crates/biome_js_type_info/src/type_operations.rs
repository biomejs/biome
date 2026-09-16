//! Evaluates types that select object keys or array elements.
//!
//! The operands must already be resolved and normalized: references point to
//! their types, and wrappers such as `typeof` have been removed. An incomplete
//! result stays unknown so a lint rule cannot mistake a partial list for every
//! possible key or element. This does not depend on which branches execute.

use crate::TypeOperator;
use crate::interned_types::{InternedLiteral, Literal, TypeData, TypeDb, TypeMemberKind};
use rustc_hash::FxHashSet;

const MAX_PROJECTION_MEMBERS: usize = 1024;

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
    let TypeData::Object(object) = ty else {
        return None;
    };
    if object.has_unknown_members(db) || object.prototype(db).is_some() {
        return None;
    }
    let members = object.members(db);
    if members.len() > MAX_PROJECTION_MEMBERS {
        return None;
    }

    let mut keys = Vec::with_capacity(members.len());
    for member in members {
        let name = match &member.kind {
            TypeMemberKind::Named(name)
            | TypeMemberKind::NamedOptional(name)
            | TypeMemberKind::Getter(name)
            | TypeMemberKind::ConstAssertedNamed(name)
            | TypeMemberKind::ConstAssertedNamedOptional(name)
            | TypeMemberKind::ConstAssertedGetter(name) => name,
            TypeMemberKind::CallSignature | TypeMemberKind::ConstAssertedCallSignature => continue,
            _ => return None,
        };
        // Named members do not retain whether a numeric key was quoted. Those
        // spellings have different keyof types, so neither can be inferred here.
        if name.text().contains(['\\', '"', '\n', '\r'])
            || name
                .text()
                .starts_with(|c: char| c.is_ascii_digit() || matches!(c, '.' | '-' | '+'))
        {
            return None;
        }
        keys.push(TypeData::Literal(InternedLiteral::new(
            db,
            Literal::String(name.clone().into()),
        )));
    }
    Some(TypeData::union_from_types(db, keys))
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
                Literal::String(string) if string.as_str().contains(['\\', '"', '\n', '\r']) => {
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
