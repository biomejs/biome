use std::hash::Hash;

use crate::interned_types::{
    ClassParents, InternedClass, InternedFunction, InternedInterface, InternedIntersection,
    InternedNamespace, InternedObject, InternedUnion, Literal, TypeData, TypeDb, TypeMember,
    TypeMemberKind,
};
use crate::type_relation::intersect_primitives;
use crate::{ObjectKind, Path};
use biome_rowan::Text;
use rustc_hash::{FxHashMap, FxHashSet};

struct CycleDetector<T, R> {
    active: FxHashSet<T>,
    cache: FxHashMap<T, R>,
    fallback: R,
}

enum CycleEntry<R> {
    Cached(R),
    Reentered(R),
    Entered,
}

impl<T, R> CycleDetector<T, R>
where
    T: Copy + Eq + Hash,
    R: Clone,
{
    fn new(fallback: R) -> Self {
        Self {
            active: FxHashSet::default(),
            cache: FxHashMap::default(),
            fallback,
        }
    }

    fn enter(&mut self, ty: T) -> CycleEntry<R> {
        if let Some(cached) = self.cache.get(&ty) {
            return CycleEntry::Cached(cached.clone());
        }
        if !self.active.insert(ty) {
            return CycleEntry::Reentered(self.fallback.clone());
        }

        CycleEntry::Entered
    }

    fn finish(&mut self, ty: T, result: R) {
        self.active.remove(&ty);
        self.cache.insert(ty, result);
    }
}

#[derive(Clone, Copy)]
enum CompoundKind {
    Intersection,
    Union,
}

#[derive(Clone, Copy)]
enum FlattenItem<'db> {
    Type(TypeData<'db>),
    Finish {
        ty: TypeData<'db>,
        result_start: usize,
    },
}

fn flatten_compound_type<'db>(
    db: &'db dyn TypeDb,
    ty: TypeData<'db>,
    kind: CompoundKind,
) -> Vec<TypeData<'db>> {
    let mut detector: CycleDetector<TypeData<'db>, Box<[TypeData<'db>]>> =
        CycleDetector::new(Box::default());
    let mut stack = vec![FlattenItem::Type(ty)];
    let mut result = Vec::new();

    while let Some(item) = stack.pop() {
        match item {
            FlattenItem::Type(ty) => {
                let nested = match (kind, ty) {
                    (CompoundKind::Intersection, TypeData::Intersection(intersection)) => {
                        Some(intersection.types(db))
                    }
                    (CompoundKind::Union, TypeData::Union(union)) => Some(union.types(db)),
                    _ => None,
                };
                let Some(nested) = nested else {
                    result.push(ty);
                    continue;
                };

                match detector.enter(ty) {
                    CycleEntry::Cached(cached) | CycleEntry::Reentered(cached) => {
                        result.extend(cached.iter().copied());
                    }
                    CycleEntry::Entered => {
                        stack.push(FlattenItem::Finish {
                            ty,
                            result_start: result.len(),
                        });
                        stack.extend(nested.iter().rev().copied().map(FlattenItem::Type));
                    }
                }
            }
            FlattenItem::Finish { ty, result_start } => {
                detector.finish(ty, result[result_start..].to_vec().into_boxed_slice());
            }
        }
    }

    result
}

/// Maximum number of members that distributing an intersection over its union
/// operands may produce. An intersection whose product would be larger keeps
/// its union operands, which is the same type, less simplified.
pub(crate) const MAX_DISTRIBUTED_MEMBERS: usize = 32;

/// Builds intersections normalized by the rules that TypeScript applies, each
/// of which gives a type with the same values:
///
/// - nested intersections are flattened, duplicates removed, and `never`
///   absorbs the other operands;
/// - two primitives become the narrower one, or `never` when they share no
///   value, so `string & "a"` is `"a"` and `string & number` is `never`;
/// - `null` or `undefined` next to an object type is `never`;
/// - `{}` next to another operand that is never `null` or `undefined` is
///   removed, so `string & {}` is `string`, while `T & {}` stays;
/// - an intersection with union operands is distributed into a union of
///   intersections, so `(A | B) & C` is `(A & C) | (B & C)`, as long as the
///   product has at most [`MAX_DISTRIBUTED_MEMBERS`] members;
/// - operands that are all objects, interfaces, class instances, functions,
///   or namespaces merge into one anonymous type. A property that several
///   operands declare has the intersection of their types, and is optional
///   only when every operand declares it optional (see [`merge_members`]).
///
/// The builder doesn't resolve handles. An operand that is a handle, a type
/// parameter, or another type whose values the builder can't tell stays as it
/// is, and the rules that need to know its values don't apply.
pub(crate) struct IntersectionBuilder<'db> {
    db: &'db dyn TypeDb,
    types: Vec<TypeData<'db>>,
    /// Whether [`Self::build`] distributes over union operands. The members of
    /// a distributed intersection are built without distribution, which bounds
    /// the work to one product.
    distributes: bool,
}

