use std::hash::Hash;

use crate::Path;
use crate::interned_types::{
    InternedClass, InternedFunction, InternedInterface, InternedIntersection, InternedNamespace,
    InternedObject, InternedUnion, Literal, TypeData, TypeDb, TypeMember, TypeMemberKind,
};
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

pub(crate) struct IntersectionBuilder<'db> {
    db: &'db dyn TypeDb,
    types: Vec<TypeData<'db>>,
}

impl<'db> IntersectionBuilder<'db> {
    pub(crate) fn new(db: &'db dyn TypeDb) -> Self {
        Self {
            db,
            types: Vec::new(),
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
                ty => {
                    if ty.is_primitive(self.db)
                        && self
                            .types
                            .iter()
                            .any(|other| *other != ty && other.is_primitive(self.db))
                    {
                        self.types.clear();
                        self.types.push(TypeData::NeverKeyword);
                        return self;
                    }

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
}

pub(crate) fn object_from_members<'db>(
    db: &'db dyn TypeDb,
    members: Vec<TypeMember<'db>>,
) -> TypeData<'db> {
    TypeData::Object(InternedObject::new(db, None, members.into_boxed_slice()))
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
    Object(Vec<TypeMember<'db>>),
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
            TypeData::Class(class) => Some(Self::Object(class_static_members(class.members(db)))),
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
                Literal::Object(members) => Some(Self::Object(members.to_vec())),
                Literal::RegExp(_) => Some(Self::Unknown),
            },
            TypeData::Namespace(namespace) => Some(Self::Namespace(namespace.members(db).to_vec())),
            TypeData::NeverKeyword => Some(Self::Never),
            TypeData::Object(object) => Some(Self::Object(object.members(db).to_vec())),
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
                let (left_kind, left_members) = left.into_kind_with_members();
                let (right_kind, right_members) = right.into_kind_with_members();
                let mut members = left_members;
                merge_members(db, &mut members, &right_members);
                Self::from_kind_and_members(left_kind.intersection_with(right_kind), members)
            }
        }
    }

    fn into_kind_with_members(self) -> (MergedTypeKind, Vec<TypeMember<'db>>) {
        match self {
            Self::Any => (MergedTypeKind::Any, Vec::new()),
            Self::ClassInstance(members) => (MergedTypeKind::ClassInstance, members),
            Self::Function(_) => (MergedTypeKind::Function, Vec::new()),
            Self::Interface(members) => (MergedTypeKind::Interface, members),
            Self::Namespace(members) => (MergedTypeKind::Namespace, members),
            Self::Never => (MergedTypeKind::Never, Vec::new()),
            Self::Object(members) => (MergedTypeKind::Object, members),
            Self::Primitive(_) => (MergedTypeKind::Primitive, Vec::new()),
            Self::Unknown => (MergedTypeKind::Unknown, Vec::new()),
        }
    }

    fn from_kind_and_members(kind: MergedTypeKind, members: Vec<TypeMember<'db>>) -> Self {
        match kind {
            MergedTypeKind::Any => Self::Any,
            MergedTypeKind::ClassInstance => Self::ClassInstance(members),
            MergedTypeKind::Interface => Self::Interface(members),
            MergedTypeKind::Namespace => Self::Namespace(members),
            MergedTypeKind::Never => Self::Never,
            MergedTypeKind::Object => Self::Object(members),
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
                    None,
                    Box::default(),
                    members.into_boxed_slice(),
                    None,
                    false,
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
            )),
            Self::Namespace(members) => TypeData::Namespace(InternedNamespace::new(
                db,
                members.into_boxed_slice(),
                Path::from(Text::new_static("")),
            )),
            Self::Never => TypeData::NeverKeyword,
            Self::Object(members) => {
                TypeData::Object(InternedObject::new(db, None, members.into_boxed_slice()))
            }
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
            Some(existing) if existing.ty != member.ty => {
                existing.ty = TypeData::union_from_types(db, Vec::from([existing.ty, member.ty]));
            }
            Some(_) => {}
            None => merged.push(member.clone()),
        }
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
    use super::{CycleDetector, CycleEntry};

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
