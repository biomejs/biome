use crate::interned_types::{
    InternedFunction, InternedIntersection, InternedObject, InternedUnion, ReturnType, TypeData,
    TypeDb, TypeMember,
};

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
        if self.types.as_slice() == [TypeData::NeverKeyword] {
            return self;
        }

        match ty {
            TypeData::NeverKeyword => {
                self.types.clear();
                self.types.push(TypeData::NeverKeyword);
            }
            TypeData::Intersection(intersection) => {
                for ty in intersection.types(self.db) {
                    self = self.add(*ty);
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

        self
    }

    pub(crate) fn add_all(mut self, types: impl IntoIterator<Item = TypeData<'db>>) -> Self {
        for ty in types {
            self = self.add(ty);
        }

        self
    }

    pub(crate) fn build(mut self) -> TypeData<'db> {
        if let Some(merged_function) = self.try_build_function() {
            return merged_function;
        }

        if let Some(merged_object) = self.try_build_object() {
            return merged_object;
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

    fn try_build_function(&self) -> Option<TypeData<'db>> {
        if self.types.len() < 2 {
            return None;
        }

        let mut return_types = Vec::new();
        for ty in &self.types {
            let TypeData::Function(function) = ty else {
                return None;
            };
            let ReturnType::Type(return_ty) = function.return_type(self.db) else {
                return Some(TypeData::Function(InternedFunction::new(
                    self.db,
                    Box::default(),
                    Box::default(),
                    ReturnType::Type(TypeData::Boolean),
                    false,
                    None,
                )));
            };
            return_types.push(*return_ty);
        }

        Some(TypeData::Function(InternedFunction::new(
            self.db,
            Box::default(),
            Box::default(),
            ReturnType::Type(TypeData::union_from_types(self.db, return_types)),
            false,
            None,
        )))
    }

    fn try_build_object(&self) -> Option<TypeData<'db>> {
        if self.types.len() < 2 {
            return None;
        }

        let mut members = Vec::new();
        for ty in &self.types {
            let TypeData::Object(object) = ty else {
                return None;
            };
            merge_members(self.db, &mut members, object.members(self.db));
        }

        Some(TypeData::Object(InternedObject::new(
            self.db,
            None,
            members.into_boxed_slice(),
        )))
    }
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
        if self.types.as_slice() == [TypeData::AnyKeyword] {
            return self;
        }

        match ty {
            TypeData::AnyKeyword => {
                self.types.clear();
                self.types.push(TypeData::AnyKeyword);
            }
            TypeData::NeverKeyword => {}
            TypeData::Union(union) => {
                for ty in union.types(self.db) {
                    self = self.add(*ty);
                }
            }
            ty => {
                if !self.types.contains(&ty) {
                    self.types.push(ty);
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
        self.normalize_boolean_variants();

        match self.types.len() {
            0 => TypeData::NeverKeyword,
            1 => self.types.pop().unwrap_or(TypeData::NeverKeyword),
            _ => TypeData::Union(InternedUnion::new(self.db, self.types.into_boxed_slice())),
        }
    }

    fn normalize_boolean_variants(&mut self) {
        let has_boolean = self.types.iter().any(|ty| *ty == TypeData::Boolean);
        let has_true = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, true));
        let has_false = self
            .types
            .iter()
            .any(|ty| ty.is_boolean_literal(self.db, false));

        if !(has_boolean || has_true && has_false) {
            return;
        }

        self.types.retain(|ty| {
            !ty.is_boolean_literal(self.db, true) && !ty.is_boolean_literal(self.db, false)
        });

        if !has_boolean {
            self.types.push(TypeData::Boolean);
        }
    }
}