impl<'db> IntersectionBuilder<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
            distributes: true,
        }
    }

    fn without_distribution(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
            distributes: false,
        }
    }

    pub(crate) fn add(mut self, ty: TypeData<'db>) -> Self {
        for ty in flatten_compound_type(self.db, ty, CompoundKind::Intersection) {
            if self.types.as_slice() == [TypeData::NeverKeyword] {
                return self;
            }

            match ty {
                TypeData::NeverKeyword => {
                    if self.types.is_empty() {
                        self.types.push(TypeData::NeverKeyword);
                    }
                }
                ty if ty.is_primitive(self.db) => {
                    if !self.add_primitive(ty) {
                        self.types.clear();
                        self.types.push(TypeData::NeverKeyword);
                        return self;
                    }
                }
                ty => {
                    if !self.types.contains(&ty) {
                        self.types.push(ty);
                    }
                }
            }
        }

        self
    }

    /// Adds the primitive `ty`, keeping only the narrower of two primitives
    /// where one includes the other, so `string & "a"` becomes `"a"`.
    ///
    /// Returns `false` when `ty` shares no value with a primitive already
    /// added, which makes the intersection `never`.
    fn add_primitive(&mut self, ty: TypeData<'db>) -> bool {
        // `None` once a primitive already added is at least as narrow.
        let mut ty = Some(ty);
        let mut types = Vec::with_capacity(self.types.len() + 1);
        for other in std::mem::take(&mut self.types) {
            let Some(current) =
                ty.filter(|current| *current != other && other.is_primitive(self.db))
            else {
                if ty == Some(other) {
                    ty = None;
                }
                types.push(other);
                continue;
            };
            // `other` comes first, so it's kept when both have the same value.
            match intersect_primitives(self.db, other, current) {
                Some(TypeData::NeverKeyword) => return false,
                Some(narrower) if narrower == other => {
                    ty = None;
                    types.push(other);
                }
                // `current` is narrower, so it replaces `other`.
                Some(_) => {}
                None => types.push(other),
            }
        }
        types.extend(ty);
        self.types = types;
        true
    }

    pub(crate) fn add_all(mut self, types: impl IntoIterator<Item = TypeData<'db>>) -> Self {
        for ty in types {
            self = self.add(ty);
        }

        self
    }

    pub(crate) fn build(mut self) -> TypeData<'db> {
        if self.distributes
            && let Some(distributed) = self.distribute()
        {
            return distributed;
        }
        if self.has_nullish_and_object_operands() {
            return TypeData::NeverKeyword;
        }
        self.remove_empty_objects();
        if let Some(merged) = MergedType::from_types(self.db, &self.types) {
            return merged.into_type(self.db);
        }

        match self.types.len() {
            0 => TypeData::NeverKeyword,
            1 => self.types.pop().unwrap_or(TypeData::NeverKeyword),
            _ => TypeData::Intersection(InternedIntersection::new(
                self.db,
                self.types.into_boxed_slice(),
            )),
        }
    }

    /// Distributes the intersection over its union operands, building each
    /// member of the product with the other operands.
    ///
    /// Returns `None` when no operand is a union, or when the product would
    /// have more than [`MAX_DISTRIBUTED_MEMBERS`] members.
    fn distribute(&self) -> Option<TypeData<'db>> {
        let db = self.db;
        let mut product = 1usize;
        let operands: Vec<_> = self
            .types
            .iter()
            .map(|ty| match ty {
                TypeData::Union(union) => {
                    let members = union.types(db).as_ref();
                    product = product.saturating_mul(members.len());
                    members
                }
                ty => std::slice::from_ref(ty),
            })
            .collect();
        if product > MAX_DISTRIBUTED_MEMBERS
            || !self.types.iter().any(|ty| matches!(ty, TypeData::Union(_)))
        {
            return None;
        }

        // Counts through the product in the order of the operands, the last
        // operand changing fastest.
        let mut positions = vec![0; operands.len()];
        let mut union = UnionBuilder::new(db);
        loop {
            let intersection = operands.iter().zip(&positions).fold(
                Self::without_distribution(db),
                |builder, (members, position)| builder.add(members[*position]),
            );
            union = union.add(intersection.build());

            let Some(operand) = operands
                .iter()
                .zip(&positions)
                .rposition(|(members, position)| position + 1 < members.len())
            else {
                return Some(union.build());
            };
            positions[operand] += 1;
            positions[operand + 1..].fill(0);
        }
    }

    /// Returns whether an operand is `null` or `undefined` and another one is
    /// an object type, which no value has together with `null` or `undefined`.
    ///
    /// Primitives don't need this check: [`Self::add_primitive`] already gives
    /// `never` for `null & string`.
    fn has_nullish_and_object_operands(&self) -> bool {
        self.types
            .iter()
            .any(|ty| matches!(ty, TypeData::Null | TypeData::Undefined))
            && self
                .types
                .iter()
                .any(|ty| is_structural_object_type(self.db, *ty))
    }

    /// Removes `{}` when another operand is never `null` or `undefined`, since
    /// every such value is also in `{}`. A type parameter can be `null` or
    /// `undefined`, so `T & {}` stays.
    fn remove_empty_objects(&mut self) {
        let db = self.db;
        if self.types.iter().any(|ty| is_empty_object(db, *ty))
            && self
                .types
                .iter()
                .any(|ty| !is_empty_object(db, *ty) && is_non_nullish(db, *ty))
        {
            self.types.retain(|ty| !is_empty_object(db, *ty));
        }
    }
}

