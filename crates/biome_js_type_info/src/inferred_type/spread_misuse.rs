use super::{InferredType, MAX_TYPE_VARIANT_STEPS};
use crate::generated::global_types::ids::{
    ITERABLE_ID_GLOBAL_TYPE_ID, ITERABLE_ITERATOR_ID_GLOBAL_TYPE_ID, MAP_ID_GLOBAL_TYPE_ID,
    READONLY_ARRAY_ID_GLOBAL_TYPE_ID, READONLY_MAP_ID_GLOBAL_TYPE_ID,
    READONLY_SET_ID_GLOBAL_TYPE_ID, SET_ID_GLOBAL_TYPE_ID, SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID,
    WEAK_MAP_ID_GLOBAL_TYPE_ID,
};
use crate::interned_types::{Literal, TypeData, TypeMember};
use crate::{GlobalTypeId, TypeDb};
use rustc_hash::FxHashSet;

/// Describes why spreading a value into an object literal is likely a
/// mistake.
///
/// The variants are ordered by priority. When the members of a union match
/// different kinds, [`InferredType::object_spread_misuse`] returns the first
/// one in this order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectSpreadMisuse {
    /// A `Promise`, whose resolved value isn't copied.
    Promise,
    /// A function without properties of its own.
    Function,
    /// A `Map`, `ReadonlyMap`, or `WeakMap`, whose entries aren't properties.
    Map,
    /// An array or tuple, whose elements are copied under their indices.
    Array,
    /// Any other value with a `[Symbol.iterator]()` method, such as a `Set`.
    Iterable,
    /// An instance of a class, which loses the methods of its class.
    ClassInstance,
    /// A class itself, of which only static properties are copied.
    Class,
}

impl<'db> InferredType<'db> {
    /// Returns whether any variant of this type is a string.
    ///
    /// Generics are checked by their constraint, and intersections such as
    /// `string & { brand: "id" }` by their primitive members. Returns `false`
    /// when no variant is known to be a string.
    pub fn has_string_like_variant(self) -> bool {
        self.try_any_variant_matches(|data| match data {
            TypeData::String => true,
            TypeData::Literal(literal) => matches!(
                literal.literal(self.db),
                Literal::String(_) | Literal::Template(_)
            ),
            _ => false,
        })
        .unwrap_or(false)
    }

    /// Classifies whether spreading a value of this type into an object
    /// literal, as in `{ ...value }`, is likely a mistake.
    ///
    /// A union or intersection matches when any of its members matches.
    /// Generics are checked by their constraint. Returns `None` when no
    /// member matches, and when a member is `any` or `unknown`, because these
    /// absorb the other members of a union.
    pub fn object_spread_misuse(self) -> Option<ObjectSpreadMisuse> {
        let db = self.db;
        let mut found: Option<ObjectSpreadMisuse> = None;
        let mut seen = FxHashSet::default();
        // Each entry is a type, and whether the value is an instance of it.
        let mut pending = vec![(self.data, false)];

        for _ in 0..MAX_TYPE_VARIANT_STEPS {
            let Some((data, is_instance)) = pending.pop() else {
                break;
            };
            if !seen.insert((data, is_instance)) {
                continue;
            }

            let kind = if is_instance && let Some(kind) = builtin_instance_misuse(db, data) {
                Some(kind)
            } else {
                match data.expand_canonical_global(db) {
                    TypeData::AnyKeyword | TypeData::UnknownKeyword => return None,
                    TypeData::Union(union) => {
                        pending.extend(union.types(db).iter().map(|ty| (*ty, is_instance)));
                        None
                    }
                    TypeData::Intersection(intersection) => {
                        pending.extend(intersection.types(db).iter().map(|ty| (*ty, is_instance)));
                        None
                    }
                    TypeData::Generic(generic) => {
                        pending.extend(generic.constraint(db).map(|ty| (ty, is_instance)));
                        None
                    }
                    TypeData::InstanceOf(instance) => {
                        pending.push((instance.ty(db), true));
                        None
                    }
                    TypeData::MergedReference(reference) => {
                        let target = if is_instance {
                            reference.ty(db).or_else(|| reference.value_ty(db))
                        } else {
                            reference.value_ty(db).or_else(|| reference.ty(db))
                        };
                        pending.extend(target.map(|ty| (ty, is_instance)));
                        None
                    }
                    TypeData::TypeOperator(operator) => {
                        pending.push((operator.ty(db), is_instance));
                        None
                    }
                    TypeData::TypeofType(typeof_type) => {
                        pending.push((typeof_type.ty(db), is_instance));
                        None
                    }
                    TypeData::TypeofValue(typeof_value) => {
                        pending.push((typeof_value.ty(db), is_instance));
                        None
                    }
                    TypeData::Tuple(_) => Some(ObjectSpreadMisuse::Array),
                    TypeData::Function(_) => Some(ObjectSpreadMisuse::Function),
                    TypeData::Class(_) if !is_instance => Some(ObjectSpreadMisuse::Class),
                    TypeData::Class(class) => {
                        if has_iterator_member(db, class.members(db)) {
                            Some(ObjectSpreadMisuse::Iterable)
                        } else {
                            // A base class may be a built-in type, such as `Promise`.
                            pending.extend(class.extends(db).map(|ty| (ty, true)));
                            Some(ObjectSpreadMisuse::ClassInstance)
                        }
                    }
                    TypeData::Interface(interface) => {
                        let kind = members_misuse(db, interface.members(db));
                        if kind.is_none() {
                            pending
                                .extend(interface.extends(db).iter().map(|ty| (*ty, is_instance)));
                        }
                        kind
                    }
                    TypeData::Object(object) => members_misuse(db, object.members(db)),
                    TypeData::Literal(literal) => match literal.literal(db) {
                        Literal::Object(members) => members_misuse(db, members),
                        _ => None,
                    },
                    _ => None,
                }
            };

            match kind {
                Some(ObjectSpreadMisuse::Promise) => return Some(ObjectSpreadMisuse::Promise),
                Some(kind) => found = Some(found.map_or(kind, |found| found.min(kind))),
                None => {}
            }
        }

        found
    }
}