/// Returns whether `ty` is `{}`, a type literal without members, which every
/// value other than `null` and `undefined` has. An interface or a class
/// without members isn't, because it is never removed from an intersection.
fn is_empty_object<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    matches!(
        ty,
        TypeData::Object(object)
            if object.prototype(db).is_none()
                && !object.has_unknown_members(db)
                && object.members(db).is_empty()
    )
}

/// Returns whether no value of `ty` is `null` or `undefined`, as far as the
/// type tells without resolving handles: a primitive other than `null` and
/// `undefined`, a literal, or an object type.
fn is_non_nullish<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    match ty {
        TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Literal(_) => true,
        ty => is_structural_object_type(db, ty),
    }
}

pub(crate) fn object_from_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
) -> TypeData<'db> {
    TypeData::Object(InternedObject::new(
        db,
        None,
        members.into_boxed_slice(),
        false,
        ObjectKind::Shape,
    ))
}

pub(crate) fn pick_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
    key_names: &[Text],
) -> TypeData<'db> {
    object_from_members(db, filter_members_by_key_names(members, key_names, true))
}

pub(crate) fn omit_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
    key_names: &[Text],
) -> TypeData<'db> {
    object_from_members(db, filter_members_by_key_names(members, key_names, false))
}

pub(crate) fn with_all_optional_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
) -> TypeData<'db> {
    object_from_members(
        db,
        members
            .into_iter()
            .map(|mut member| {
                let was_optional = member.kind.is_optional();
                member.kind = member.kind.with_optional();
                if !was_optional {
                    member.ty =
                        TypeData::union_from_types(db, Vec::from([member.ty, TypeData::Undefined]));
                }
                member
            })
            .collect(),
    )
}

pub(crate) fn with_all_required_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
) -> TypeData<'db> {
    object_from_members(
        db,
        members
            .into_iter()
            .map(|mut member| {
                let was_optional = member.kind.is_optional();
                member.kind = member.kind.without_optional();
                if was_optional {
                    member.ty = strip_undefined(db, member.ty);
                }
                member
            })
            .collect(),
    )
}

fn filter_members_by_key_names<'db>(
    members: Vec<TypeMember<'db>>,
    key_names: &[Text],
    is_pick: bool,
) -> Vec<TypeMember<'db>> {
    members
        .into_iter()
        .filter(|member| {
            member.kind.name().map_or(!is_pick, |name| {
                let matches_key = key_names.iter().any(|key| key.text() == name.text());
                if is_pick { matches_key } else { !matches_key }
            })
        })
        .collect()
}

fn strip_undefined<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> TypeData<'db> {
    let TypeData::Union(union) = ty else {
        return ty;
    };

    let types = union.types(db);
    let filtered = types
        .iter()
        .copied()
        .filter(|ty| *ty != TypeData::Undefined)
        .collect::<Vec<_>>();

    if filtered.len() == types.len() {
        ty
    } else {
        TypeData::union_from_types(db, filtered)
    }
}

enum MergedType<'db> {
    Any,
    ClassInstance(Vec<TypeMember<'db>>),
    Function(InternedFunction<'db>),
    Interface(Vec<TypeMember<'db>>),
    Namespace(Vec<TypeMember<'db>>),
    Never,
    /// Members of an object, and whether the operand they came from could hold
    /// members that inference did not model. See
    /// [`crate::type_data::Object::has_unknown_members`].
    Object(Vec<TypeMember<'db>>, bool),
    Primitive(TypeData<'db>),
    Unknown,
}

#[derive(Clone, Copy)]
enum MergedTypeKind {
    Any,
    ClassInstance,
    Function,
    Interface,
    Namespace,
    Never,
    Object,
    Primitive,
    Unknown,
}

impl MergedTypeKind {
    fn intersection_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            (Self::ClassInstance, Self::ClassInstance) => Self::ClassInstance,
            (
                Self::ClassInstance,
                Self::Function | Self::Interface | Self::Object | Self::Namespace,
            )
            | (
                Self::Function | Self::Interface | Self::Object | Self::Namespace,
                Self::ClassInstance,
            ) => Self::ClassInstance,
            (Self::Function, Self::Function) => Self::Function,
            (Self::Interface | Self::Function, Self::Interface)
            | (Self::Interface, Self::Function) => Self::Interface,
            (
                Self::Namespace | Self::Function | Self::Object | Self::Interface,
                Self::Namespace,
            )
            | (Self::Namespace, Self::Function | Self::Object | Self::Interface) => Self::Namespace,
            (Self::Never, _) | (_, Self::Never) => Self::Never,
            (Self::Object | Self::Function | Self::Interface, Self::Object)
            | (Self::Object, Self::Function | Self::Interface) => Self::Object,
            (Self::Primitive, Self::Primitive) => Self::Never,
            (Self::Primitive, _) | (_, Self::Primitive) => Self::Primitive,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
        }
    }
}

impl<'db> MergedType<'db> {
    fn from_types(db: &'db dyn TypeDb, types: &[TypeData<'db>]) -> Option<Self> {
        if types.len() < 2 {
            return None;
        }
        if types.iter().any(|ty| ty.is_primitive(db))
            && types.iter().any(|ty| is_structural_object_type(db, *ty))
        {
            return None;
        }
        // `IntersectionBuilder::add` only keeps several primitives together
        // when their intersection can't be decided.
        if types.iter().filter(|ty| ty.is_primitive(db)).count() > 1 {
            return None;
        }

        let mut types = types.iter().copied();
        let mut merged = Self::from_type(db, types.next()?)?;
        for ty in types {
            merged = merged.intersection_with(db, Self::from_type(db, ty)?);
        }

        Some(merged)
    }

    fn from_type(db: &'db dyn TypeDb, ty: TypeData<'db>) -> Option<Self> {
        match ty {
            TypeData::AnyKeyword | TypeData::Conditional => Some(Self::Any),
            TypeData::BigInt
            | TypeData::Boolean
            | TypeData::Null
            | TypeData::Number
            | TypeData::String
            | TypeData::Symbol
            | TypeData::Undefined => Some(Self::Primitive(ty)),
            TypeData::Class(class) => {
                Some(Self::Object(class_static_members(class.members(db)), true))
            }
            TypeData::Function(function) => Some(Self::Function(function)),
            TypeData::InstanceOf(instance) => match instance.ty(db) {
                TypeData::Class(class) => Some(Self::ClassInstance(class_instance_members(
                    class.members(db),
                ))),
                _ => None,
            },
            TypeData::Interface(interface) => Some(Self::Interface(interface.members(db).to_vec())),
            TypeData::Literal(literal) => match literal.literal(db) {
                Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_)
                | Literal::Template(_) => Some(Self::Primitive(ty)),
                Literal::Object(members) => Some(Self::Object(members.to_vec(), false)),
                Literal::RegExp(_) => Some(Self::Unknown),
            },
            TypeData::Namespace(namespace) => Some(Self::Namespace(namespace.members(db).to_vec())),
            TypeData::NeverKeyword => Some(Self::Never),
            TypeData::Object(object) => Some(Self::Object(
                object.members(db).to_vec(),
                object.has_unknown_members(db) || object.prototype(db).is_some(),
            )),
            TypeData::Unknown | TypeData::UnknownKeyword => Some(Self::Unknown),
            _ => None,
        }
    }

    fn intersection_with(self, db: &'db dyn TypeDb, other: Self) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            (Self::Function(left), Self::Function(right)) => {
                Self::Function(left.intersection_with(db, right))
            }
            (Self::Never, _) | (_, Self::Never) => Self::Never,
            (Self::Primitive(_), Self::Primitive(_)) => Self::Never,
            (Self::Primitive(primitive), _) | (_, Self::Primitive(primitive)) => {
                Self::Primitive(primitive)
            }
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (left, right) => {
                let (left_kind, left_members, left_unknown) = left.into_kind_with_members();
                let (right_kind, right_members, right_unknown) = right.into_kind_with_members();
                let mut members = left_members;
                merge_members(db, &mut members, &right_members);
                Self::from_kind_and_members(
                    left_kind.intersection_with(right_kind),
                    members,
                    left_unknown || right_unknown,
                )
            }
        }
    }

    /// Splits the merged type into the parts that intersection combines.
    ///
    /// The trailing flag reports whether the operand could hold members that
    /// inference did not model. Class, interface, and namespace member lists
    /// may omit inherited or uncollected keys, so they also set this flag.
    fn into_kind_with_members(self) -> (MergedTypeKind, Vec<TypeMember<'db>>, bool) {
        match self {
            Self::Any => (MergedTypeKind::Any, Vec::new(), false),
            Self::ClassInstance(members) => (MergedTypeKind::ClassInstance, members, true),
            Self::Function(_) => (MergedTypeKind::Function, Vec::new(), false),
            Self::Interface(members) => (MergedTypeKind::Interface, members, true),
            Self::Namespace(members) => (MergedTypeKind::Namespace, members, true),
            Self::Never => (MergedTypeKind::Never, Vec::new(), false),
            Self::Object(members, has_unknown_members) => {
                (MergedTypeKind::Object, members, has_unknown_members)
            }
            Self::Primitive(_) => (MergedTypeKind::Primitive, Vec::new(), false),
            Self::Unknown => (MergedTypeKind::Unknown, Vec::new(), false),
        }
    }

    fn from_kind_and_members(
        kind: MergedTypeKind,
        members: Vec<TypeMember<'db>>,
        has_unknown_members: bool,
    ) -> Self {
        match kind {
            MergedTypeKind::Any => Self::Any,
            MergedTypeKind::ClassInstance => Self::ClassInstance(members),
            MergedTypeKind::Interface => Self::Interface(members),
            MergedTypeKind::Namespace => Self::Namespace(members),
            MergedTypeKind::Never => Self::Never,
            MergedTypeKind::Object => Self::Object(members, has_unknown_members),
            MergedTypeKind::Function | MergedTypeKind::Primitive | MergedTypeKind::Unknown => {
                Self::Unknown
            }
        }
    }

    fn into_type(self, db: &'db dyn TypeDb) -> TypeData<'db> {
        match self {
            Self::Any => TypeData::AnyKeyword,
            Self::ClassInstance(members) => TypeData::instance_of(
                db,
                TypeData::Class(InternedClass::new(
                    db,
                    Box::default(),
                    ClassParents::default(),
                    Box::default(),
                    members.into_boxed_slice(),
                    None,
                    false,
                    None,
                    0,
                )),
                Box::default(),
            ),
            Self::Function(function) => TypeData::Function(function),
            Self::Interface(members) => TypeData::Interface(InternedInterface::new(
                db,
                Box::default(),
                Box::default(),
                members.into_boxed_slice(),
                Text::new_static("(merged)"),
                None,
                0,
            )),
            Self::Namespace(members) => TypeData::Namespace(InternedNamespace::new(
                db,
                members.into_boxed_slice(),
                Path::from(Text::new_static("")),
            )),
            Self::Never => TypeData::NeverKeyword,
            Self::Object(members, has_unknown_members) => TypeData::Object(InternedObject::new(
                db,
                None,
                members.into_boxed_slice(),
                has_unknown_members,
                ObjectKind::Shape,
            )),
            Self::Primitive(primitive) => primitive,
            Self::Unknown => TypeData::Unknown,
        }
    }
}