/// Classifies an instance of a built-in type whose structure doesn't reveal
/// its kind.
fn builtin_instance_misuse<'db>(
    db: &'db dyn TypeDb,
    data: TypeData<'db>,
) -> Option<ObjectSpreadMisuse> {
    if data.is_promise_class(db) {
        Some(ObjectSpreadMisuse::Promise)
    } else if data.is_array_class(db)
        || is_builtin(db, data, READONLY_ARRAY_ID_GLOBAL_TYPE_ID, "ReadonlyArray")
    {
        Some(ObjectSpreadMisuse::Array)
    } else if is_builtin(db, data, MAP_ID_GLOBAL_TYPE_ID, "Map")
        || is_builtin(db, data, READONLY_MAP_ID_GLOBAL_TYPE_ID, "ReadonlyMap")
        || is_builtin(db, data, WEAK_MAP_ID_GLOBAL_TYPE_ID, "WeakMap")
    {
        Some(ObjectSpreadMisuse::Map)
    } else if is_builtin(db, data, SET_ID_GLOBAL_TYPE_ID, "Set")
        || is_builtin(db, data, READONLY_SET_ID_GLOBAL_TYPE_ID, "ReadonlySet")
        || is_builtin(db, data, ITERABLE_ID_GLOBAL_TYPE_ID, "Iterable")
        || is_builtin(
            db,
            data,
            ITERABLE_ITERATOR_ID_GLOBAL_TYPE_ID,
            "IterableIterator",
        )
    {
        Some(ObjectSpreadMisuse::Iterable)
    } else {
        None
    }
}

/// Returns whether `data` is the global type with the given ID, or a copy of
/// the built-in class with the given name, such as an instantiated generic.
fn is_builtin<'db>(db: &'db dyn TypeDb, data: TypeData<'db>, id: GlobalTypeId, name: &str) -> bool {
    data == TypeData::GlobalType(id)
        || matches!(
            data,
            TypeData::Class(class)
                if class.is_builtin(db)
                    && class.name(db).as_ref().is_some_and(|own| own.text() == name)
        )
}

/// Classifies an interface or object type by its members.
fn members_misuse<'db>(
    db: &'db dyn TypeDb,
    members: &[TypeMember<'db>],
) -> Option<ObjectSpreadMisuse> {
    if has_iterator_member(db, members) {
        Some(ObjectSpreadMisuse::Iterable)
    } else if !members.is_empty() && members.iter().all(|member| member.kind.is_call_signature()) {
        Some(ObjectSpreadMisuse::Function)
    } else {
        None
    }
}

/// Returns whether `members` include a non-static `[Symbol.iterator]` member.
fn has_iterator_member<'db>(db: &'db dyn TypeDb, members: &[TypeMember<'db>]) -> bool {
    members.iter().any(|member| {
        !member.kind.is_static()
            && (member.kind.computed_name() == Some("Symbol.iterator")
                || member.kind.computed_value_type().is_some_and(|ty| {
                    ty == TypeData::GlobalType(SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID)
                        || ty.expand_global_local(db)
                            == TypeData::GlobalType(SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID)
                }))
    })
}