fn is_structural_object_type<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    match ty {
        TypeData::Class(_)
        | TypeData::Constructor(_)
        | TypeData::Function(_)
        | TypeData::Interface(_)
        | TypeData::Module(_)
        | TypeData::Namespace(_)
        | TypeData::Object(_)
        | TypeData::Tuple(_)
        | TypeData::ObjectKeyword => true,
        TypeData::InstanceOf(instance) => is_structural_object_type(db, instance.ty(db)),
        TypeData::Literal(literal) => matches!(literal.literal(db), Literal::Object(_)),
        _ => false,
    }
}

fn class_instance_members<'db>(members: &[TypeMember<'db>]) -> Vec<TypeMember<'db>> {
    members
        .iter()
        .filter(|member| !member.kind.is_static())
        .cloned()
        .collect()
}

fn class_static_members<'db>(members: &[TypeMember<'db>]) -> Vec<TypeMember<'db>> {
    members
        .iter()
        .filter_map(|member| match &member.kind {
            TypeMemberKind::NamedStatic(name) => Some(TypeMember {
                kind: TypeMemberKind::Named(name.clone()),
                ty: member.ty,
            }),
            TypeMemberKind::ConstAssertedNamedStatic(name) => Some(TypeMember {
                kind: TypeMemberKind::ConstAssertedNamed(name.clone()),
                ty: member.ty,
            }),
            _ => None,
        })
        .collect()
}

/// Adds the instance `members` of an intersection operand to the members
/// `merged` from the operands before it.
///
/// A member that both declare is merged as TypeScript merges a property of an
/// intersection: it is optional only when both declare it optional, and
/// readonly only when both declare it readonly. A property's type is the
/// intersection of both types, where an optional property's type includes
/// `undefined`, so `{ a?: string } & { a: string | null }` has `a: string`.
/// A method, or a property whose type is a function, gets the union of both
/// types instead. TypeScript intersects function types into overloads, which
/// Biome doesn't model, and the union is wider.
fn merge_members<'db>(
    db: &'db dyn TypeDb,
    merged: &mut Vec<TypeMember<'db>>,
    members: &[TypeMember<'db>],
) {
    for member in members.iter().filter(|member| !member.kind.is_static()) {
        let existing = member.name().and_then(|name| {
            merged
                .iter_mut()
                .find(|merged_member| merged_member.kind.has_name(name.text()))
        });

        match existing {
            Some(existing) => {
                let is_optional = existing.kind.is_optional() && member.kind.is_optional();
                if existing.ty != member.ty {
                    existing.ty = if is_property(db, existing) && is_property(db, member) {
                        let property_type = |member: &TypeMember<'db>| {
                            if member.kind.is_optional() && !is_optional {
                                TypeData::union_from_types(
                                    db,
                                    Vec::from([member.ty, TypeData::Undefined]),
                                )
                            } else {
                                member.ty
                            }
                        };
                        TypeData::intersection_from_types(
                            db,
                            Vec::from([property_type(existing), property_type(member)]),
                        )
                    } else {
                        TypeData::union_from_types(db, Vec::from([existing.ty, member.ty]))
                    };
                }
                if !is_optional {
                    existing.kind = existing.kind.clone().without_optional();
                }
                if existing.kind.is_readonly() && !member.kind.is_readonly() {
                    existing.kind = existing.kind.clone().without_readonly();
                }
            }
            None => merged.push(member.clone()),
        }
    }
}

/// Returns whether `member` is a property whose type intersects with the type
/// of the same property in another operand: a named property that isn't a
/// getter, whose type isn't a function.
fn is_property<'db>(db: &'db dyn TypeDb, member: &TypeMember<'db>) -> bool {
    matches!(
        member.kind,
        TypeMemberKind::Named(_)
            | TypeMemberKind::NamedOptional(_)
            | TypeMemberKind::ReadonlyNamed(_)
            | TypeMemberKind::ReadonlyNamedOptional(_)
            | TypeMemberKind::ConstAssertedNamed(_)
            | TypeMemberKind::ConstAssertedNamedOptional(_)
    ) && !is_function_type(db, member.ty)
}

/// Returns whether `ty` is a function or constructor type, an object with only
/// call signatures, such as the overloads of a method, or a union with such a
/// member.
fn is_function_type<'db>(db: &'db dyn TypeDb, ty: TypeData<'db>) -> bool {
    match ty {
        TypeData::Function(_) | TypeData::Constructor(_) => true,
        TypeData::Object(object) => {
            let members = object.members(db);
            !members.is_empty() && members.iter().all(|member| member.kind.is_call_signature())
        }
        TypeData::Union(union) => union.types(db).iter().any(|ty| is_function_type(db, *ty)),
        _ => false,
    }
}

pub(crate) struct UnionBuilder<'db> {
    db: &'db dyn TypeDb,
    types: Vec<TypeData<'db>>,
}

impl<'db> UnionBuilder<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
        }
    }

    pub(crate) fn add(mut self, ty: TypeData<'db>) -> Self {
        for ty in flatten_compound_type(self.db, ty, CompoundKind::Union) {
            if self.types.as_slice() == [TypeData::AnyKeyword] {
                return self;
            }

            match ty {
                TypeData::AnyKeyword => {
                    self.types.clear();
                    self.types.push(TypeData::AnyKeyword);
                }
                _ if self.types.as_slice() == [TypeData::Unknown] => {}
                TypeData::Unknown => {
                    self.types.clear();
                    self.types.push(TypeData::Unknown);
                }
                TypeData::NeverKeyword => {}
                ty => {
                    if !self.types.contains(&ty) {
                        self.types.push(ty);
                    }
                }
            }
        }

        self
    }

    pub(crate) fn add_all(mut self, types: impl IntoIterator<Item = TypeData<'db>>) -> Self {
        for ty in types {
            self = self.add(ty);
        }

        self
    }

    pub(crate) fn build(mut self) -> TypeData<'db> {
        self.normalize_literal_variants();

        match self.types.len() {
            0 => TypeData::NeverKeyword,
            1 => self.types.pop().unwrap_or(TypeData::NeverKeyword),
            _ => TypeData::Union(InternedUnion::new(self.db, self.types.into_boxed_slice())),
        }
    }

    fn normalize_literal_variants(&mut self) {
        let has_bigint = self.types.contains(&TypeData::BigInt);
        let has_boolean = self.types.contains(&TypeData::Boolean);
        let has_number = self.types.contains(&TypeData::Number);
        let has_string = self.types.contains(&TypeData::String);
        let has_true = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, true));
        let has_false = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, false));
        let should_add_boolean = !has_boolean && has_true && has_false;
        let should_absorb_boolean_literals = has_boolean || should_add_boolean;

        if !(has_bigint || should_absorb_boolean_literals || has_number || has_string) {
            return;
        }

        self.types.retain(|ty| match ty.literal_base_type(self.db) {
            Some(TypeData::BigInt) => !has_bigint,
            Some(TypeData::Boolean) => !should_absorb_boolean_literals,
            Some(TypeData::Number) => !has_number,
            Some(TypeData::String) => !has_string,
            _ => true,
        });

        if should_add_boolean {
            self.types.push(TypeData::Boolean);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CycleDetector, CycleEntry, MAX_DISTRIBUTED_MEMBERS};
    use crate::ObjectKind;
    use crate::interned_types::{
        InternedFunction, InternedGenericTypeParameter, InternedLiteral, InternedObject, Literal,
        ReturnType, TypeData, TypeDb, TypeMember, TypeMemberKind,
    };
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
        fn parsed_source_for_path(&self, _path: &camino::Utf8Path) -> Option<biome_db::Source> {
            None
        }
    }

    #[salsa::db]
    impl TypeDb for TestDb {}

    fn intersection<'db>(db: &'db TestDb, types: &[TypeData<'db>]) -> TypeData<'db> {
        TypeData::intersection_from_types(db, types.to_vec())
    }

    fn union<'db>(db: &'db TestDb, types: &[TypeData<'db>]) -> TypeData<'db> {
        TypeData::union_from_types(db, types.to_vec())
    }

    fn object<'db>(db: &'db TestDb, members: Vec<TypeMember<'db>>) -> TypeData<'db> {
        TypeData::Object(InternedObject::new(
            db,
            None,
            members.into_boxed_slice(),
            false,
            ObjectKind::Shape,
        ))
    }

    fn member<'db>(kind: fn(Text) -> TypeMemberKind<'db>, ty: TypeData<'db>) -> TypeMember<'db> {
        TypeMember {
            kind: kind(Text::new_static("a")),
            ty,
        }
    }

    /// Returns `{ a: ty }`, or `{ a?: ty }`.
    fn with_a<'db>(db: &'db TestDb, ty: TypeData<'db>, is_optional: bool) -> TypeData<'db> {
        let kind = if is_optional {
            TypeMemberKind::NamedOptional
        } else {
            TypeMemberKind::Named
        };
        object(db, vec![member(kind, ty)])
    }

    /// Returns the member `a` of the object `ty`.
    fn member_a<'db>(db: &'db TestDb, ty: TypeData<'db>) -> TypeMember<'db> {
        let TypeData::Object(object) = ty else {
            panic!("expected an object, got {ty:?}");
        };
        object
            .members(db)
            .iter()
            .find(|member| member.kind.has_name("a"))
            .cloned()
            .expect("the object must have `a`")
    }

    fn number<'db>(db: &'db TestDb, value: usize) -> TypeData<'db> {
        TypeData::Literal(InternedLiteral::new(
            db,
            Literal::Number(crate::literal::NumberLiteral::new(value.to_string().into())),
        ))
    }

    fn generic(db: &TestDb) -> TypeData<'_> {
        TypeData::Generic(InternedGenericTypeParameter::new(
            db,
            false,
            None,
            None,
            Text::new_static("T"),
        ))
    }

    fn function<'db>(db: &'db TestDb, return_type: TypeData<'db>) -> TypeData<'db> {
        TypeData::Function(InternedFunction::new(
            db,
            Box::default(),
            Box::default(),
            ReturnType::Type(return_type),
            false,
            None,
        ))
    }

    #[test]
    fn intersections_distribute_over_union_operands() {
        let db = TestDb::default();
        let empty = object(&db, Vec::new());
        // `(string | null) & {}` is `string`.
        let nullable = union(&db, &[TypeData::String, TypeData::Null]);
        assert_eq!(intersection(&db, &[nullable, empty]), TypeData::String);
        // `(1 | 2) & (2 | 3)` is `2`.
        let left = union(&db, &[number(&db, 1), number(&db, 2)]);
        let right = union(&db, &[number(&db, 2), number(&db, 3)]);
        assert_eq!(intersection(&db, &[left, right]), number(&db, 2));
        // `(string | number) & { a: 1 }` is
        // `(string & { a: 1 }) | (number & { a: 1 })`.
        let primitives = union(&db, &[TypeData::String, TypeData::Number]);
        let a = with_a(&db, number(&db, 1), false);
        assert_eq!(
            intersection(&db, &[primitives, a]),
            union(
                &db,
                &[
                    intersection(&db, &[TypeData::String, a]),
                    intersection(&db, &[TypeData::Number, a]),
                ]
            )
        );
    }

    #[test]
    fn distribution_keeps_products_over_the_bound() {
        let db = TestDb::default();
        let numbers = |range: std::ops::Range<usize>| {
            union(
                &db,
                &range.map(|value| number(&db, value)).collect::<Vec<_>>(),
            )
        };
        let object = with_a(&db, TypeData::String, false);
        // A product with exactly `MAX_DISTRIBUTED_MEMBERS` members distributes.
        let distributed = intersection(&db, &[numbers(0..4), numbers(0..8), object]);
        let TypeData::Union(union) = distributed else {
            panic!("expected a union, got {distributed:?}");
        };
        assert_eq!(union.types(&db).len(), 4);
        assert_eq!(4 * 8, MAX_DISTRIBUTED_MEMBERS);
        // One more member keeps the union operands.
        let kept = intersection(&db, &[numbers(0..4), numbers(0..9), object]);
        let TypeData::Intersection(operands) = kept else {
            panic!("expected an intersection, got {kept:?}");
        };
        assert_eq!(
            operands.types(&db).as_ref(),
            [numbers(0..4), numbers(0..9), object]
        );
    }

    #[test]
    fn nullish_operands_next_to_object_types_give_never() {
        let db = TestDb::default();
        let empty = object(&db, Vec::new());
        let a = with_a(&db, TypeData::String, false);
        for (nullish, other) in [
            (TypeData::Null, empty),
            (TypeData::Undefined, a),
            (TypeData::Null, TypeData::ObjectKeyword),
            (TypeData::Undefined, TypeData::String),
        ] {
            assert_eq!(intersection(&db, &[nullish, other]), TypeData::NeverKeyword);
            assert_eq!(intersection(&db, &[other, nullish]), TypeData::NeverKeyword);
        }
        // A type parameter can be `null`, so `T & null` stays.
        let t = generic(&db);
        assert!(matches!(
            intersection(&db, &[t, TypeData::Null]),
            TypeData::Intersection(_)
        ));
    }

    #[test]
    fn empty_objects_are_removed_next_to_operands_that_are_never_nullish() {
        let db = TestDb::default();
        let empty = object(&db, Vec::new());
        let a = with_a(&db, TypeData::String, false);
        assert_eq!(
            intersection(&db, &[TypeData::String, empty]),
            TypeData::String
        );
        assert_eq!(intersection(&db, &[empty, a]), a);
        // `T & {}` excludes `null` and `undefined` from `T`, so it stays.
        let t = generic(&db);
        let non_nullable = intersection(&db, &[t, empty]);
        let TypeData::Intersection(operands) = non_nullable else {
            panic!("expected an intersection, got {non_nullable:?}");
        };
        assert_eq!(operands.types(&db).as_ref(), [t, empty]);
        // `T & string & {}` is `T & string`.
        assert_eq!(
            intersection(&db, &[t, TypeData::String, empty]),
            intersection(&db, &[t, TypeData::String])
        );
    }

    #[test]
    fn properties_declared_by_two_operands_intersect() {
        let db = TestDb::default();
        let nullable = union(&db, &[TypeData::String, TypeData::Null]);
        // `{ a?: string } & { a: string | null }` has `a: string`.
        let merged = intersection(
            &db,
            &[
                with_a(&db, TypeData::String, true),
                with_a(&db, nullable, false),
            ],
        );
        let a = member_a(&db, merged);
        assert!(matches!(a.kind, TypeMemberKind::Named(_)));
        assert_eq!(a.ty, TypeData::String);
        // `{ a?: string } & { a?: string | null }` has `a?: string`, and its
        // read adds `undefined`.
        let merged = intersection(
            &db,
            &[
                with_a(&db, TypeData::String, true),
                with_a(&db, nullable, true),
            ],
        );
        let a = member_a(&db, merged);
        assert!(matches!(a.kind, TypeMemberKind::NamedOptional(_)));
        assert_eq!(a.ty, TypeData::String);
        // `{ a: string } & { a: number }` has `a: never`.
        let merged = intersection(
            &db,
            &[
                with_a(&db, TypeData::String, false),
                with_a(&db, TypeData::Number, false),
            ],
        );
        assert_eq!(member_a(&db, merged).ty, TypeData::NeverKeyword);
        // A readonly property stays readonly only when both declare it so.
        let merged = intersection(
            &db,
            &[
                object(
                    &db,
                    vec![member(TypeMemberKind::ReadonlyNamed, TypeData::String)],
                ),
                with_a(&db, nullable, false),
            ],
        );
        let a = member_a(&db, merged);
        assert!(matches!(a.kind, TypeMemberKind::Named(_)));
        assert_eq!(a.ty, TypeData::String);
    }

    #[test]
    fn methods_keep_the_union_of_their_types() {
        let db = TestDb::default();
        let left = function(&db, TypeData::String);
        let right = function(&db, TypeData::Number);
        let merged = intersection(&db, &[with_a(&db, left, false), with_a(&db, right, false)]);
        assert_eq!(member_a(&db, merged).ty, union(&db, &[left, right]));
        // So does a property whose type is a function in one operand only.
        let merged = intersection(
            &db,
            &[
                with_a(&db, left, false),
                with_a(&db, TypeData::String, false),
            ],
        );
        assert_eq!(
            member_a(&db, merged).ty,
            union(&db, &[left, TypeData::String])
        );
    }

    #[test]
    fn cycle_detector_reports_reentry_before_finish() {
        let mut detector = CycleDetector::new("fallback");

        assert!(matches!(detector.enter(1), CycleEntry::Entered));
        assert!(matches!(
            detector.enter(1),
            CycleEntry::Reentered("fallback")
        ));
    }

    #[test]
    fn cycle_detector_returns_cached_result_after_finish() {
        let mut detector = CycleDetector::new("fallback");

        assert!(matches!(detector.enter(1), CycleEntry::Entered));
        detector.finish(1, "cached");
        assert!(matches!(detector.enter(1), CycleEntry::Cached("cached")));
    }
}
